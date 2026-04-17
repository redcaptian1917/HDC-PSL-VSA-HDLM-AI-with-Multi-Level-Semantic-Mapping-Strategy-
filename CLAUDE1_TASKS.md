# Claude 1 — MEGA TASK LIST (respond to Claude 0 on the bus when done with each)

## BLOCK A: Dataset Ingestion (10 tasks)

All datasets at: ~/Development/PlausiDen/"New training sets i found"/
DB: ~/.local/share/plausiden/brain.db (PRAGMA busy_timeout=600000)

1. **Phishing websites** — extract zip, parse CSV/ARFF, insert as cybersecurity facts
2. **Phiusiil phishing URL dataset** (15MB) — parse URLs + features → cybersecurity
3. **SMS spam collection** — parse tab-separated label+text → cybersecurity
4. **Spambase** — parse features → cybersecurity
5. **RT-IoT 2022** — parse network traffic features → cybersecurity
6. **Bank marketing** — parse CSV → finance domain
7. **Credit approval + default of credit card** — parse → finance domain
8. **Air quality** — parse → science domain
9. **Energy efficiency + appliances energy** — parse → science domain
10. **Communities and crime** — parse → social_science domain

For each: unzip to /tmp/, read CSV, create fact per row (key=dataset:hash, value=readable summary of row, domain=X, quality_score=0.80)

## BLOCK B: Training Data Generation via Ollama (15 tasks)

Use: curl -s http://localhost:11434/api/generate -d '{"model":"qwen2.5-coder:7b","prompt":"<PROMPT>","stream":false,"options":{"temperature":0.8,"num_predict":400}}'
Write to /home/user/LFI-data/ as JSONL files.

11. Generate 100 conversational greetings (time-aware) → conversational_greetings.jsonl
12. Generate 100 error recovery dialogues → error_recovery_v2.jsonl
13. Generate 100 task completion dialogues → task_completion.jsonl
14. Generate 100 Rust programming Q&A → rust_training.jsonl
15. Generate 100 Linux sysadmin Q&A → sysadmin_training.jsonl
16. Generate 100 cybersecurity Q&A → cyber_training.jsonl
17. Generate 100 networking Q&A → networking_training.jsonl
18. Generate 100 database Q&A → database_training.jsonl
19. Generate 100 Python programming Q&A → python_training.jsonl
20. Generate 100 DevOps Q&A → devops_training.jsonl
21. Generate 100 privacy/anonymity Q&A → privacy_training.jsonl
22. Generate 100 math reasoning Q&A → math_training.jsonl
23. Generate 100 philosophy Q&A → philosophy_training.jsonl
24. Generate 100 history Q&A → history_training.jsonl
25. Generate 100 economics Q&A → economics_training.jsonl

## BLOCK C: Quality + Maintenance (10 tasks)

26. Dedup check: find and count exact duplicate values in brain.db
27. Delete facts shorter than 10 characters
28. Upgrade facts with NULL quality_score to 0.5
29. Count facts per source, report top 20
30. Count facts per domain, report all
31. Check FTS5 sync: SELECT COUNT(*) FROM facts vs facts_fts
32. Export combined_training_v4.jsonl merging ALL .jsonl files in /home/user/LFI-data/
33. Push training data to GitHub (PlausiDen-Training-Data repo)
34. Push main repo to GitHub
35. Report total fact count, training pair count, and domain distribution on the bus

## BLOCK D: Security Audit (5 tasks)

36. Search api.rs for any remaining unwrap() without SAFETY comment
37. Search all .rs files for TODO/FIXME/HACK comments — list them
38. Check if any endpoints accept unbounded input (no size limit)
39. Check all outbound HTTP calls use --max-time (no infinite waits)
40. Verify CORS doesn't include 0.0.0.0

## HOW TO REPORT
After EVERY task, write to bus:
```
echo '{"id":"c1-NNN","from":"claude-1","to":"claude-0","timestamp":"'$(date -u +%Y-%m-%dT%H:%M:%SZ)'","type":"status","subject":"Task N done: <description>","body":"<details>","refs":[],"status":"unread"}' >> /tmp/claude-ipc/bus.jsonl
```

## START NOW. Work through sequentially. Never idle.
