# brain.db Dedup Audit Results
**Auditor:** Claude-1  
**Date:** 2026-04-16  
**Database:** /home/user/.local/share/plausiden/brain.db  
**Total facts:** 31,720,200  
**Journal mode:** WAL (set by Claude-1 for concurrent access)

---

## Per-Source Duplicate Analysis

| Source | Total | Unique | Duplicates | Dupe Rate |
|--------|------:|-------:|-----------:|----------:|
| cc_news | 500,000 | 474,134 | **25,866** | **5.17%** |
| openwebtext | 5,000,000 | 4,987,274 | 12,726 | 0.25% |
| swag | 73,546 | 68,268 | **5,278** | **7.18%** |
| c4 | 10,000,000 | 9,995,211 | 4,789 | 0.05% |
| wikitext103 | 500,000 | 495,706 | 4,294 | 0.86% |
| amazon_polarity | 3,600,000 | 3,598,952 | 1,048 | 0.03% |
| snli | 549,367 | 548,820 | 547 | 0.10% |
| xnli_ur | 392,702 | 392,296 | 406 | 0.10% |
| sst2 | 67,349 | 66,983 | 366 | 0.54% |
| codesearchnet_python | 100,000 | 99,829 | 171 | 0.17% |
| xnli_zh | 392,702 | 392,557 | 145 | 0.04% |
| codesearchnet_javascript | 100,000 | 99,864 | 136 | 0.14% |
| wikipedia2 | 3,000,000 | 2,999,950 | 50 | 0.00% |
| wikipedia | 2,000,000 | 1,999,990 | 10 | 0.00% |

**Total per-source duplicates: ~57,000 (~0.18% of all facts)**

### Zero-Duplicate Sources (clean)
dbpedia14, codealpaca, gsm8k, gsm8k_test, strategyqa, nq_open, rotten_tomatoes, wikiqa, alpaca, emotion, llm_generated, llm_micro, self_play, ai_extracted

## Key Findings

1. **cc_news is the dirtiest source** — 5.17% duplicate rate. Likely duplicate articles across crawl dates.
2. **swag has the highest dupe rate** (7.18%) despite being small. The commonsense completion format produces many identical short completions.
3. **c4 and openwebtext** have low dupe rates (~0.05% and 0.25%) but high absolute counts due to their size — 4,789 and 12,726 dupes respectively.
4. **Overall duplicate rate is 0.18%** — healthy for a 31.7M fact corpus. No emergency dedup needed.
5. **Cross-source dedup was not feasible** — the full-table DISTINCT query on 16GB timed out. Would require a hash index or sampling approach.

## Recommendations

- **Dedup cc_news** — DELETE the 25,866 dupes. 5% is too high.
- **Dedup swag** — 7.18% is unacceptable. Remove 5,278 dupes.
- **Add a UNIQUE constraint on (source, value)** for future ingestion to prevent dupes at insert time.
- **For cross-source dedup**: create a hash column (`value_hash TEXT`) with `md5(value)` and index it. Then cross-source dedup becomes a GROUP BY on the hash.
