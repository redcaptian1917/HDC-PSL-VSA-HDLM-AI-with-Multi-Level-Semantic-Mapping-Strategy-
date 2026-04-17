#!/usr/bin/env python3
"""
Fact Deduplication Pipeline — Find and remove near-duplicate facts.

Uses MinHash-based similarity detection to find facts with >90% text
overlap. Reports duplicates for review or auto-removes them.

Usage:
    python3 dedup_facts.py --sample 10000 --threshold 0.9 --report
    python3 dedup_facts.py --sample 10000 --threshold 0.9 --remove
"""

import argparse
import hashlib
import os
import sqlite3
import sys
import time
from collections import defaultdict

BRAIN_DB = os.path.expanduser("~/.local/share/plausiden/brain.db")


def shingle(text: str, k: int = 3) -> set:
    """Generate k-character shingles from text."""
    text = text.lower().strip()
    if len(text) < k:
        return {text}
    return {text[i:i+k] for i in range(len(text) - k + 1)}


def jaccard_similarity(s1: set, s2: set) -> float:
    """Compute Jaccard similarity between two sets."""
    if not s1 or not s2:
        return 0.0
    intersection = len(s1 & s2)
    union = len(s1 | s2)
    return intersection / union if union > 0 else 0.0


def minhash_signature(shingles: set, num_hashes: int = 100) -> list:
    """Generate MinHash signature for a set of shingles."""
    sig = []
    for i in range(num_hashes):
        min_hash = float('inf')
        for s in shingles:
            h = hash((s, i)) & 0xFFFFFFFF
            if h < min_hash:
                min_hash = h
        sig.append(min_hash)
    return sig


def estimate_similarity(sig1: list, sig2: list) -> float:
    """Estimate Jaccard similarity from MinHash signatures."""
    if len(sig1) != len(sig2):
        return 0.0
    matches = sum(1 for a, b in zip(sig1, sig2) if a == b)
    return matches / len(sig1)


def find_duplicates(sample_size: int = 10000, threshold: float = 0.9) -> list:
    """Find near-duplicate fact pairs using MinHash."""
    conn = sqlite3.connect(BRAIN_DB, timeout=30)
    conn.execute("PRAGMA busy_timeout=30000")

    print(f"Sampling {sample_size} facts...")
    t0 = time.time()

    rows = conn.execute(
        "SELECT key, value, source, domain FROM facts ORDER BY RANDOM() LIMIT ?",
        (sample_size,)
    ).fetchall()
    print(f"  Loaded {len(rows)} facts in {time.time()-t0:.1f}s")

    # Compute shingles and signatures
    print("Computing MinHash signatures...")
    signatures = {}
    for key, value, source, domain in rows:
        s = shingle(value[:500], k=4)  # Use first 500 chars, 4-char shingles
        sig = minhash_signature(s, num_hashes=50)
        signatures[key] = (sig, value[:200], source, domain)

    # LSH: bucket by signature bands
    print("Finding candidates via LSH...")
    num_bands = 10
    band_size = 5  # 50 hashes / 10 bands = 5 per band
    buckets = defaultdict(list)

    for key, (sig, _, _, _) in signatures.items():
        for band in range(num_bands):
            band_sig = tuple(sig[band * band_size:(band + 1) * band_size])
            bucket_key = (band, hash(band_sig))
            buckets[bucket_key].append(key)

    # Find candidate pairs from same buckets
    candidates = set()
    for bucket_key, keys in buckets.items():
        if len(keys) < 2 or len(keys) > 20:  # Skip singletons and huge buckets
            continue
        for i in range(len(keys)):
            for j in range(i + 1, min(len(keys), i + 5)):
                pair = tuple(sorted([keys[i], keys[j]]))
                candidates.add(pair)

    print(f"  {len(candidates)} candidate pairs from {len(buckets)} buckets")

    # Verify candidates with actual similarity
    duplicates = []
    for key_a, key_b in candidates:
        if key_a not in signatures or key_b not in signatures:
            continue
        sim = estimate_similarity(signatures[key_a][0], signatures[key_b][0])
        if sim >= threshold:
            duplicates.append({
                "key_a": key_a,
                "key_b": key_b,
                "similarity": round(sim, 3),
                "preview_a": signatures[key_a][1][:80],
                "preview_b": signatures[key_b][1][:80],
                "source_a": signatures[key_a][2],
                "source_b": signatures[key_b][3],
            })

    conn.close()
    duplicates.sort(key=lambda x: -x["similarity"])
    print(f"\nFound {len(duplicates)} duplicate pairs (threshold={threshold})")
    return duplicates


def main():
    parser = argparse.ArgumentParser(description="Fact Deduplication Pipeline")
    parser.add_argument("--sample", type=int, default=10000, help="Sample size")
    parser.add_argument("--threshold", type=float, default=0.9, help="Similarity threshold")
    parser.add_argument("--report", action="store_true", help="Print report only")
    parser.add_argument("--remove", action="store_true", help="Remove duplicates (keep first)")
    args = parser.parse_args()

    duplicates = find_duplicates(args.sample, args.threshold)

    if args.report or not args.remove:
        print(f"\n=== DUPLICATE REPORT ({len(duplicates)} pairs) ===")
        for i, d in enumerate(duplicates[:20]):
            print(f"\n  Pair {i+1} (similarity: {d['similarity']}):")
            print(f"    A [{d['source_a']}]: {d['preview_a']}...")
            print(f"    B [{d['source_b']}]: {d['preview_b']}...")

    if args.remove and duplicates:
        conn = sqlite3.connect(BRAIN_DB, timeout=30)
        conn.execute("PRAGMA busy_timeout=30000")
        removed = 0
        for d in duplicates:
            # Keep key_a (first alphabetically), remove key_b
            conn.execute("DELETE FROM facts WHERE key = ?", (d["key_b"],))
            removed += 1
        conn.commit()
        conn.close()
        print(f"\nRemoved {removed} duplicate facts")


if __name__ == "__main__":
    main()
