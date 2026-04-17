# train_adaptive.sh Audit — Claude-1

## Bugs Found

### BUG 1: SQL Injection in get_domain_priority (Line 30)
```bash
fact_count=$(sqlite3 "$DB" "SELECT count(*) FROM facts WHERE key LIKE '${domain}%';" 2>/dev/null || echo "0")
```
**Issue:** `$domain` is interpolated directly into SQL. If a domain name contained `'; DROP TABLE facts; --` it would execute. Low risk since domains are hardcoded, but violates AVP-2.
**Fix:** Use parameterized query or validate domain against whitelist.

### BUG 2: Priority calculation overflow (Line 58)
```bash
local priority=$(( elapsed + (100 - sessions) * 60 + (1000 - fact_count) ))
```
**Issue:** `fact_count` for web_knowledge is 26M+. `1000 - 26000000 = -25999000`. This gives massive negative priority to large domains, which is actually correct behavior (deprioritizes over-represented domains). But the arithmetic could overflow on 32-bit shells.
**Fix:** Cap fact_count: `fact_count=$((fact_count > 1000 ? 1000 : fact_count))`

### BUG 3: Missing domains (Line 16)
```bash
DOMAINS=(social math code security philosophy biology chemistry physics language psychology sales)
```
**Issue:** Missing critical domains from brain.db: pentesting, economics, politics, cybersecurity, adversarial, reasoning, commonsense, legal, finance. Only 11 domains trained vs 33 in the DB.
**Fix:** Add all domains or dynamically pull from: `sqlite3 $DB "SELECT DISTINCT domain FROM facts WHERE domain IS NOT NULL"`

### BUG 4: Hardcoded binary path (Line 12)
```bash
BIN="$PWD/target/release/ollama_train"
```
**Issue:** After symlink to /home/user/cargo-target, binary is at /home/user/cargo-target/release/ollama_train. Old path breaks.
**Fix:** `BIN="${CARGO_TARGET_DIR:-$PWD/target}/release/ollama_train"`

### BUG 5: No Ollama health check
**Issue:** Script starts training immediately without verifying Ollama is running. If Ollama is down, every cycle fails silently (|| true on line 119).
**Fix:** Add pre-flight check: `curl -sf http://localhost:11434/api/tags > /dev/null || { echo "Ollama not running"; exit 1; }`

### BUG 6: State file race condition (Lines 65-76)
**Issue:** `update_state` reads JSON, modifies, writes back. If two processes run simultaneously, last-write-wins and state is lost.
**Fix:** Use file locking: `flock -x "$STATE_FILE.lock" python3 -c ...`

### BUG 7: brain.db query uses key LIKE, not domain column (Line 30)
```bash
fact_count=$(sqlite3 "$DB" "SELECT count(*) FROM facts WHERE key LIKE '${domain}%';" 2>/dev/null || echo "0")
```
**Issue:** Queries by key prefix instead of the domain column. Most keys don't start with domain names. Should use: `SELECT count(*) FROM facts WHERE domain = '${domain}'`

## Recommendations
1. Add `set -e` for fail-fast (currently only `set -u`)
2. Add structured logging (JSON format) for dashboard parsing
3. Add contam_flag exclusion: `WHERE contam_flag = 0`
4. Use quality_score filtering: `WHERE quality_score >= 0.75`
5. Add graceful shutdown on SIGTERM
6. Rotate log files (training logs grow unbounded)
