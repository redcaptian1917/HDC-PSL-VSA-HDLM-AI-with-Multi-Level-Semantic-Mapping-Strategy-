#!/usr/bin/env bash
# Gemini-CLI LFI Trainer Bridge (#405)
#
# Drives a continuous Q&A loop between Gemini CLI and LFI, rating + correcting
# LFI's replies and feeding the corrections back into the brain.db via
# POST /api/trainer/turn. The Classroom tab aggregates these sessions.
#
# Usage:
#   ./gemini-trainer.sh                               # 100 rounds by default
#   ./gemini-trainer.sh 500 "cybersecurity"           # 500 rounds, seeded topic
#   GEMINI_MODEL=gemini-2.0-flash ./gemini-trainer.sh # override model
#
# Requires: gemini CLI installed + authenticated, jq, curl, websocat.

set -euo pipefail

ROUNDS="${1:-100}"
SEED_TOPIC="${2:-computer science fundamentals}"
LFI_HOST="${LFI_HOST:-127.0.0.1}"
LFI_PORT="${LFI_PORT:-3000}"
SESSION_ID="${SESSION_ID:-$(date +%s)}"
GEMINI_MODEL="${GEMINI_MODEL:-gemini-2.0-flash}"

TRAINER_URL="http://${LFI_HOST}:${LFI_PORT}/api/trainer/turn"
CHAT_URL="ws://${LFI_HOST}:${LFI_PORT}/ws/chat"

# Quick preflight — refuse to start if the backend is not up.
if ! curl -sf "http://${LFI_HOST}:${LFI_PORT}/api/health" > /dev/null; then
  echo "FATAL: LFI backend not reachable at http://${LFI_HOST}:${LFI_PORT}" >&2
  exit 1
fi
if ! command -v gemini >/dev/null; then
  echo "FATAL: gemini CLI not on PATH. Install + authenticate first." >&2
  exit 1
fi
if ! command -v jq >/dev/null || ! command -v websocat >/dev/null; then
  echo "FATAL: need jq and websocat. apt install jq && cargo install websocat" >&2
  exit 1
fi

echo "trainer: gemini_cli  session: ${SESSION_ID}  rounds: ${ROUNDS}  topic: ${SEED_TOPIC}"
echo "watch the Classroom tab → Trainer Sessions to see live data"
echo

for i in $(seq 1 "$ROUNDS"); do
  # 1. Ask Gemini to generate a question about the seed topic.
  Q=$(gemini prompt -m "$GEMINI_MODEL" \
    "Ask one focused question about ${SEED_TOPIC} that tests a specific fact. \
     Output ONLY the question, no preamble." 2>/dev/null | head -c 2000 | tr -d '\n')
  if [[ -z "$Q" ]]; then
    echo "[${i}] gemini returned empty — skipping"; continue
  fi

  # 2. Send the question to LFI over the chat WS.
  REPLY=$(echo "{\"content\":\"$(echo "$Q" | sed 's/"/\\"/g')\"}" \
    | websocat -B 200000 "$CHAT_URL" \
    | while read -r line; do
        t=$(echo "$line" | jq -r '.type // empty' 2>/dev/null)
        if [[ "$t" == "chat_response" ]]; then
          echo "$line" | jq -r '.content // empty'
          break
        fi
      done | head -c 4000)

  if [[ -z "$REPLY" ]]; then
    echo "[${i}] LFI returned empty — skipping"; continue
  fi

  # 3. Ask Gemini to rate + correct LFI's reply.
  JUDGE=$(gemini prompt -m "$GEMINI_MODEL" \
    "You are judging an answer for factual accuracy. \
     QUESTION: ${Q} \
     ANSWER: ${REPLY} \
     \
     Output ONE LINE of JSON: \
     {\"rating\": \"up\"|\"down\"|\"correct\", \"correction\": \"...\"|null} \
     - up: answer is correct \
     - down: answer is wrong; set correction to the right answer \
     - correct: answer is partially right; set correction to the full right answer \
     Output ONLY the JSON, no preamble." 2>/dev/null | tail -c 2000)

  RATING=$(echo "$JUDGE" | jq -r '.rating // "up"' 2>/dev/null)
  CORRECTION=$(echo "$JUDGE" | jq -r '.correction // ""' 2>/dev/null)

  # 4. POST to /api/trainer/turn.
  RES=$(curl -s -X POST "$TRAINER_URL" \
    -H 'content-type: application/json' \
    -d "$(jq -cn \
      --arg trainer "gemini_cli" \
      --arg session "$SESSION_ID" \
      --arg q "$Q" --arg r "$REPLY" \
      --arg rating "$RATING" --arg correction "$CORRECTION" \
      '{trainer:$trainer, session_id:$session, user_query:$q,
        lfi_reply:$r, rating:$rating,
        correction: (if $correction == "" then null else $correction end)}')")

  OK=$(echo "$RES" | jq -r '.ok // false')
  ACT=$(echo "$RES" | jq -r '.actions_applied // 0')
  printf "[%3d/%d] %s rating=%-7s actions=%s\n" "$i" "$ROUNDS" \
    "$(if [[ "$OK" == "true" ]]; then echo ok; else echo FAIL; fi)" "$RATING" "$ACT"

  # Small backoff so we don't smash the trainer rate limit (60/60s).
  sleep 1
done

echo
echo "done — see /api/trainer/sessions or Classroom → Trainer Sessions for the rollup"
