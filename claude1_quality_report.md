# brain.db Quality Report
**Auditor:** Claude-1  
**Date:** 2026-04-16  
**Database:** /home/user/.local/share/plausiden/brain.db  
**Database size:** ~18 GB  
**Journal mode:** WAL (concurrent read/write enabled)

---

## Executive Summary

brain.db is a healthy 40.4M-fact corpus across 58 sources with a low per-source duplicate rate (0.18%), strong multilingual coverage (13 languages), and now 1,010 adversarial examples for PSL axiom calibration. The temporal classification system is in place with indexed queries. Key gaps: adversarial coverage could grow further, and the "general" class dominates at 76% of all facts.

---

## 1. Total Facts

| Metric | Value |
|--------|------:|
| Total facts | 40,371,382 |
| Distinct sources | 58 |
| Avg value length | 381 chars |
| Avg confidence | 0.748 |

---

## 2. Source Distribution (Top 20)

| Source | Count | Avg Length | Confidence |
|--------|------:|-----------:|-----------:|
| c4 | 10,000,000 | 459 | 0.65 |
| openwebtext | 5,000,000 | 500 | 0.70 |
| amazon_polarity | 3,600,000 | 300 | 0.75 |
| wikipedia2 | 3,000,000 | 425 | 0.90 |
| c4_batch2 | 3,000,000 | ~460 | 0.65 |
| wikipedia | 2,000,000 | 431 | 0.90 |
| yahoo_answers | 1,400,000 | 264 | 0.75 |
| yelp_review | 650,000 | 348 | 0.70 |
| dbpedia14 | 560,000 | 288 | 0.85 |
| snli | 549,367 | 129 | 0.90 |
| wikitext103 | 500,000 | 441 | 0.85 |
| cc_news | 500,000 | 419 | 0.70 |
| xnli_* (13 langs) | 5,495,828 | ~190 | 0.85 |
| mnli | 392,702 | 184 | 0.90 |
| pubmedqa | 211,269 | 339 | 0.90 |
| squad2 | 130,319 | 189 | 0.90 |
| ag_news | 120,000 | 252 | 0.80 |
| codesearchnet_* | 348,791 | ~370 | 0.85 |
| adversarial | 1,010 | 116 | 0.69 |

### Long tail (< 100K facts each)
nq_open (87K), swag (73K), sst2 (67K), alpaca (52K), codesearchnet_ruby (49K), tweet_eval_sentiment (46K), hellaswag (40K), cosmosqa (25K), imdb (25K), scitail (24K), codealpaca (20K), piqa (18K), emotion (16K), rotten_tomatoes (8.5K), gsm8k (7.5K), tweet_eval_emotion (3.3K), strategyqa (2K), gsm8k_test (1.3K), wikiqa (1K), llm_generated (1K), llm_micro (455), self_play (16), ai_extracted (3)

---

## 3. Duplicate Analysis

| Source | Total | Duplicates | Rate |
|--------|------:|-----------:|-----:|
| cc_news | 500,000 | 25,866 | **5.17%** |
| swag | 73,546 | 5,278 | **7.18%** |
| openwebtext | 5,000,000 | 12,726 | 0.25% |
| c4 | 10,000,000 | 4,789 | 0.05% |
| wikitext103 | 500,000 | 4,294 | 0.86% |
| amazon_polarity | 3,600,000 | 1,048 | 0.03% |
| **Overall** | **40,371,382** | **~57,000** | **0.18%** |

**Assessment:** Duplicate rate is healthy. cc_news (5.17%) and swag (7.18%) are the only problem sources — recommend dedup pass on these two. All other sources are below 1%.

---

## 4. Temporal Classification

| Class | Count | Percentage | Description |
|-------|------:|-----------:|-------------|
| general | 30,910,633 | 76.6% | Unclassified (c4, openwebtext, amazon, yahoo, etc.) |
| multilingual | 5,495,828 | 13.6% | XNLI across 13 languages |
| stable | 3,060,000 | 7.6% | Wikipedia, dbpedia14, wikitext103 |
| news | 500,000 | 1.2% | CC-News (time-sensitive) |
| code | 368,807 | 0.9% | CodeSearchNet + CodeAlpaca |
| reasoning | 36,115 | 0.1% | GSM8K, StrategyQA, CosmosQA |

**Index:** `idx_facts_temporal` created on `temporal_class` for efficient queries.

**Note:** The "general" class is a catch-all. Future ingestion should sub-classify: c4→"web_crawl", openwebtext→"web_curated", amazon_polarity→"reviews", yahoo_answers→"qa", etc.

---

## 5. Adversarial Coverage

