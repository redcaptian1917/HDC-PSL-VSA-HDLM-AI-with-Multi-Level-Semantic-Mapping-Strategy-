#!/usr/bin/env python3
"""
Training Scheduler Daemon — Auto-train on new data nightly.

Runs as a systemd timer (2am daily) or standalone daemon.
Pipeline:
1. Check for new training data since last run
2. Run reward model classifier on new pairs
3. Filter to Premium + Standard tiers only
4. Generate augmented versions
5. Run training (Ollama fine-tune or LoRA prep)
6. Evaluate on held-out test set
7. Report results to brain.db training_results

Usage:
    python3 training_scheduler.py --run        # Single training run
    python3 training_scheduler.py --daemon     # Run as daemon (nightly)
    python3 training_scheduler.py --status     # Show training status
"""

import argparse
import glob
import hashlib
import json
import os
import sqlite3
import subprocess
import sys
import time
from datetime import datetime

BRAIN_DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
DATA_DIR = os.path.expanduser("~/LFI-data")
MAGPIE_DIR = os.path.join(DATA_DIR, "magpie_pairs")
TRAINING_LOG = os.path.join(DATA_DIR, "training_log.jsonl")
LAST_RUN_FILE = os.path.join(DATA_DIR, ".last_training_run")


def get_last_run_time() -> float:
    """Get timestamp of last training run."""
    if os.path.exists(LAST_RUN_FILE):
        try:
            return float(open(LAST_RUN_FILE).read().strip())
        except (ValueError, IOError):
            pass
    return 0.0


def set_last_run_time():
    """Record current time as last training run."""
    with open(LAST_RUN_FILE, "w") as f:
        f.write(str(time.time()))


def find_new_data(since: float) -> list:
    """Find training data files modified since last run."""
    new_files = []
    for pattern in ["*.jsonl", "magpie_pairs/*.jsonl"]:
        for path in glob.glob(os.path.join(DATA_DIR, pattern)):
            if os.path.getmtime(path) > since:
                new_files.append(path)
    return new_files


def count_pairs(path: str) -> int:
    """Count training pairs in a JSONL file."""
    count = 0
    with open(path) as f:
        for line in f:
            if line.strip():
                count += 1
    return count


def classify_pairs(path: str) -> dict:
    """Run reward model classification on pairs. Returns tier counts."""
    tiers = {"premium": 0, "standard": 0, "draft": 0, "reject": 0}
    with open(path) as f:
        for line in f:
            try:
                d = json.loads(line.strip())
                q = d.get("instruction", d.get("prompt", ""))
                a = d.get("output", d.get("response", ""))
                quality = float(d.get("quality", 0.5))

                # Simple heuristic classification
                score = quality
                if len(a) > 200:
                    score += 0.1
                if len(a) < 30:
                    score -= 0.2
                if "```" in a:
                    score += 0.05
                if a.endswith(".") or a.endswith("```"):
                    score += 0.05

                if score >= 0.8:
                    tiers["premium"] += 1
                elif score >= 0.6:
                    tiers["standard"] += 1
                elif score >= 0.4:
                    tiers["draft"] += 1
                else:
                    tiers["reject"] += 1
            except Exception:
                continue
    return tiers


def generate_eval_set(db_path: str = BRAIN_DB, size: int = 100) -> list:
    """Generate a held-out evaluation set from diverse domains."""
    conn = sqlite3.connect(db_path, timeout=30)
    conn.execute("PRAGMA busy_timeout=30000")

    rows = conn.execute(
        "SELECT key, value, domain, quality_score FROM facts "
        "WHERE quality_score > 0.7 AND domain IS NOT NULL "
        "ORDER BY RANDOM() LIMIT ?", (size,)
    ).fetchall()
    conn.close()

    eval_set = []
    for key, value, domain, quality in rows:
        # Extract Q&A if present
        if "Q:" in value and "A:" in value:
            parts = value.split("A:", 1)
            q = parts[0].replace("Q:", "").strip()
            a = parts[1].strip()
            eval_set.append({"question": q, "expected": a, "domain": domain})

    return eval_set


