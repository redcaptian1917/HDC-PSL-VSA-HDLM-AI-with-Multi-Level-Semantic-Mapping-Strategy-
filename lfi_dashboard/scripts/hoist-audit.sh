#!/usr/bin/env bash
# hoist-audit.sh — catches the TDZ class of bug where a useEffect (or
# useMemo / useCallback) references a useState/useRef/useMemo that is
# declared LATER in the same function.
#
# Reason for existence: we've hit this bug THREE times now:
#   1. currentConversationIdRef (minified 'ke') crashed prod Apr 18.
#   2. settingsRef + conversationsRef (minified 'ge') crashed prod.
#   3. per-view tour activeView ref (2026-04-19, 'ue'): the new
#      autolaunch effect was inserted at line ~894 but activeView's
#      useState lives at line ~1451. React reads the deps array during
#      render, evaluating activeView before it's initialized.
#
# Why type-check didn't catch it: TypeScript doesn't model the order of
# statement evaluation inside a function body for `let` bindings — it
# treats them as hoisted. Runtime throws a TDZ; tsc passes clean.
#
# Detection strategy (grep-based, conservative — false positives OK):
#   For each .tsx file:
#     1. Find every line that contains `useEffect(` or `useMemo(` with
#        a deps array.
#     2. Extract the identifier names from `[dep1, dep2, ...]`.
#     3. For each identifier, find the line of its `const [NAME`, or
#        `const NAME = useRef`, or `const NAME = useMemo` declaration.
#     4. If the declaration line > the useEffect line → FAIL.
#
# This is a text audit — it'll miss some cases (nested scopes, rename
# through destructure) but catches the common pattern. The correctness
# goal is: no regression should ship without the audit flagging it.
#
# Exit 0 if clean, 1 if bugs found. CI wires the exit code into build.

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/src"
if [ ! -d "$SRC" ]; then
  echo "hoist-audit: no src at $SRC"
  exit 0
fi

echo "Hoist-TDZ audit — $SRC"
echo

FAIL=0

for f in $(find "$SRC" -name '*.tsx' -o -name '*.ts'); do
  # Skip test / stub files.
  case "$(basename "$f")" in *.test.*|*.spec.*) continue ;; esac

  # Pass 1: build a map of "identifier → declaration-line" for the file.
  # Simple regex: `const [NAME` or `const [NAME,` or `const NAME =
  # useState|useRef|useMemo|useCallback`.
  DECLS="$(mktemp)"
  awk '
    # const [name, ...] = useState(...)  or  const [name] = useState(...)
    /^[[:space:]]*const[[:space:]]+\[[[:space:]]*[A-Za-z_][A-Za-z0-9_]*/ {
      match($0, /const[[:space:]]+\[[[:space:]]*([A-Za-z_][A-Za-z0-9_]*)/, a);
      if (a[1] != "") print a[1] ":" NR;
      next;
    }
    # const name = useRef(...)  or  = useMemo(...)  or  = useCallback(...)
    /^[[:space:]]*const[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*=[[:space:]]*use(Ref|Memo|Callback)/ {
      match($0, /const[[:space:]]+([A-Za-z_][A-Za-z0-9_]*)/, a);
      if (a[1] != "") print a[1] ":" NR;
      next;
    }
  ' "$f" > "$DECLS"

  # Pass 2: find every useEffect / useMemo / useCallback with a deps array
  # and check that every dep name is declared before the call line.
  awk -v f="$f" -v decls="$DECLS" '
    BEGIN {
      # load decls map: NAME → LINE (first decl wins if duplicates)
      while ((getline line < decls) > 0) {
        split(line, p, ":");
        if (!(p[1] in lineOf)) lineOf[p[1]] = p[2] + 0;
      }
    }
    # Match: }, [dep1, dep2, ...])  — typical closing of useEffect.
    # Capture the content between [ and ].
    /^[[:space:]]*\},[[:space:]]*\[[^\]]*\]\)/ {
      useLine = NR;
      # extract the [...] payload
      match($0, /\[[^\]]*\]/);
      raw = substr($0, RSTART + 1, RLENGTH - 2);
      # split by comma
      n = split(raw, deps, ",");
      for (i = 1; i <= n; i++) {
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", deps[i]);
        # strip property access (e.g. foo.bar → foo)
        sub(/\..*/, "", deps[i]);
        sub(/\[.*/, "", deps[i]);
        name = deps[i];
        if (name == "" || name ~ /^[0-9"]/) continue;
        if (name in lineOf) {
          if (lineOf[name] > useLine) {
            print f ":" useLine ": uses \"" name "\" declared later at line " lineOf[name];
            failures++;
          }
        }
      }
    }
    END { exit (failures > 0 ? 1 : 0) }
  ' "$f"
  rc=$?
  [ "$rc" -ne 0 ] && FAIL=1
  rm -f "$DECLS"
done

echo
if [ "$FAIL" -eq 0 ]; then
  echo "PASS — no hoist-TDZ patterns detected"
else
  echo "FAIL — fix the hoist-TDZ reports above."
  echo "(Move the useEffect / useMemo / useCallback below the declarations it"
  echo " lists in its deps array. React reads deps during render, not at mount,"
  echo " so a 'let' declared further down in the same function body throws TDZ.)"
fi

exit $FAIL
