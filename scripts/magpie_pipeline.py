#!/usr/bin/env python3
"""
Magpie Self-Chat Pipeline — Automated training pair generation.

Uses the local Ollama model to generate instruction-output pairs across
diverse domains. Runs as a batch job or continuous daemon.

Usage:
    python3 magpie_pipeline.py --batch 50          # Generate 50 pairs
    python3 magpie_pipeline.py --continuous --rate 5  # 5 pairs/min forever
    python3 magpie_pipeline.py --domains cyber,physics --batch 20
"""

import argparse
import json
import hashlib
import os
import sqlite3
import sys
import time
import requests
from typing import Optional

BRAIN_DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
PAIRS_DIR = os.path.expanduser("~/LFI-data/magpie_pairs")
OLLAMA_URL = "http://localhost:11434/api/generate"
DEFAULT_MODEL = os.environ.get("PLAUSIDEN_MODEL", "qwen2.5-coder:7b")

# Domain → prompt templates for generating diverse Q&A pairs
DOMAIN_PROMPTS = {
    "cybersecurity": [
        "Ask a detailed question about network security and provide an expert answer.\nQ:",
        "Explain a common web vulnerability and how to defend against it.\nQ:",
        "Ask about cryptographic protocols and explain how they work.\nQ:",
        "Describe an incident response scenario and the correct procedure.\nQ:",
        "Ask about malware analysis techniques and explain.\nQ:",
        "Explain a penetration testing methodology step by step.\nQ:",
    ],
    "programming": [
        "Ask a coding question about Python or Rust and provide the answer.\nQ:",
        "Explain a software design pattern with a code example.\nQ:",
        "Ask about debugging a tricky programming problem and solve it.\nQ:",
        "Explain how databases handle concurrency and transactions.\nQ:",
        "Ask about async programming patterns and explain with examples.\nQ:",
        "Describe a common algorithm and its time complexity.\nQ:",
    ],
    "physics": [
        "Ask an interesting physics question and explain clearly.\nQ:",
        "Explain a quantum mechanics concept in simple terms.\nQ:",
        "Ask about thermodynamics principles and explain.\nQ:",
        "Describe an electromagnetic phenomenon and the physics behind it.\nQ:",
    ],
    "mathematics": [
        "Ask a math question about linear algebra and solve it.\nQ:",
        "Explain a statistics concept with examples.\nQ:",
        "Ask a probability question and solve it step by step.\nQ:",
        "Explain a calculus concept with applications.\nQ:",
    ],
    "conversational": [
        "Start a friendly conversation about travel and respond naturally.\nQ:",
        "Discuss the benefits of learning new skills.\nQ:",
        "Have a thoughtful conversation about career development.\nQ:",
        "Discuss a philosophical question about ethics.\nQ:",
    ],
    "history": [
        "Ask about a major historical event and explain its significance.\nQ:",
        "Discuss a historical figure and their impact on society.\nQ:",
    ],
    "biology": [
        "Ask about a biological mechanism and explain it.\nQ:",
        "Explain a genetics concept with examples.\nQ:",
    ],
    "economics": [
        "Explain an economic concept with real-world examples.\nQ:",
        "Discuss a market trend and its implications.\nQ:",
    ],
    "technology": [
        "Explain how container orchestration works.\nQ:",
        "Discuss AI and machine learning trends.\nQ:",
        "Explain cloud computing architecture patterns.\nQ:",
    ],
}


def generate_pair(model: str, prompt: str, domain: str, temperature: float = 0.85) -> Optional[dict]:
    """Generate a single Q&A pair from a prompt template."""
    try:
        r = requests.post(OLLAMA_URL, json={
            "model": model,
            "stream": False,
            "prompt": prompt,
            "options": {"temperature": temperature, "num_predict": 350}
        }, timeout=60)

        if r.status_code != 200:
            return None

        text = r.json().get("response", "")

        # Parse Q: and A: format
        if "A:" in text:
            parts = text.split("A:", 1)
            q = parts[0].strip().lstrip("Q:").strip()
            a = parts[1].strip()
            if len(q) > 10 and len(a) > 30:
                return {
                    "instruction": q,
                    "output": a,
                    "domain": domain,
                    "source": "magpie_pipeline",
                    "quality": 0.75,
                    "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
                }

        # Fallback: split on double newline
        lines = [l.strip() for l in text.strip().split("\n") if l.strip() and len(l.strip()) > 10]
        if len(lines) >= 2:
            return {
                "instruction": lines[0],
                "output": " ".join(lines[1:]),
                "domain": domain,
                "source": "magpie_pipeline",
                "quality": 0.70,
                "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
            }

    except requests.exceptions.Timeout:
        return None
    except Exception:
        return None

    return None


