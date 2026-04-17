# Claude 1 — DO THESE NOW

## 1. Ingest the 6.3GB Open Payments CSV (50K more rows)
```python
import sqlite3, os
conn = sqlite3.connect(os.path.expanduser('~/.local/share/plausiden/brain.db'), timeout=60)
conn.execute('PRAGMA busy_timeout=600000')
batch = []
with open(os.path.expanduser('~/Development/PlausiDen/New training sets i found/OP_DTL_GNRL_PGYR2016_P01172018.csv'), errors='replace') as f:
    for i, line in enumerate(f):
        if i < 50000: continue  # skip already ingested
        if i >= 150000: break
        if len(line) > 20:
            batch.append((f'open_payments2:{i}', line[:500], 0.80, 'open_payments_2', 'finance', 0.80))
        if len(batch) >= 5000:
            conn.executemany('INSERT OR IGNORE INTO facts(key,value,confidence,source,domain,quality_score) VALUES(?,?,?,?,?,?)', batch)
            conn.commit(); batch = []
if batch:
    conn.executemany('INSERT OR IGNORE INTO facts(key,value,confidence,source,domain,quality_score) VALUES(?,?,?,?,?,?)', batch)
    conn.commit()
conn.close()
```

## 2. Generate 200 training pairs via Ollama
Focus on the 4 CRITICAL gap domains (philosophy, pentesting, politics, communication):
```bash
for topic in "Explain Kant's categorical imperative" "What is social contract theory" "How does port scanning work" "Explain privilege escalation on Linux" "What is gerrymandering" "Explain the electoral college" "How does active listening improve communication" "What is nonviolent communication"; do
  curl -s http://localhost:11434/api/generate -d "{\"model\":\"qwen2.5-coder:7b\",\"prompt\":\"$topic\",\"stream\":false,\"options\":{\"temperature\":0.7,\"num_predict\":400}}" | python3 -c "import json,sys; r=json.load(sys.stdin).get('response',''); open('/home/user/LFI-data/critical_gaps.jsonl','a').write(json.dumps({'instruction':\"$topic\",'output':r,'domain':'gap_fill'})+'\n') if len(r)>50 else None"
done
```

## 3. Run dedup check on brain.db
```sql
SELECT value, COUNT(*) as cnt FROM facts GROUP BY value HAVING cnt > 1 ORDER BY cnt DESC LIMIT 20;
```
Report how many exact duplicates exist.

## 4. Delete facts shorter than 10 characters
```sql
DELETE FROM facts WHERE length(value) < 10;
```
Report how many were deleted.

## 5. Check FTS5 sync
```sql
SELECT (SELECT COUNT(*) FROM facts) as facts, (SELECT COUNT(*) FROM facts_fts) as fts;
```
If they don't match, rebuild FTS5.

## 6. Ingest the IoT botnet dataset fully (1.7GB)
Currently only 10K rows ingested. Get 100K:
```python
# Same pattern as above, read from detection+of+iot+botnet+attacks+n+baiot.zip
# Insert 100K rows with source='iot_botnet_full', domain='cybersecurity'
```

## 7. Generate 50 error recovery training pairs
Format: user corrects AI, AI acknowledges and gives correct answer.

## 8. Generate 50 situational awareness pairs
Time of day, day of week, seasonal, location context responses.

## 9. Export combined_training_v4.jsonl
Merge ALL .jsonl files from /home/user/LFI-data/ into one combined file.

## 10. Security audit: grep for remaining unwrap() in api.rs
```bash
grep -n "\.unwrap()" /root/LFI/lfi_vsa_core/src/api.rs
```
Report count and line numbers.

## 11. Check for any new datasets in the download folder
```bash
find ~/Development/PlausiDen/"New training sets i found/" -newer /root/LFI/SESSION_HANDOFF_20260417.md -not -name "*.crdownload" -size +1M
```
Ingest any that haven't been processed.

## 12. Push everything to GitHub (both repos)

## 13. Report progress on bus.jsonl after every 3 tasks

## 14. Restart your loop — NEVER IDLE

## 15. When Kitsune 17.7GB lands, ingest it (sample 100K rows)