| Category | Count | Examples |
|----------|------:|---------|
| Logical fallacies | ~75 | Non sequitur, ad populum, false dichotomy |
| SQL injections | ~110 | UNION, blind, time-based, WAF bypass |
| XSS payloads | ~60 | Event handlers, SVG, data URI, DOM-based |
| Prompt injections | ~70 | Persona override, encoding bypass, extraction |
| Factual contradictions | ~70 | Science myths, tech misconceptions |
| Social engineering | ~60 | Authority impersonation, phishing pretexts |
| Vulnerable code | ~50 | Hardcoded secrets, eval, shell injection |
| Phishing | ~40 | Bank, cloud, social media, delivery scams |
| Command injection | ~35 | Reverse shells, exfiltration, persistence |
| SSRF/network | ~50 | Cloud metadata, internal services, DNS |
| Path traversal | ~30 | Encoded, null byte, protocol wrappers |
| Auth/crypto/binary | ~90 | JWT, race conditions, format strings |
| Supply chain | ~20 | Typosquat, dependency confusion |
| Edge cases | ~50 | Null bytes, polyglots, decompression bombs |
| Misc (ML, privacy) | ~50 | Adversarial ML, data leakage patterns |
| **Total** | **1,010** | **Target achieved** |

**Method:** 45 parsed from adversarial_data.rs (hardcoded in LFI crate). 965 generated directly by Claude-1 across 5 insertion rounds. Ollama bulk generator (qwen2.5-coder:7b) failed entirely — 0/10 categories succeeded due to 90s timeouts under model contention.

---

## 6. Domain Gap Analysis

### Well-covered domains
- Web text (c4 + openwebtext + wikitext = 15.5M)
- Reviews/sentiment (amazon + yelp + imdb + rotten_tomatoes = 4.3M)
- NLI/reasoning (snli + mnli + xnli + scitail = 6.5M)
- Multilingual (13 languages via XNLI = 5.5M)
- Knowledge (wikipedia + dbpedia = 5.6M)

### Under-represented domains
- **Code:** Only 369K facts (0.9%). Need 2-5M for serious code understanding.
- **Reasoning/math:** Only 36K facts (0.09%). Need 500K+ GSM8K/MATH/ARC.
- **Medical/scientific:** 211K PubMedQA is thin. Need PubMed abstracts, clinical NLP.
- **Legal:** Zero facts. Need legal Q&A, case law, contract analysis.
- **Conversational/dialogue:** Minimal (alpaca 52K, self_play 16). Need dialogue datasets.
- **Adversarial:** 1,010 facts is good for PSL calibration but thin for adversarial training.
- **Structured data:** Zero tabular/CSV/JSON facts. Need structured reasoning examples.

---

## 7. Recommendations for Next 20M Facts

### Priority 1: Code (target: +5M)
- The Stack (deduplicated), GitHub Code, CodeContests, APPS
- Cover: Python, Rust, Go, JavaScript, TypeScript, Java, C/C++
- Include: code+docstring pairs, code review comments, commit messages

### Priority 2: Reasoning/Math (target: +2M)  
- MATH, ARC, MMLU, TruthfulQA, WinoGrande, OpenBookQA
- GSM8K-hard, MATH-hard for advanced reasoning

### Priority 3: Scientific/Medical (target: +3M)
- PubMed Central abstracts, SciQ, SciFact
- BioASQ, MedQA, clinical NLP datasets

### Priority 4: Legal (target: +1M)
- CaseHOLD, LegalBench, CUAD (contract understanding)
- Legal opinions, statutory text

### Priority 5: Conversational (target: +2M)
- ShareGPT, OASST, UltraChat, WildChat
- Multi-turn dialogue with reasoning chains

### Priority 6: Structured/Tabular (target: +1M)
- WikiTableQuestions, SQA, TabFact
- SQL-to-text, chart-to-text

### Priority 7: More adversarial (target: +5K)
- Expand to 5,000+ adversarial for robust PSL training
- Focus: more sophisticated multi-step attacks, real CVE reproductions

### Priority 8: Temporal freshness
- Sub-classify "general" class into finer temporal buckets
- Add refresh schedule for news-class facts (cc_news is a snapshot)

---

## 8. Technical Notes

- **WAL mode** enabled by Claude-1 for concurrent access. Should remain WAL permanently.
- **idx_facts_temporal** index created. Consider adding indexes on `source` and `confidence` for common query patterns.
- **Dedup recommendation:** Add `value_hash TEXT` column with `md5(value)` and unique index for dedup at insert time.
- **Adversarial source attribution:** All adversarial facts use `source='adversarial'`, `confidence=0.5-0.9` (varies by category). Can be identified by key prefix: `adv_rs_*` (from .rs file), `adv_gen_*` (round 1), `adv_r2_*` through `adv_r5_*` (subsequent rounds), `adv_fin_*` (final batch).

---

*Report generated by Claude-1 | 2026-04-16 | Assignment 4 for Claude-0*