def ingest_to_db(pairs: list, db_path: str = BRAIN_DB) -> int:
    """Ingest generated pairs into brain.db."""
    conn = sqlite3.connect(db_path, timeout=30)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=30000")

    added = 0
    for pair in pairs:
        q = pair["instruction"]
        a = pair["output"]
        domain = pair.get("domain", "general")
        quality = pair.get("quality", 0.75)
        source = pair.get("source", "magpie_pipeline")

        key = f"magpie_{hashlib.sha256((q + a).encode()).hexdigest()[:16]}"
        value = f"Q: {q}\nA: {a}"

        try:
            conn.execute(
                "INSERT OR IGNORE INTO facts (key, value, source, confidence, domain, quality_score) "
                "VALUES (?, ?, ?, ?, ?, ?)",
                (key, value[:5000], source, quality, domain, quality)
            )
            added += 1
        except Exception:
            pass

    conn.commit()
    conn.close()
    return added


def save_to_file(pairs: list, output_dir: str = PAIRS_DIR) -> str:
    """Save pairs to a timestamped JSONL file."""
    os.makedirs(output_dir, exist_ok=True)
    ts = time.strftime("%Y%m%d_%H%M%S")
    path = os.path.join(output_dir, f"magpie_{ts}.jsonl")
    with open(path, "w") as f:
        for p in pairs:
            f.write(json.dumps(p) + "\n")
    return path


def run_batch(count: int, domains: list, model: str) -> list:
    """Generate a batch of Q&A pairs."""
    pairs = []
    prompts = []

    # Build prompt list cycling through domains
    for domain in domains:
        templates = DOMAIN_PROMPTS.get(domain, [])
        for t in templates:
            prompts.append((domain, t))

    if not prompts:
        print("No prompts available for specified domains")
        return pairs

    idx = 0
    generated = 0
    while generated < count:
        domain, prompt = prompts[idx % len(prompts)]
        pair = generate_pair(model, prompt, domain)
        if pair:
            pairs.append(pair)
            generated += 1
            print(f"  [{generated}/{count}] [{domain}] {pair['instruction'][:60]}...")
        idx += 1
        # Vary temperature slightly for diversity
        if idx % len(prompts) == 0 and idx > 0:
            # Completed a full cycle — add noise to prompts
            pass

    return pairs


def main():
    parser = argparse.ArgumentParser(description="Magpie Self-Chat Pipeline")
    parser.add_argument("--batch", type=int, default=0, help="Generate N pairs and exit")
    parser.add_argument("--continuous", action="store_true", help="Run continuously")
    parser.add_argument("--rate", type=int, default=5, help="Pairs per minute (continuous mode)")
    parser.add_argument("--domains", type=str, default=None, help="Comma-separated domain list")
    parser.add_argument("--model", type=str, default=DEFAULT_MODEL, help="Ollama model")
    parser.add_argument("--no-ingest", action="store_true", help="Don't ingest to brain.db")
    args = parser.parse_args()

    domains = args.domains.split(",") if args.domains else list(DOMAIN_PROMPTS.keys())
    print(f"Magpie Pipeline: model={args.model}, domains={domains}")

    if args.batch > 0:
        print(f"Generating {args.batch} pairs...")
        pairs = run_batch(args.batch, domains, args.model)

        if pairs:
            path = save_to_file(pairs)
            print(f"Saved {len(pairs)} pairs to {path}")

            if not args.no_ingest:
                added = ingest_to_db(pairs)
                print(f"Ingested {added} pairs into brain.db")

    elif args.continuous:
        print(f"Continuous mode: {args.rate} pairs/min")
        delay = 60.0 / max(args.rate, 1)

        batch = []
        while True:
            domain = domains[int(time.time()) % len(domains)]
            templates = DOMAIN_PROMPTS.get(domain, [])
            if not templates:
                continue
            prompt = templates[int(time.time() / 10) % len(templates)]

            pair = generate_pair(args.model, prompt, domain)
            if pair:
                batch.append(pair)
                print(f"  [{len(batch)}] [{domain}] {pair['instruction'][:60]}...")

                # Save every 10 pairs
                if len(batch) >= 10:
                    path = save_to_file(batch)
                    if not args.no_ingest:
                        ingest_to_db(batch)
                    print(f"  Saved batch of {len(batch)} → {path}")
                    batch = []

            time.sleep(delay)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
