#!/bin/bash
# Magpie Synthetic Data Generation
# Feed chat template header → model hallucinates user query → feed back for response
# Produces instruction+response pairs for ORPO/GRPO training
#
# Usage: bash magpie_generate.sh [count] [output_file]
# Requires: Ollama running with qwen2.5-coder:7b

set -euo pipefail

COUNT=${1:-1000}
OUTPUT=${2:-/root/LFI/lfi_vsa_core/magpie_pairs.jsonl}
MODEL="qwen2.5-coder:7b"
OLLAMA="http://localhost:11434/api/generate"

echo "=== Magpie Data Generation ==="
echo "Model: $MODEL"
echo "Target: $COUNT pairs"
echo "Output: $OUTPUT"

> "$OUTPUT"  # Clear output

for i in $(seq 1 $COUNT); do
    # Step 1: Generate a user query by feeding just the chat header
    QUERY=$(curl -s --max-time 30 -X POST "$OLLAMA" \
        -H 'Content-Type: application/json' \
        -d "{\"model\":\"$MODEL\",\"prompt\":\"<|im_start|>user\\n\",\"stream\":false,\"options\":{\"temperature\":1.0,\"num_predict\":100,\"stop\":[\"<|im_end|>\"]}}" \
        2>/dev/null | python3 -c "import json,sys; print(json.load(sys.stdin).get('response','').strip())" 2>/dev/null)

    if [ -z "$QUERY" ] || [ ${#QUERY} -lt 10 ]; then
        continue
    fi

    # Step 2: Get response to the generated query
    RESPONSE=$(curl -s --max-time 60 -X POST "$OLLAMA" \
        -H 'Content-Type: application/json' \
        -d "{\"model\":\"$MODEL\",\"prompt\":\"Answer helpfully and thoroughly: $QUERY\",\"stream\":false,\"options\":{\"temperature\":0.4,\"num_predict\":500}}" \
        2>/dev/null | python3 -c "import json,sys; print(json.load(sys.stdin).get('response','').strip())" 2>/dev/null)

    if [ -z "$RESPONSE" ] || [ ${#RESPONSE} -lt 20 ]; then
        continue
    fi

    # Step 3: Write pair
    python3 -c "
import json
pair = {'instruction': '''$QUERY''', 'input': '', 'output': '''$RESPONSE''', 'source': 'magpie'}
print(json.dumps(pair))
" >> "$OUTPUT" 2>/dev/null

    if [ $((i % 10)) -eq 0 ]; then
        echo "  $i/$COUNT pairs generated ($(wc -l < "$OUTPUT") valid)"
    fi
done

echo "=== Done: $(wc -l < "$OUTPUT") pairs in $OUTPUT ==="
