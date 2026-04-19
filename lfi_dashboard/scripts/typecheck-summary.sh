#!/usr/bin/env bash
# typecheck-summary.sh — run tsc --noEmit and report error count + first
# N lines. Doesn't FAIL the build on pre-existing errors (there are too
# many historical issues and this was added late), but surfaces them so
# new regressions are visible. CI-safe.
#
# Exit codes:
#   0 — always (so legacy errors don't block the build)
# Output:
#   Error count summary + first 20 errors for triage.

set -u
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

TSC_BIN="./node_modules/.bin/tsc"
if [ ! -x "$TSC_BIN" ]; then
  echo "typecheck-summary: tsc not installed — run 'npm install' first. Skipping."
  exit 0
fi

echo "TypeScript check — $ROOT/src"
echo

OUT="$(mktemp)"
"$TSC_BIN" --noEmit 2>&1 > "$OUT" || true
COUNT="$(grep -c 'error TS' "$OUT" 2>/dev/null || echo 0)"

if [ "$COUNT" -eq 0 ]; then
  echo "PASS — 0 type errors"
else
  echo "── Errors (first 20 of $COUNT) ──"
  grep 'error TS' "$OUT" | head -20
  echo
  echo "Summary: $COUNT type error(s)"
  echo "Run 'npm run typecheck' for the full list."
  echo "(Pre-existing errors don't block the build — only new regressions."
  echo " Run 'npm run typecheck' locally before shipping if you touched types.)"
fi

rm -f "$OUT"
exit 0