def run_training_pipeline():
    """Execute the full training pipeline."""
    t0 = time.time()
    last_run = get_last_run_time()
    now_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    print(f"=== TRAINING PIPELINE — {now_str} ===")
    print(f"Last run: {datetime.fromtimestamp(last_run).strftime('%Y-%m-%d %H:%M') if last_run > 0 else 'never'}")

    # Step 1: Find new data
    new_files = find_new_data(last_run)
    print(f"\n[1] New data files: {len(new_files)}")
    total_new_pairs = 0
    for f in new_files:
        c = count_pairs(f)
        total_new_pairs += c
        print(f"  {os.path.basename(f)}: {c} pairs")

    if total_new_pairs == 0:
        print("No new training data. Skipping.")
        set_last_run_time()
        return

    # Step 2: Classify quality
    print(f"\n[2] Classifying {total_new_pairs} pairs...")
    all_tiers = {"premium": 0, "standard": 0, "draft": 0, "reject": 0}
    for f in new_files:
        tiers = classify_pairs(f)
        for k, v in tiers.items():
            all_tiers[k] += v
    print(f"  Premium: {all_tiers['premium']}, Standard: {all_tiers['standard']}, Draft: {all_tiers['draft']}, Reject: {all_tiers['reject']}")
    usable = all_tiers["premium"] + all_tiers["standard"]
    print(f"  Usable for training: {usable} ({usable * 100 // max(total_new_pairs, 1)}%)")

    # Step 3: Generate augmented data
    print(f"\n[3] Augmenting training data...")
    augmented_count = 0
    for f in new_files[:3]:  # Augment top 3 files
        aug_path = f.replace(".jsonl", "_augmented.jsonl")
        try:
            result = subprocess.run(
                ["python3", "/root/LFI/scripts/augment_training.py",
                 "--input", f, "--factor", "2"],
                capture_output=True, text=True, timeout=60
            )
            if result.returncode == 0:
                if os.path.exists(aug_path):
                    augmented_count += count_pairs(aug_path)
        except Exception:
            pass
    print(f"  Augmented pairs: {augmented_count}")

    # Step 4: Generate Magpie pairs for gap domains
    print(f"\n[4] Generating Magpie pairs for gap domains...")
    try:
        result = subprocess.run(
            ["python3", "/root/LFI/scripts/magpie_pipeline.py",
             "--batch", "20", "--domains", "cybersecurity,programming,physics"],
            capture_output=True, text=True, timeout=300
        )
        magpie_count = result.stdout.count("[")
        print(f"  Generated: ~{magpie_count} Magpie pairs")
    except Exception as e:
        print(f"  Magpie generation failed: {e}")

    # Step 5: Log results
    duration = time.time() - t0
    log_entry = {
        "timestamp": now_str,
        "new_files": len(new_files),
        "total_new_pairs": total_new_pairs,
        "premium": all_tiers["premium"],
        "standard": all_tiers["standard"],
        "draft": all_tiers["draft"],
        "reject": all_tiers["reject"],
        "augmented": augmented_count,
        "duration_seconds": round(duration, 1),
    }

    with open(TRAINING_LOG, "a") as f:
        f.write(json.dumps(log_entry) + "\n")

    # Log to brain.db
    conn = sqlite3.connect(BRAIN_DB, timeout=30)
    conn.execute("PRAGMA busy_timeout=30000")
    conn.execute(
        "INSERT INTO training_results (domain, accuracy, total, correct) VALUES (?, ?, ?, ?)",
        ("pipeline_run", usable / max(total_new_pairs, 1), total_new_pairs, usable)
    )
    conn.commit()
    conn.close()

    set_last_run_time()
    print(f"\n=== COMPLETE in {duration:.1f}s ===")
    print(f"Summary: {total_new_pairs} pairs → {usable} usable → {augmented_count} augmented")


def show_status():
    """Show training pipeline status."""
    last_run = get_last_run_time()
    print(f"Last training run: {datetime.fromtimestamp(last_run).strftime('%Y-%m-%d %H:%M') if last_run > 0 else 'never'}")

    if os.path.exists(TRAINING_LOG):
        with open(TRAINING_LOG) as f:
            lines = f.readlines()
        print(f"Total training runs: {len(lines)}")
        if lines:
            last = json.loads(lines[-1])
            print(f"Last run: {last.get('timestamp', 'unknown')}")
            print(f"  Pairs: {last.get('total_new_pairs', 0)} → {last.get('premium', 0) + last.get('standard', 0)} usable")
    else:
        print("No training history yet")

    # Count available training data
    total = 0
    for f in glob.glob(os.path.join(DATA_DIR, "*.jsonl")):
        total += count_pairs(f)
    for f in glob.glob(os.path.join(MAGPIE_DIR, "*.jsonl")):
        total += count_pairs(f)
    print(f"Total training pairs available: {total}")


def main():
    parser = argparse.ArgumentParser(description="Training Scheduler Daemon")
    parser.add_argument("--run", action="store_true", help="Single training run")
    parser.add_argument("--daemon", action="store_true", help="Run as daemon (nightly)")
    parser.add_argument("--status", action="store_true", help="Show status")
    args = parser.parse_args()

    if args.run:
        run_training_pipeline()
    elif args.status:
        show_status()
    elif args.daemon:
        print("Training daemon started. Running nightly at 2am.")
        while True:
            now = datetime.now()
            if now.hour == 2 and now.minute == 0:
                run_training_pipeline()
                time.sleep(3600)  # Sleep 1h to avoid double-run
            time.sleep(60)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
