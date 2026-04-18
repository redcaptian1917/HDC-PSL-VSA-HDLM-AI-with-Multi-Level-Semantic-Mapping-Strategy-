#!/usr/bin/env python3
"""
Causal DAG edge builder from ConceptNet tuples (#336).

Parses ConceptNet facts with the value shape "subject [Predicate] object"
into (source, target, predicate) edges and inserts them into the
fact_edges table. Concept keys are namespaced `concept:<text>` so they
are addressable without having to join back to the facts table's
opaque cn_... keys.

Predicates included:
- IsA              (taxonomic)      — 221K rows
- UsedFor          (functional)     — 40K rows
- HasSubevent      (event structure) — 25K rows
- HasPrerequisite  (causal/enabling) — 23K rows
- Causes           (causal)         — 17K rows
- PartOf           (meronym)        — 13K rows
- MotivatedByGoal  (intentional)    — 9K rows
- CausesDesire     (causal/desire)  — 5K rows

Total ~353K edges. Runs server-respectful with batched inserts + sleeps.
"""

import os
import re
import sqlite3
import time

DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
PREDICATES = [
    "IsA", "UsedFor", "HasSubevent", "HasPrerequisite",
    "Causes", "PartOf", "MotivatedByGoal", "CausesDesire",
]
BATCH = 500
SLEEP_PER_BATCH = float(os.environ.get("EDGE_INGEST_SLEEP", "0.5"))

# Predicate → default edge strength. Well-evidenced taxonomic (IsA) +
# causal (Causes) get high strength; looser relations (HasSubevent)
# slightly lower. ConceptNet doesn't ship per-row confidence in the
# value text, so we assign by predicate type.
STRENGTH = {
    "IsA": 0.85,
    "UsedFor": 0.80,
    "HasSubevent": 0.75,
    "HasPrerequisite": 0.80,
    "Causes": 0.85,
    "PartOf": 0.85,
    "MotivatedByGoal": 0.75,
    "CausesDesire": 0.70,
}


def normalize(s: str) -> str:
    """Trim + lowercase + collapse whitespace for concept-key canonicalization."""
    return " ".join(s.strip().lower().split())


def open_db() -> sqlite3.Connection:
    conn = sqlite3.connect(DB, timeout=300)
    conn.execute("PRAGMA busy_timeout=600000")
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA synchronous=NORMAL")
    return conn


def main() -> None:
    t0 = time.time()
    conn = open_db()
    existing = conn.execute("SELECT COUNT(*) FROM fact_edges").fetchone()[0]
    print(f"[start] existing fact_edges: {existing:,}", flush=True)

    total_inserted = 0
    for pred in PREDICATES:
        sep = f"[{pred}]"
        # Match facts whose value contains the predicate marker. We pull
        # both the original key (for `evidence` provenance) and the full
        # value (for parsing).
        cur = conn.execute(
            "SELECT key, value FROM facts WHERE source='conceptnet' AND instr(value, ?)>0",
            (sep,),
        )
        batch = []
        n_pred = 0
        p_t0 = time.time()
        strength = STRENGTH[pred]
        for fact_key, value in cur:
            # Parse "subject [Predicate] object". Use the first occurrence.
            idx = value.find(sep)
            if idx <= 0:
                continue
            subj = normalize(value[:idx])
            obj = normalize(value[idx + len(sep):])
            if not subj or not obj or len(subj) > 200 or len(obj) > 200:
                continue
            source_key = f"concept:{subj}"
            target_key = f"concept:{obj}"
            batch.append((source_key, target_key, pred, strength, fact_key))
            n_pred += 1
            if len(batch) >= BATCH:
                conn.executemany(
                    "INSERT OR IGNORE INTO fact_edges"
                    "(source_key, target_key, edge_type, strength, evidence) "
                    "VALUES (?, ?, ?, ?, ?)",
                    batch,
                )
                conn.commit()
                total_inserted += len(batch)
                batch = []
                time.sleep(SLEEP_PER_BATCH)
        if batch:
            conn.executemany(
                "INSERT OR IGNORE INTO fact_edges"
                "(source_key, target_key, edge_type, strength, evidence) "
                "VALUES (?, ?, ?, ?, ?)",
                batch,
            )
            conn.commit()
            total_inserted += len(batch)
        elapsed = time.time() - p_t0
        print(
            f"[{pred:>18}] {n_pred:>7,} rows parsed in {elapsed:.0f}s",
            flush=True,
        )

    # Force a WAL checkpoint so the writes are materialised and the
    # server's next read sees the new edges without additional lag.
    try:
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")
    except sqlite3.Error as e:
        print(f"[warn] wal_checkpoint failed: {e}", flush=True)

    final = conn.execute("SELECT COUNT(*) FROM fact_edges").fetchone()[0]
    conn.close()
    elapsed = time.time() - t0
    print(
        f"[done] fact_edges {existing:,} → {final:,} "
        f"(+{final - existing:,})  elapsed {elapsed:.0f}s",
        flush=True,
    )


if __name__ == "__main__":
    main()
