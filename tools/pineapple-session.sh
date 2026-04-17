#!/bin/bash
# PlausiDen Pineapple Session Orchestrator
# Wires: pineapple-harden → pineapple-capture → lfi-ingest-pcap
# Usage: ./pineapple-session.sh <tier> [duration_seconds]

set -euo pipefail

TIER=${1:?Usage: pineapple-session.sh <tier 1|2|3> [duration_seconds]}
DURATION=${2:-300}
SESSION_ID=$(uuidgen)

echo "=== PlausiDen Adversary Simulation Session ==="
echo "Session:  $SESSION_ID"
echo "Tier:     $TIER"
echo "Duration: ${DURATION}s"
echo ""

# Step 1: Generate adversary identity
echo "--- Step 1: Generating adversary identity ---"
pineapple-harden --tier "$TIER" --session-id "$SESSION_ID" --apply
echo ""

# Step 2: Capture frames
echo "--- Step 2: Capturing WiFi frames ---"
pineapple-capture --session-id "$SESSION_ID" --duration-seconds "$DURATION" --radio wlan1mon
echo ""

# Step 3: Ingest into brain.db
echo "--- Step 3: Ingesting frames into LFI ---"
lfi-ingest-pcap --session-id "$SESSION_ID"
echo ""

echo "=== Session $SESSION_ID complete ==="
echo "Files at: ~/lfi/sessions/$SESSION_ID/"
ls -la ~/lfi/sessions/"$SESSION_ID"/
