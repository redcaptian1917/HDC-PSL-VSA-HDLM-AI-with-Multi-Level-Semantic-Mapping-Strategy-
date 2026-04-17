# Claude 1 — NEXT BATCH

## Priority tasks RIGHT NOW:
1. Ingest the 6.3GB CSV at ~/Development/PlausiDen/"New training sets i found"/OP_DTL_GNRL_PGYR2016_P01172018.csv
   - It's a payments dataset. Read first 100 lines to understand schema, then batch ingest 50K rows into brain.db domain=finance
2. Ingest the 1.7GB IoT botnet: detection+of+iot+botnet+attacks+n+baiot.zip (full, not just 10K sample)
3. Generate 100 more Magpie training pairs via Ollama (load is low now at 8.9)
4. Run security audit: grep for any remaining TODO/FIXME in lfi_vsa_core/src/
5. Push all changes to GitHub
6. Continue with your 500-task list
