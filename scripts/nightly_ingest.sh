#!/bin/bash
# PlausiDen Nightly Dataset Ingestion
# Runs via systemd timer — checks for new datasets and ingests them
# Also refreshes DuckDB analytics snapshot

set -euo pipefail

LOG="/var/log/lfi/nightly_ingest.log"
DB="$HOME/.local/share/plausiden/brain.db"
DATASET_DIR="$HOME/Development/PlausiDen/New training sets i found"
HF_DIR="$HOME/LFI-data/hf-conversations"

echo "[$(date)] Nightly ingest starting" >> "$LOG"

# 1. Ingest any new zip files
python3 -u << 'PYEOF' >> "$LOG" 2>&1
import zipfile, sqlite3, os

DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
conn = sqlite3.connect(DB, timeout=60)
conn.execute("PRAGMA busy_timeout=600000")
conn.execute("PRAGMA journal_mode=WAL")
processed = set(r[0] for r in conn.execute("SELECT DISTINCT source FROM facts").fetchall())

SRC = os.path.expanduser("~/Development/PlausiDen/New training sets i found")
total = 0
for fname in sorted(os.listdir(SRC)):
    if not fname.endswith(".zip"): continue
    source = fname.replace(".zip","").replace("+","_").replace(" ","_").lower()[:40]
    if source in processed: continue
    zpath = f"{SRC}/{fname}"
    try:
        with zipfile.ZipFile(zpath) as z:
            for name in z.namelist():
                if any(name.endswith(e) for e in [".csv",".data",".txt",".arff"]) and "__MACOSX" not in name:
                    data = z.read(name).decode("utf-8", errors="replace")
                    lines = data.strip().split("\n")
                    batch = [(f"{source}:{i}", line[:500], 0.80, source, "general", 0.80)
                             for i, line in enumerate(lines[:10000]) if len(line) > 10]
                    if batch:
                        conn.executemany("INSERT OR IGNORE INTO facts(key,value,confidence,source,domain,quality_score) VALUES(?,?,?,?,?,?)", batch)
                        conn.commit()
                        total += len(batch)
                        print(f"  {fname}: {len(batch)} facts")
                    break
    except: pass
conn.close()
print(f"New facts from zips: {total}")
PYEOF

# 2. Ingest any new JSONL files from HF
python3 -u << 'PYEOF' >> "$LOG" 2>&1
import json, sqlite3, os

DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
conn = sqlite3.connect(DB, timeout=60)
conn.execute("PRAGMA busy_timeout=600000")
processed = set(r[0] for r in conn.execute("SELECT DISTINCT source FROM facts").fetchall())

SRC = os.path.expanduser("~/LFI-data/hf-conversations")
total = 0
for fname in sorted(os.listdir(SRC)):
    if not fname.endswith(".jsonl"): continue
    source = fname.replace(".jsonl","").replace("-","_").lower()[:40]
    if source in processed: continue
    path = f"{SRC}/{fname}"
    if os.path.getsize(path) < 1000: continue
    batch = []
    try:
        with open(path) as f:
            for i, line in enumerate(f):
                if i >= 50000: break
                item = json.loads(line)
                text = json.dumps(item)[:500]
                if len(text) > 20:
                    batch.append((f"{source}:{i}", text, 0.85, source, "conversational", 0.85))
        if batch:
            conn.executemany("INSERT OR IGNORE INTO facts(key,value,confidence,source,domain,quality_score) VALUES(?,?,?,?,?,?)", batch)
            conn.commit()
            total += len(batch)
            print(f"  {fname}: {len(batch)} facts")
    except: pass
conn.close()
print(f"New facts from JSONL: {total}")
PYEOF

# 3. Refresh DuckDB analytics
python3 -u << 'PYEOF' >> "$LOG" 2>&1
import duckdb, os
BRAIN = os.path.expanduser("~/.local/share/plausiden/brain.db")
ANALYTICS = os.path.expanduser("~/.local/share/plausiden/analytics.duckdb")
duck = duckdb.connect(ANALYTICS)
duck.execute("INSTALL sqlite; LOAD sqlite;")
duck.execute(f"CREATE OR REPLACE TABLE fact_analytics AS SELECT domain, source, COALESCE(quality_score, 0.5) as quality_score, length(value) as value_length FROM sqlite_scan('{BRAIN}', 'facts')")
count = duck.execute("SELECT COUNT(*) FROM fact_analytics").fetchone()[0]
duck.close()
print(f"DuckDB refreshed: {count:,} facts")
PYEOF

# 4. Push to GitHub
cd /root/LFI && git add -A && git commit -m "Nightly auto-ingest $(date +%Y-%m-%d)

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>" 2>/dev/null && SSH_ASKPASS="" git push 2>/dev/null

echo "[$(date)] Nightly ingest complete" >> "$LOG"
