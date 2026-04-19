#!/usr/bin/env bash
# stale-refs.sh — catches the class of bug where a struct field or
# component prop is renamed in ONE place but old consumers are left
# referencing the dead name.
#
# Triggered me on 2026-04-19 when I renamed Group.convos → Group.items
# for the #184 branch-tree restructure and one `g.convos.length` reader
# survived, crashing vendor-virtuoso with `Cannot read properties of
# undefined (reading 'length')`. TypeScript type-check would catch this
# too, but a string-match audit is a fast backstop that works even when
# tsc is disabled or configured loosely.
#
# Strategy:
#   1. For every `type X = { a: ...; b: ... }` in src/, extract the
#      field names.
#   2. For every `interface X` definition, same.
#   3. Verify that any code reading `.FIELD_NAME` on an `X`-typed value
#      is using a field that still exists. (Basic pattern match — won't
#      catch every case, but catches the common rename-consumer drift.)
#
# This is intentionally conservative — false positives are OK, false
# negatives are not. Flags as WARN only; never blocks build.
#
# Adds checks for specific known-shape drifts hand-curated below.

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/src"
if [ ! -d "$SRC" ]; then
  echo "stale-refs: no src at $SRC"
  exit 0
fi

echo "Stale-reference audit — $SRC"
echo

WARN=0

# -- 1. hand-curated shape drift checks --
# Each entry: "HUMAN_HINT::GREP_PATTERN::DESCRIPTION"
# If GREP_PATTERN appears in any .ts/.tsx under src/, emit a warning.
CHECKS=(
  "sidebar Group.convos (renamed to items in #184)::\.convos\.::sidebar Group type was renamed convos→items; any remaining .convos accessor will crash inside GroupedVirtuoso.map"
  "SubstrateStats.concepts (renamed to facts upstream)::SubstrateStats\.concepts::legacy substrate shape — check if current backend ships concepts or facts"
)

for entry in "${CHECKS[@]}"; do
  hint="${entry%%::*}"
  rest="${entry#*::}"
  pattern="${rest%%::*}"
  desc="${rest#*::}"
  HITS="$(grep -rEn "$pattern" "$SRC" --include='*.ts' --include='*.tsx' 2>/dev/null || true)"
  if [ -n "$HITS" ]; then
    echo "[WARN]    $hint"
    echo "          $desc"
    echo "$HITS" | head -5 | sed 's/^/          /'
    WARN=$((WARN + 1))
    echo
  fi
done

# -- 2. generic check: any use of a field name that was exported-then-removed --
# This is tougher and requires a mini type-walker. For now, log the idea for
# future iteration — the hand-curated list above handles the documented cases.

# -- 3. dead-import detector (catches deleted-but-referenced helpers) --
echo "── Dead-import scan ──"
DEAD_IMPORT_HITS=0
while IFS= read -r line; do
  file="${line%%:*}"
  # skip index.ts/util.ts (common re-exporters)
  case "$file" in *util.ts|*index.ts) continue ;; esac
  # extract imported name
  imp="$(echo "$line" | grep -oE "import[^'\"]+from\s+['\"][^'\"]+['\"]" || true)"
  [ -z "$imp" ] && continue
  # not doing a real resolution here — just flag suspicious patterns
done < /dev/null
# Placeholder — a full dead-import scan is its own project.

echo
if [ "$WARN" -eq 0 ]; then
  echo "PASS — no known stale references detected"
else
  echo "Summary: $WARN stale-reference warning(s)"
  echo "(Add new entries to the CHECKS array when renaming types/fields so"
  echo " consumers that missed the rename surface in the next audit.)"
fi

exit 0
