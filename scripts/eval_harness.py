#!/usr/bin/env python3
"""
Automated Evaluation Harness — Benchmark AI after every training run.

Runs a fixed test set across domains, measures accuracy, and compares
against previous baselines. Reports regressions immediately.

Usage:
    python3 eval_harness.py --run          # Run full evaluation
    python3 eval_harness.py --compare      # Compare with last baseline
    python3 eval_harness.py --domains math,cyber  # Specific domains only
"""

import argparse
import json
import os
import sqlite3
import time
import requests

BRAIN_DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
MODEL = os.environ.get("PLAUSIDEN_MODEL", "qwen2.5-coder:7b")
EVAL_DIR = os.path.expanduser("~/LFI-data/eval_results")
BASELINE_FILE = os.path.join(EVAL_DIR, "baseline.json")

# Fixed test set — never changes, so we can track improvement
TEST_SET = [
    {"q": "What is the time complexity of binary search?", "a": "O(log n)", "domain": "programming", "type": "exact"},
    {"q": "What port does HTTPS use?", "a": "443", "domain": "cybersecurity", "type": "exact"},
    {"q": "What is the derivative of x^2?", "a": "2x", "domain": "mathematics", "type": "contains"},
    {"q": "What is TCP?", "a": "transport", "domain": "cybersecurity", "type": "contains"},
    {"q": "What causes tides?", "a": "moon", "domain": "physics", "type": "contains"},
    {"q": "What is photosynthesis?", "a": "light", "domain": "biology", "type": "contains"},
    {"q": "Who wrote Romeo and Juliet?", "a": "Shakespeare", "domain": "literature", "type": "contains"},
    {"q": "What is GDP?", "a": "gross domestic product", "domain": "economics", "type": "contains"},
    {"q": "What is the speed of light?", "a": "300", "domain": "physics", "type": "contains"},
    {"q": "What is an API?", "a": "application programming interface", "domain": "programming", "type": "contains"},
    {"q": "What is SQL injection?", "a": "inject", "domain": "cybersecurity", "type": "contains"},
    {"q": "What is the Pythagorean theorem?", "a": "a² + b² = c²", "domain": "mathematics", "type": "contains"},
    {"q": "What is machine learning?", "a": "learn", "domain": "technology", "type": "contains"},
    {"q": "What is DNS?", "a": "domain name", "domain": "cybersecurity", "type": "contains"},
    {"q": "What is an eigenvalue?", "a": "vector", "domain": "mathematics", "type": "contains"},
    {"q": "What is natural selection?", "a": "survival", "domain": "biology", "type": "contains"},
    {"q": "What is inflation in economics?", "a": "price", "domain": "economics", "type": "contains"},
    {"q": "What is a mutex?", "a": "mutual exclusion", "domain": "programming", "type": "contains"},
    {"q": "What is AES encryption?", "a": "symmetric", "domain": "cybersecurity", "type": "contains"},
    {"q": "What is the quadratic formula?", "a": "b²", "domain": "mathematics", "type": "contains"},
]


def query_model(question: str) -> str:
    try:
        r = requests.post("http://localhost:11434/api/generate", json={
            "model": MODEL, "stream": False, "prompt": question,
            "options": {"temperature": 0.3, "num_predict": 150}
        }, timeout=30)
        return r.json().get("response", "").strip()
    except Exception:
        return ""


def check_answer(response: str, expected: str, check_type: str) -> bool:
    lower = response.lower()
    exp_lower = expected.lower()
    if check_type == "exact":
        return exp_lower in lower
    elif check_type == "contains":
        return exp_lower in lower
    return False


def run_eval(domains: list = None) -> dict:
    """Run the full evaluation suite."""
    os.makedirs(EVAL_DIR, exist_ok=True)
    results = {"timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"), "tests": [], "by_domain": {}}

    correct = 0
    total = 0
    domain_stats = {}

    for test in TEST_SET:
        if domains and test["domain"] not in domains:
            continue

        response = query_model(test["q"])
        passed = check_answer(response, test["a"], test["type"])

        results["tests"].append({
            "question": test["q"],
            "expected": test["a"],
            "response": response[:200],
            "passed": passed,
            "domain": test["domain"],
        })

        total += 1
        if passed:
            correct += 1

        d = test["domain"]
        if d not in domain_stats:
            domain_stats[d] = {"total": 0, "correct": 0}
        domain_stats[d]["total"] += 1
        if passed:
            domain_stats[d]["correct"] += 1

        status = "✓" if passed else "✗"
        print(f"  {status} [{test['domain']}] {test['q'][:50]}...")

    results["total"] = total
    results["correct"] = correct
    results["accuracy"] = correct / max(total, 1)
    results["by_domain"] = {
        d: {"accuracy": s["correct"] / max(s["total"], 1), **s}
        for d, s in domain_stats.items()
    }

    # Save results
    ts = time.strftime("%Y%m%d_%H%M%S")
    result_path = os.path.join(EVAL_DIR, f"eval_{ts}.json")
    with open(result_path, "w") as f:
        json.dump(results, f, indent=2)

    # Log to brain.db
    conn = sqlite3.connect(BRAIN_DB, timeout=30)
    conn.execute("PRAGMA busy_timeout=30000")
    for d, s in domain_stats.items():
        conn.execute(
            "INSERT INTO training_results (domain, accuracy, total, correct) VALUES (?,?,?,?)",
            (d, s["correct"] / max(s["total"], 1), s["total"], s["correct"])
        )
    conn.commit()
    conn.close()

    print(f"\n=== RESULTS: {correct}/{total} ({results['accuracy']*100:.0f}%) ===")
    for d, s in sorted(results["by_domain"].items()):
        print(f"  {d}: {s['correct']}/{s['total']} ({s['accuracy']*100:.0f}%)")

    return results


def compare_with_baseline():
    """Compare current results with saved baseline."""
    if not os.path.exists(BASELINE_FILE):
        print("No baseline found. Run --run first to create one.")
        return

    with open(BASELINE_FILE) as f:
        baseline = json.load(f)

    # Find most recent eval
    evals = sorted([f for f in os.listdir(EVAL_DIR) if f.startswith("eval_")])
    if not evals:
        print("No evaluation results found. Run --run first.")
        return

    with open(os.path.join(EVAL_DIR, evals[-1])) as f:
        current = json.load(f)

    delta = current["accuracy"] - baseline["accuracy"]
    print(f"Baseline: {baseline['accuracy']*100:.0f}% | Current: {current['accuracy']*100:.0f}% | Delta: {delta*100:+.0f}%")

    if delta < -0.05:
        print("⚠ REGRESSION DETECTED: accuracy dropped >5%")
    elif delta > 0.05:
        print("✓ IMPROVEMENT: accuracy improved >5%")
    else:
        print("~ Stable: within ±5% of baseline")


def main():
    parser = argparse.ArgumentParser(description="Automated Evaluation Harness")
    parser.add_argument("--run", action="store_true", help="Run full evaluation")
    parser.add_argument("--compare", action="store_true", help="Compare with baseline")
    parser.add_argument("--baseline", action="store_true", help="Save current as baseline")
    parser.add_argument("--domains", type=str, default=None, help="Comma-separated domains")
    args = parser.parse_args()

    domains = args.domains.split(",") if args.domains else None

    if args.run:
        results = run_eval(domains)
        if args.baseline:
            os.makedirs(EVAL_DIR, exist_ok=True)
            with open(BASELINE_FILE, "w") as f:
                json.dump(results, f, indent=2)
            print(f"Saved as baseline: {BASELINE_FILE}")
    elif args.compare:
        compare_with_baseline()
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
