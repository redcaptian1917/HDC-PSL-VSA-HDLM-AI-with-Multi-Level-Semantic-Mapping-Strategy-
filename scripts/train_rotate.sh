#!/usr/bin/env bash
# Rotate through domains, 100 examples each, never stop.
set -u
mkdir -p /var/log/lfi
cd /root/LFI/lfi_vsa_core
BIN="$PWD/target/release/ollama_train"
# #326: env-overridable model + domain list so lessons/start can pass
# model_tier and a subset of domains through.
MODEL="${PLAUSIDEN_MODEL:-qwen2.5-coder:7b}"
if [ -n "${PLAUSIDEN_DOMAINS:-}" ]; then
    IFS=',' read -ra DOMAINS <<< "$PLAUSIDEN_DOMAINS"
else
    DOMAINS=(social math code security philosophy biology chemistry physics language psychology sales)
fi
i=0
while true; do
  d="${DOMAINS[$((i % ${#DOMAINS[@]}))]}"
  echo "[$(date -Iseconds)] cycle=$i domain=$d model=$MODEL starting" | tee -a /var/log/lfi/training.jsonl
  "$BIN" --examples 100 --domain "$d" --model "$MODEL" \
    >> "/var/log/lfi/training-$d.log" 2>&1 || true
  echo "[$(date -Iseconds)] cycle=$i domain=$d done" | tee -a /var/log/lfi/training.jsonl
  i=$((i+1))
  sleep 30
done
