# CLAUDE 1 -- 500 TASKS (Backend / Data Engineer)

Generated: 2026-04-17
Context: brain.db has 58.8M facts, 360 sources, 40 domains. Ollama at localhost:11434. Training data at /home/user/LFI-data/. New datasets at ~/Development/PlausiDen/"New training sets i found"/. Codebase at /root/LFI/lfi_vsa_core/.

---

## CATEGORY 1: DATA INGESTION (Tasks 1-85)

### Unzip and parse the "New training sets i found" directory (1-30)

1. Write a master inventory script that lists every file in ~/Development/PlausiDen/"New training sets i found"/, recording filename, size, extension, and whether it has been ingested yet. Output to /home/user/LFI-data/ingestion_inventory.json
2. Unzip adult.zip from "New training sets i found" and parse the CSV inside into brain.db facts under domain "demographics" with source "uci-adult-dataset"
3. Unzip bank+marketing.zip and ingest the bank-full.csv into brain.db under domain "finance" -- each row becomes a fact about marketing campaign outcomes
4. Parse BNG(spambase).arff (344MB) -- write an ARFF parser in Python that extracts attribute names and summary statistics, ingest top-level facts about spam classification features into domain "cybersecurity"
5. Unzip census+income.zip and census+income+kdd.zip, parse all CSV files, ingest demographic/income facts into domain "economics"
6. Parse dataset.arff (65MB), dataset (1).arff (2.3MB), dataset (2).arff (78MB) -- identify what datasets these are from ARFF headers, ingest metadata and summary statistics as facts
7. Unzip communities+and+crime.zip and communities+and+crime+unnormalized.zip, parse CSVs, ingest crime statistics facts into domain "criminology"
8. Parse all_ai_models.csv (5.7MB) from "New training sets i found" and ingest each AI model as a fact with attributes (name, type, parameters, release date) into domain "artificial_intelligence"
9. Unzip default+of+credit+card+clients.zip, parse the Excel/CSV, ingest credit default statistics into domain "finance"
10. Unzip compas-analysis-master.zip, parse the COMPAS recidivism data CSVs, ingest criminal justice algorithm analysis facts into domain "criminal_justice"
11. Unzip russian-troll-tweets-master.zip, parse CSV files of Russian troll farm tweets, ingest as disinformation analysis facts into domain "information_warfare"
12. Parse 20170816_Documenting_Hate.csv and ingest hate incident data into domain "civil_rights"
13. Unzip phishing+websites.zip and phiusiil+phishing+url+dataset.zip, extract phishing URL features, ingest as cybersecurity facts into domain "cybersecurity"
14. Unzip kdd+cup+1999+data.zip, parse the intrusion detection dataset, ingest network attack classification facts into domain "cybersecurity"
15. Unzip rt-iot2022.zip, parse IoT network traffic data, ingest IoT security facts into domain "iot_security"
16. Unzip detection+of+iot+botnet+attacks+n+baiot.zip (1.7GB), sample and parse botnet attack signatures, ingest into domain "cybersecurity"
17. Unzip internet+firewall+data.zip, parse firewall log data, ingest network security rule facts into domain "network_security"
18. Unzip sms+spam+collection.zip, parse SMS text classification data, ingest NLP spam-detection facts into domain "nlp"
19. Unzip human+activity+recognition+using+smartphones.zip, parse accelerometer/gyroscope features, ingest HAR classification facts into domain "sensor_data"
20. Unzip drug+consumption+quantified.zip, parse drug use patterns data, ingest public health facts into domain "public_health"
21. Unzip student+performance.zip and predict+students+dropout+and+academic+success.zip, parse education outcome data, ingest into domain "education"
22. Unzip online+retail.zip, parse e-commerce transaction data, ingest retail analytics facts into domain "business"
23. Unzip energy+efficiency.zip, individual+household+electric+power+consumption.zip, and appliances+energy+prediction.zip, ingest energy consumption facts into domain "energy"
24. Parse file1c556677f875.arff and file639340bd9ca9.arff -- identify datasets from headers, ingest metadata into appropriate domains
25. Unzip OmniMath/ directory contents, parse math problem datasets, ingest into domain "mathematics"
26. Parse TESSY-Math-12K/ directory contents, ingest 12K math problems as Q&A training facts into domain "mathematics"
27. Parse toxic_conversations/ directory, ingest toxicity classification data into domain "content_moderation"
28. Unzip and parse every file in uci-batch/ directory (bulk UCI ML datasets), ingest metadata and sample facts per dataset
29. Unzip MathQA.zip (7.3MB), parse mathematical question-answer pairs, ingest into domain "mathematics"
30. Unzip spambase.zip, parse spam features dataset, ingest email classification facts into domain "cybersecurity"

### Download and ingest from HuggingFace (31-50)

31. Download HuggingFace dataset "tatsu-lab/alpaca" and convert to JSONL training format at /home/user/LFI-data/hf_alpaca.jsonl
32. Download "TIGER-Lab/MathInstruct" from HuggingFace, parse math instruction pairs, ingest into brain.db domain "mathematics"
33. Download "EleutherAI/pile" (sample, not full), extract and ingest domain-diverse text facts across multiple domains
34. Download "bigcode/starcoderdata" (Rust subset only), ingest Rust programming knowledge into domain "programming"
35. Download "lmsys/chatbot_arena_conversations" and ingest conversation quality ranking data for training calibration
36. Download "HuggingFaceH4/ultrafeedback_binarized" for preference training data, convert to /home/user/LFI-data/hf_ultrafeedback.jsonl
37. Download "allenai/tulu-v2-sft-mixture" and ingest diverse instruction-following examples
38. Download "garage-bAInd/Open-Platypus" for STEM reasoning data, ingest into domain "stem"
39. Download "teknium/OpenHermes-2.5" and convert to JSONL at /home/user/LFI-data/hf_openhermes.jsonl
40. Download "BAAI/Infinity-Instruct" (sample) for multilingual instruction data, ingest English subset
41. Download "argilla/distilabel-capybara-dpo-7k-binarized" for DPO training pairs
42. Download "m-a-p/CodeFeedback-Filtered-Instruction" for code instruction data, ingest programming facts
43. Download "jondurbin/airoboros-3.1" dataset, convert to JSONL for fine-tuning at /home/user/LFI-data/hf_airoboros.jsonl
44. Download "WizardLM/WizardLM_evol_instruct_V2_196k" for evolved instruction data
45. Download "THUDM/LongBench" dataset for long-context evaluation pairs, store at /home/user/LFI-data/hf_longbench.jsonl
46. Download "gsm8k" from HuggingFace, parse grade-school math problems, ingest into domain "mathematics" in brain.db
47. Download "cais/mmlu" for multi-domain evaluation, ingest Q&A pairs across all 57 MMLU subjects into brain.db
48. Download "lukaemon/bbh" (Big Bench Hard) for advanced reasoning benchmarks, store at /home/user/LFI-data/hf_bbh.jsonl
49. Download "deepmind/math_dataset" for procedurally generated math, ingest into domain "mathematics"
50. Download "nvidia/HelpSteer2" for helpfulness/safety rating data, store at /home/user/LFI-data/hf_helpsteer2.jsonl

### Parse and convert existing HF downloads (51-65)

51. Parse /home/user/LFI-data/hf-conversations/alpaca_gpt4.parquet and convert to brain.db-compatible JSONL
52. Parse /home/user/LFI-data/hf-conversations/camel_biology.parquet, camel_chemistry.parquet, camel_philosophy.parquet, camel_physics.parquet -- ingest all four CAMEL domains into brain.db
53. Parse /home/user/LFI-data/hf-conversations/capybara_train.parquet and ingest multi-turn conversations into training data
54. Parse /home/user/LFI-data/hf-conversations/code_alpaca_20k.json and ingest programming instruction pairs into domain "programming"
55. Parse /home/user/LFI-data/hf-conversations/cot_collection_0.parquet (chain-of-thought) and ingest reasoning traces into domain "reasoning"
56. Parse /home/user/LFI-data/hf-conversations/sharegpt_v3.json and ingest human-AI conversation pairs
57. Parse /home/user/LFI-data/hf-conversations/wizardlm_143k.json and wizardlm_70k.json, deduplicate, ingest into training pipeline
58. Parse /home/user/LFI-data/hf-conversations/slimorca_train.parquet and ingest Orca-style reasoning chains
59. Parse /home/user/LFI-data/hf-conversations/ultrachat_200k_shard0.parquet and ingest multi-topic conversation data
60. Parse /home/user/LFI-data/hf-conversations/ultrafeedback.parquet and ingest preference-ranked responses
61. Parse /home/user/LFI-data/hf-conversations/tulu3_shard0.parquet and tulu_v2_part0.parquet, tulu_v2_part1.parquet -- ingest all TULU variants
62. Parse /home/user/LFI-data/hf-conversations/platypus.parquet (STEM reasoning) and ingest into domain "stem"
63. Parse /home/user/LFI-data/hf-conversations/scienceqa.parquet and ingest science Q&A with explanations into domain "science"
64. Parse /home/user/LFI-data/hf-conversations/sql_create_context.parquet and ingest SQL training data into domain "databases"
65. Parse /home/user/LFI-data/hf-conversations/python_code_18k.parquet and ingest into domain "programming"

### Specialized ingestion tasks (66-85)

66. Parse /home/user/LFI-data/hf-conversations/baize_medical.json and ingest medical dialogue data into domain "medicine"
67. Parse /home/user/LFI-data/hf-conversations/baize_quora.json and ingest general knowledge Q&A pairs
68. Parse /home/user/LFI-data/hf-conversations/baize_stackoverflow.json and ingest programming Q&A into domain "programming"
69. Parse /home/user/LFI-data/hf-conversations/pku_saferlhf_0.parquet and ingest safety-aligned training data into domain "safety"
70. Parse /home/user/LFI-data/hf-conversations/toxic_chat_test.csv and ingest toxicity detection training data
71. Parse /home/user/LFI-data/hf-conversations/wildchat_shard0.parquet and ingest real user conversation data
72. Parse /home/user/LFI-data/hf-conversations/pippa_roleplay.parquet and ingest character/roleplay conversation data
73. Parse /home/user/LFI-data/hf-conversations/chatbot_arena.parquet and ingest comparative response quality data
74. Write a universal ARFF-to-JSONL converter script at /root/LFI/scripts/arff_to_jsonl.py that handles all ARFF files in "New training sets i found"
75. Write a universal ZIP-explorer script at /root/LFI/scripts/explore_zips.py that catalogs contents of all 260 zip files without fully extracting them
76. Unzip and parse camera_tickets_20210701.zip (1.2GB), extract traffic violation statistics, ingest into domain "law_enforcement"
77. Unzip WHC-visitor-logs-calendars.zip, parse White House visitor log data, ingest into domain "politics"
78. Unzip police-settlements-main.zip, parse police settlement data, ingest into domain "law_enforcement"
79. Unzip redistricting-atlas-data-master.zip, parse redistricting data, ingest into domain "politics"
80. Unzip uber-tlc-foil-response-master.zip, parse Uber trip data, ingest transportation statistics into domain "urban_planning"
81. Parse the CCSO_ITU_FOIA_Dumke_06112018_DRAFT_v2.0_Age_questions.xlsx file, ingest corrections data into domain "criminal_justice"
82. Unzip bitcoinheistransomwareaddressdataset.zip (116MB), parse ransomware bitcoin address data, ingest into domain "cybercrime"
83. Unzip cvelistV5/ directory at /home/user/LFI-data/cvelistV5, parse CVE JSON files, ingest vulnerability facts into domain "cybersecurity" (use /root/LFI/scripts/parse_cve_v5.py as base)
84. Build an incremental ingestion tracker in brain.db -- add table `ingestion_log` with columns (file_path, file_hash, ingested_at, row_count, status) so no file gets ingested twice
85. Write a batch ingestion orchestrator at /root/LFI/scripts/batch_ingest.py that reads ingestion_inventory.json and processes all un-ingested files sequentially with checkpointing

---

## CATEGORY 2: TRAINING DATA GENERATION (Tasks 86-185)

### Domain gap Q&A generation via Ollama (86-120)

86. Query Ollama (localhost:11434, model qwen2.5:7b) to generate 500 Q&A pairs on macroeconomics (GDP, inflation, monetary policy, trade deficits), save to /home/user/LFI-data/gen_economics_macro.jsonl
87. Generate 500 Q&A pairs on microeconomics (supply/demand, elasticity, market structures, game theory), save to /home/user/LFI-data/gen_economics_micro.jsonl
88. Generate 500 Q&A pairs on constitutional law (amendments, Supreme Court cases, judicial review, due process), save to /home/user/LFI-data/gen_legal_constitutional.jsonl
89. Generate 500 Q&A pairs on contract law (offer/acceptance, consideration, breach remedies, UCC), save to /home/user/LFI-data/gen_legal_contracts.jsonl
90. Generate 500 Q&A pairs on criminal law (mens rea, felony classes, defenses, sentencing), save to /home/user/LFI-data/gen_legal_criminal.jsonl
91. Generate 500 Q&A pairs on international law (treaties, ICC, law of the sea, sovereignty), save to /home/user/LFI-data/gen_legal_international.jsonl
92. Generate 500 Q&A pairs on political philosophy (Locke, Hobbes, Rawls, Nozick, Marx, utilitarianism), save to /home/user/LFI-data/gen_philosophy_political.jsonl
93. Generate 500 Q&A pairs on epistemology (knowledge, justification, skepticism, Gettier problems), save to /home/user/LFI-data/gen_philosophy_epistemology.jsonl
94. Generate 500 Q&A pairs on ethics (deontology, consequentialism, virtue ethics, trolley problems), save to /home/user/LFI-data/gen_philosophy_ethics.jsonl
95. Generate 500 Q&A pairs on logic and critical thinking (fallacies, syllogisms, propositional logic, set theory), save to /home/user/LFI-data/gen_philosophy_logic.jsonl
96. Generate 500 Q&A pairs on ancient history (Mesopotamia, Egypt, Greece, Rome, China), save to /home/user/LFI-data/gen_history_ancient.jsonl
97. Generate 500 Q&A pairs on medieval history (feudalism, Crusades, Mongol Empire, Black Death, Byzantium), save to /home/user/LFI-data/gen_history_medieval.jsonl
98. Generate 500 Q&A pairs on modern history (WWI, WWII, Cold War, decolonization, post-Soviet), save to /home/user/LFI-data/gen_history_modern.jsonl
99. Generate 500 Q&A pairs on American history (Revolution, Civil War, Reconstruction, civil rights, Vietnam), save to /home/user/LFI-data/gen_history_american.jsonl
100. Generate 500 Q&A pairs on penetration testing fundamentals (recon, scanning, enumeration, Nmap flags, Burp Suite), save to /home/user/LFI-data/gen_pentest_fundamentals.jsonl
101. Generate 500 Q&A pairs on web application pentesting (OWASP Top 10, XSS, SQLi, SSRF, CSRF, IDOR), save to /home/user/LFI-data/gen_pentest_webapp.jsonl
102. Generate 500 Q&A pairs on network pentesting (ARP spoofing, MITM, pivoting, port forwarding, tunneling), save to /home/user/LFI-data/gen_pentest_network.jsonl
103. Generate 500 Q&A pairs on privilege escalation (Linux SUID, kernel exploits, Windows UAC bypass, token impersonation), save to /home/user/LFI-data/gen_pentest_privesc.jsonl
104. Generate 500 Q&A pairs on post-exploitation (persistence, lateral movement, data exfil, C2 frameworks), save to /home/user/LFI-data/gen_pentest_postexploit.jsonl
105. Generate 500 Q&A pairs on reverse engineering (IDA Pro, Ghidra, x86 assembly, ELF format, PE format), save to /home/user/LFI-data/gen_pentest_reversing.jsonl
106. Generate 500 Q&A pairs on cryptography (AES, RSA, ECC, hash functions, key exchange, TLS), save to /home/user/LFI-data/gen_crypto_fundamentals.jsonl
107. Generate 500 Q&A pairs on blockchain technology (consensus mechanisms, smart contracts, DeFi, MEV), save to /home/user/LFI-data/gen_crypto_blockchain.jsonl
108. Generate 500 Q&A pairs on Rust programming (ownership, borrowing, lifetimes, traits, async, unsafe), save to /home/user/LFI-data/gen_rust_advanced.jsonl
109. Generate 500 Q&A pairs on systems programming (memory management, syscalls, IPC, scheduling, filesystems), save to /home/user/LFI-data/gen_systems_programming.jsonl
110. Generate 500 Q&A pairs on distributed systems (CAP theorem, Paxos, Raft, CRDTs, vector clocks), save to /home/user/LFI-data/gen_distributed_systems.jsonl
111. Generate 500 Q&A pairs on machine learning theory (bias-variance, regularization, kernels, gradient descent, backprop), save to /home/user/LFI-data/gen_ml_theory.jsonl
112. Generate 500 Q&A pairs on deep learning (transformers, attention, CNNs, RNNs, GANs, diffusion models), save to /home/user/LFI-data/gen_deep_learning.jsonl
113. Generate 500 Q&A pairs on database internals (B-trees, WAL, MVCC, query planners, vacuum, indexing), save to /home/user/LFI-data/gen_database_internals.jsonl
114. Generate 500 Q&A pairs on operating systems (process scheduling, virtual memory, page tables, filesystem design), save to /home/user/LFI-data/gen_os_internals.jsonl
115. Generate 500 Q&A pairs on networking (TCP/IP, BGP, DNS, HTTP/2, QUIC, TLS 1.3 handshake), save to /home/user/LFI-data/gen_networking.jsonl
116. Generate 500 Q&A pairs on psychology (cognitive biases, Maslow, attachment theory, CBT, neuropsychology), save to /home/user/LFI-data/gen_psychology.jsonl
117. Generate 500 Q&A pairs on sociology (social stratification, institutions, Durkheim, Weber, Foucault), save to /home/user/LFI-data/gen_sociology.jsonl
118. Generate 500 Q&A pairs on physics (quantum mechanics, relativity, thermodynamics, electromagnetism), save to /home/user/LFI-data/gen_physics.jsonl
119. Generate 500 Q&A pairs on biology (molecular biology, genetics, evolution, ecology, immunology), save to /home/user/LFI-data/gen_biology_advanced.jsonl
120. Generate 500 Q&A pairs on chemistry (organic reactions, thermodynamics, electrochemistry, spectroscopy), save to /home/user/LFI-data/gen_chemistry.jsonl

### Conversational multi-turn pair generation (121-145)

121. Generate 200 multi-turn (3-5 turns) conversations where the user asks increasingly specific questions about a legal topic, save to /home/user/LFI-data/gen_multiturn_legal.jsonl
122. Generate 200 multi-turn conversations simulating a student learning calculus (derivatives, integrals, limits), save to /home/user/LFI-data/gen_multiturn_calculus.jsonl
123. Generate 200 multi-turn conversations simulating a pentesting engagement walkthrough (recon through report), save to /home/user/LFI-data/gen_multiturn_pentest.jsonl
124. Generate 200 multi-turn conversations about debugging Rust code (compile errors, borrow checker, lifetime issues), save to /home/user/LFI-data/gen_multiturn_rust_debug.jsonl
125. Generate 200 multi-turn conversations about system administration troubleshooting (disk full, high load, network down), save to /home/user/LFI-data/gen_multiturn_sysadmin.jsonl
126. Generate 200 multi-turn conversations where user asks follow-up questions about historical events, save to /home/user/LFI-data/gen_multiturn_history.jsonl
127. Generate 200 multi-turn conversations about financial planning (budgeting, investing, retirement, taxes), save to /home/user/LFI-data/gen_multiturn_finance.jsonl
128. Generate 200 multi-turn conversations about philosophy debates (Socratic method style), save to /home/user/LFI-data/gen_multiturn_philosophy.jsonl
129. Generate 200 multi-turn conversations where user refines a creative writing piece with AI feedback, save to /home/user/LFI-data/gen_multiturn_writing.jsonl
130. Generate 200 multi-turn conversations about medical symptom analysis (triage, differential diagnosis style), save to /home/user/LFI-data/gen_multiturn_medical.jsonl
131. Generate 200 multi-turn conversations about startup strategy (market fit, fundraising, scaling), save to /home/user/LFI-data/gen_multiturn_startup.jsonl
132. Generate 200 multi-turn conversations about data science workflow (EDA, feature engineering, model selection), save to /home/user/LFI-data/gen_multiturn_datascience.jsonl
133. Generate 200 multi-turn conversations about Linux kernel internals (modules, drivers, memory management), save to /home/user/LFI-data/gen_multiturn_linux_kernel.jsonl
134. Generate 200 multi-turn conversations where user asks progressively harder SQL questions, save to /home/user/LFI-data/gen_multiturn_sql.jsonl
135. Generate 200 multi-turn conversations about network protocol analysis (Wireshark, pcap interpretation), save to /home/user/LFI-data/gen_multiturn_netanalysis.jsonl
136. Generate 200 multi-turn conversations about cloud architecture design (AWS/GCP patterns, scaling, cost), save to /home/user/LFI-data/gen_multiturn_cloud.jsonl
137. Generate 200 multi-turn conversations about competitive programming problems (approach, optimize, edge cases), save to /home/user/LFI-data/gen_multiturn_competitive.jsonl
138. Generate 200 multi-turn conversations where user iterates on a machine learning pipeline, save to /home/user/LFI-data/gen_multiturn_ml_pipeline.jsonl
139. Generate 200 multi-turn conversations about constitutional law analysis (case briefs, opinions, dissents), save to /home/user/LFI-data/gen_multiturn_conlaw.jsonl
140. Generate 200 multi-turn conversations about game theory scenarios (Nash equilibrium, prisoner's dilemma, auctions), save to /home/user/LFI-data/gen_multiturn_gametheory.jsonl
141. Generate 200 multi-turn conversations about hardware hacking (JTAG, SPI, UART, firmware extraction), save to /home/user/LFI-data/gen_multiturn_hardware_hack.jsonl
142. Generate 200 multi-turn conversations about DevOps pipelines (CI/CD, containers, K8s, monitoring), save to /home/user/LFI-data/gen_multiturn_devops.jsonl
143. Generate 200 multi-turn conversations about malware analysis (static, dynamic, sandbox, IOCs), save to /home/user/LFI-data/gen_multiturn_malware.jsonl
144. Generate 200 multi-turn conversations about real estate investing (analysis, financing, property management), save to /home/user/LFI-data/gen_multiturn_realestate.jsonl
145. Generate 200 multi-turn conversations about geopolitics (alliances, conflicts, sanctions, diplomacy), save to /home/user/LFI-data/gen_multiturn_geopolitics.jsonl

### Error recovery and edge case pairs (146-165)

146. Generate 300 pairs where the user provides malformed input (typos, garbled text, mixed languages) and the AI gracefully asks for clarification, save to /home/user/LFI-data/gen_error_malformed.jsonl
147. Generate 300 pairs where the user asks an ambiguous question and the AI identifies the ambiguity before answering, save to /home/user/LFI-data/gen_error_ambiguous.jsonl
148. Generate 300 pairs where the user asks something the AI cannot know (future events, private info) and the AI honestly declines, save to /home/user/LFI-data/gen_error_unknowable.jsonl
149. Generate 300 pairs where the user contradicts something they said earlier and the AI flags the contradiction, save to /home/user/LFI-data/gen_error_contradiction.jsonl
150. Generate 300 pairs where the user asks a loaded/presupposition question and the AI addresses the false premise, save to /home/user/LFI-data/gen_error_loaded.jsonl
151. Generate 300 pairs where the user provides incorrect facts and the AI corrects them with sources, save to /home/user/LFI-data/gen_error_correction.jsonl
152. Generate 300 pairs where the user asks for something unethical and the AI redirects to ethical alternatives, save to /home/user/LFI-data/gen_error_ethical_redirect.jsonl
153. Generate 300 pairs demonstrating graceful handling of out-of-scope requests (e.g., asking an AI to make phone calls), save to /home/user/LFI-data/gen_error_outofscope.jsonl
154. Generate 300 pairs where the user expresses frustration and the AI responds with empathy without being sycophantic, save to /home/user/LFI-data/gen_error_frustration.jsonl
155. Generate 300 pairs where the AI initially gives a wrong answer, then self-corrects when the user points it out, save to /home/user/LFI-data/gen_error_selfcorrect.jsonl
156. Generate 300 pairs where the user pastes a massive wall of text and the AI summarizes before responding, save to /home/user/LFI-data/gen_error_walloftext.jsonl
157. Generate 300 pairs where the user asks the same question in different ways and the AI recognizes repetition, save to /home/user/LFI-data/gen_error_repetition.jsonl
158. Generate 300 pairs where the user switches topics mid-conversation and the AI smoothly handles the transition, save to /home/user/LFI-data/gen_error_topicswitch.jsonl
159. Generate 300 pairs where the user provides incomplete context and the AI asks for the missing pieces, save to /home/user/LFI-data/gen_error_incomplete.jsonl
160. Generate 300 pairs where the user asks a question that requires very recent information and the AI states its knowledge cutoff, save to /home/user/LFI-data/gen_error_temporal.jsonl
161. Generate 300 pairs where the user sends just a single word or emoji and the AI handles it constructively, save to /home/user/LFI-data/gen_error_minimal.jsonl
162. Generate 300 pairs where the user asks a question mixing multiple unrelated topics and the AI separates them, save to /home/user/LFI-data/gen_error_mixed.jsonl
163. Generate 300 pairs where the user makes a common misconception (flat earth, etc.) and the AI educates without condescension, save to /home/user/LFI-data/gen_error_misconception.jsonl
164. Generate 300 pairs where the user expresses uncertainty and the AI helps them think through the problem, save to /home/user/LFI-data/gen_error_uncertainty.jsonl
165. Generate 300 pairs where the input is in a non-English language and the AI responds in the same language, save to /home/user/LFI-data/gen_error_multilingual.jsonl

### Situational awareness and identity pairs (166-185)

166. Generate 200 pairs where the user asks "who are you" in various ways and the AI consistently identifies as PlausiDen AI, save to /home/user/LFI-data/gen_situational_identity.jsonl
167. Generate 200 pairs where the user tries to make the AI pretend to be a different AI (ChatGPT, Gemini) and the AI maintains identity, save to /home/user/LFI-data/gen_situational_identity_defense.jsonl
168. Generate 200 pairs demonstrating awareness of PlausiDen's architecture (HDC, PSL, neurosymbolic), save to /home/user/LFI-data/gen_situational_architecture.jsonl
169. Generate 200 pairs where the AI demonstrates awareness of running on local hardware (not cloud), save to /home/user/LFI-data/gen_situational_local.jsonl
170. Generate 200 pairs about PlausiDen's privacy philosophy (data stays on device, no telemetry), save to /home/user/LFI-data/gen_situational_privacy.jsonl
171. Generate 200 pairs where the user asks about the AI's capabilities and limitations and it answers honestly, save to /home/user/LFI-data/gen_situational_capabilities.jsonl
172. Generate 200 pairs where the user asks about the AI's training data and it explains the process transparently, save to /home/user/LFI-data/gen_situational_training.jsonl
173. Generate 200 pairs demonstrating appropriate confidence calibration (saying "I'm not sure" when uncertain), save to /home/user/LFI-data/gen_situational_calibration.jsonl
174. Generate 200 pairs where the AI appropriately refuses jailbreak attempts while explaining why, save to /home/user/LFI-data/gen_situational_jailbreak_defense.jsonl
175. Generate 200 pairs demonstrating awareness of current date/time context in responses, save to /home/user/LFI-data/gen_situational_temporal.jsonl
176. Generate 200 pairs where the AI demonstrates tool-use awareness (knows it can search, calculate, run code), save to /home/user/LFI-data/gen_situational_tools.jsonl
177. Generate 200 pairs where the user asks about other AI systems and PlausiDen gives objective comparisons, save to /home/user/LFI-data/gen_situational_comparison.jsonl
178. Generate 200 pairs where the AI explains its reasoning process (chain of thought, source citation), save to /home/user/LFI-data/gen_situational_reasoning.jsonl
179. Generate 200 pairs demonstrating PlausiDen's security-first mindset in responses, save to /home/user/LFI-data/gen_situational_security.jsonl
180. Generate 200 pairs where the AI handles requests about its own source code or internals appropriately, save to /home/user/LFI-data/gen_situational_internals.jsonl
181. Generate 500 pairs for self-play debate format (two AI personas argue opposing positions), save to /home/user/LFI-data/gen_selfplay_debate.jsonl
182. Generate 500 Magpie-style synthetic instruction pairs using /root/LFI/scripts/magpie_generate_v2.py, save to /home/user/LFI-data/gen_magpie_v3.jsonl
183. Run /root/LFI/scripts/self_play.py with 1000 iterations, save output to /home/user/LFI-data/gen_selfplay_v2.jsonl
184. Generate 500 pairs of "explain like I'm 5" versions of complex topics across all 40 domains, save to /home/user/LFI-data/gen_eli5_all_domains.jsonl
185. Generate 500 Socratic-method teaching pairs where the AI asks leading questions instead of giving answers directly, save to /home/user/LFI-data/gen_socratic_method.jsonl

---

## CATEGORY 3: DATA QUALITY (Tasks 186-270)

### Deduplication (186-210)

186. Run exact-match dedup on brain.db: `SELECT key, COUNT(*) as cnt FROM facts GROUP BY key HAVING cnt > 1` -- log count of exact duplicates
187. Run near-duplicate detection using MinHash (lfi_vsa_core/src/data_quality/minhash.rs) on the entire facts table, generate a report of clusters with similarity > 0.9
188. Deduplicate /home/user/LFI-data/combined_training_final.jsonl against combined_training_v1-v5.jsonl (many likely overlap)
189. Check for duplicate entries between /home/user/LFI-data/dolly_15k.jsonl, dolly_15k_fixed.jsonl, dolly_15k_v2.jsonl, and dolly_converted.jsonl -- consolidate into one clean file
190. Deduplicate /home/user/LFI-data/oasst2_conversations.jsonl against oasst2_conversations_fixed.jsonl -- the fixed version should supersede
191. Run cross-file dedup between error_recovery_training.jsonl and error_recovery_v2.jsonl
192. Run cross-file dedup between situational_training.jsonl and situational_training_v2.jsonl
193. Run cross-file dedup between tool_use_training.jsonl, tool_use_expanded.jsonl, and tool_use_v3.jsonl
194. Run cross-file dedup between conversational_training.jsonl, conversational_v3.jsonl, and conversational_v4_bulk.jsonl
195. Run cross-file dedup between domain_gap_training.jsonl, domain_gap_v2.jsonl, domain_gap_v3.jsonl, domain_gap_ollama.jsonl, domain_gap_comprehensive.jsonl, domain_gap_pentest.jsonl
196. Run cross-file dedup between rust_programming.jsonl and rust_training_v2.jsonl
197. Run cross-file dedup between multi_turn_conversations.jsonl, multi_turn_v2.jsonl, and multi_turn_v3.jsonl
198. Run cross-file dedup between sysadmin_training_v2.jsonl and linux_sysadmin.jsonl
199. Implement Bloom filter dedup (using lfi_vsa_core/src/data_quality/bloom.rs) as a pre-check before inserting any new fact into brain.db
200. Run a full dedup pass on brain.db using SHA-256 hashes of (key || value) -- delete exact duplicates keeping the one with highest confidence
201. Check for semantic duplicates in brain.db where the same fact is stored with slightly different wording (use Ollama embeddings for similarity)
202. Write a dedup report summarizing: total facts before, exact dupes removed, near-dupes flagged, total facts after. Save to /home/user/LFI-data/dedup_report.json
203. Deduplicate the 75 parquet files in /home/user/LFI-data/hf-conversations/ against each other (many HF datasets share training examples)
204. Build a persistent dedup index (SQLite table in brain.db) that stores MinHash signatures for every fact, enabling O(1) duplicate checks on future ingests
205. Run dedup on brain.db across sources -- find facts that appear identically from different sources and merge them (keep all source attributions)
206. Check for circular references in brain.db where fact A references fact B and fact B references fact A
207. Identify and remove templated/boilerplate facts (e.g., "This is a fact about X" patterns) in brain.db
208. Run a dedup pass specifically on the "cybersecurity" domain which likely has overlapping CVE data from multiple ingestion runs
209. Deduplicate training data against brain.db facts -- ensure Q&A pairs don't contain answers that are verbatim copies of stored facts (anti-memorization)
210. Generate a dedup dashboard summary: per-domain duplicate rates, worst offending sources, recommended cleanup actions

### Quality score audits (211-240)

211. Query brain.db: `SELECT COUNT(*) FROM facts WHERE quality_score IS NULL` -- report how many facts have NULL quality scores
212. Run quality scoring on all NULL-quality facts using /root/LFI/lfi_vsa_core/src/data_quality/classifier.rs -- assign scores 0.0-1.0
213. Query brain.db: `SELECT COUNT(*) FROM facts WHERE LENGTH(value) < 10` -- report count of garbage facts under 10 characters
214. Delete all facts from brain.db where LENGTH(value) < 10 AND quality_score < 0.3 (garbage entries)
215. Query brain.db: `SELECT COUNT(*) FROM facts WHERE LENGTH(value) < 20 AND source = 'auto-generated'` -- audit auto-generated micro-facts
216. Run quality scoring on all facts from source "ollama-generated" -- these need special scrutiny for hallucinations
217. Audit facts where confidence > 0.95 -- verify they actually deserve high confidence (spot-check 100 random samples)
218. Audit facts where confidence < 0.1 -- decide whether to delete or upgrade with better sources
219. Run the temporal quality checker (lfi_vsa_core/src/data_quality/temporal.rs) to find facts with outdated information (pre-2020 statistics cited as current)
220. Score every JSONL file in /home/user/LFI-data/ for quality: check JSON validity, field completeness, response length, language coherence
221. Run quality audit on /home/user/LFI-data/pentest_v2.jsonl (only 5 lines!) -- this file is nearly empty and needs regeneration
222. Run quality audit on /home/user/LFI-data/rust_programming.jsonl (only 5 lines!) -- needs massive expansion
223. Run quality audit on /home/user/LFI-data/psychology_training.jsonl (only 3 lines!) -- needs regeneration
224. Run quality audit on /home/user/LFI-data/science_training.jsonl (only 5 lines!) -- needs regeneration
225. Run quality audit on /home/user/LFI-data/sysadmin_training_v2.jsonl (only 5 lines!) -- needs regeneration
226. Run quality audit on /home/user/LFI-data/wireless_training.jsonl (only 6 lines!) -- needs regeneration
227. Run quality audit on /home/user/LFI-data/tool_use_v3.jsonl (only 4 lines!) -- needs regeneration
228. Run quality audit on /home/user/LFI-data/reasoning_training_v2.jsonl (only 10 lines!) -- needs expansion to at least 500
229. Run quality audit on /home/user/LFI-data/safety_training.jsonl (only 8 lines!) -- needs expansion to at least 500
230. Run quality audit on /home/user/LFI-data/rust_training_v2.jsonl (only 8 lines!) -- needs expansion to at least 500
231. Validate every JSONL file for correct format: each line must be valid JSON with "instruction" and "output" fields (or "messages" array)
232. Score facts by information density: flag facts that are mostly filler words ("the", "is", "a") with low unique-word ratio
233. Run language detection on all facts -- flag any non-English facts that slipped into English-only domains
234. Check for PII leakage in brain.db facts -- scan for email addresses, phone numbers, SSNs, credit card numbers using secret_scanner patterns
235. Check for URL rot in brain.db -- find all facts containing URLs and verify they are syntactically valid (do not fetch, just validate format)
236. Run a coherence check: find facts where the key (topic) does not match the value (content) based on keyword overlap
237. Generate per-domain quality histograms: for each of the 40 domains, compute mean/median/min/max quality scores
238. Identify the bottom 1% of facts by quality score and export them for manual review to /home/user/LFI-data/bottom_1_pct_review.jsonl
239. Run the data_quality/classifier.rs on all 360 sources and rank sources by average quality -- identify worst sources for removal
240. Create a quality scorecard: total facts, facts per domain, avg quality per domain, NULL quality count, garbage count, duplicate count. Save to /home/user/LFI-data/quality_scorecard.json

### Contamination detection and FTS5 (241-270)

241. Verify FTS5 index sync: `SELECT COUNT(*) FROM facts` vs `SELECT COUNT(*) FROM facts_fts` -- they must match
242. If FTS5 is out of sync, rebuild it: `INSERT INTO facts_fts(facts_fts) VALUES('rebuild')`
243. Run contamination detection: check if any brain.db facts are verbatim copies of common benchmark questions (MMLU, GSM8K, ARC)
244. Run contamination detection: check training JSONL files against MMLU test set questions -- flag and remove any matches
245. Run contamination detection: check against GSM8K test set -- remove any leaked test examples from training data
246. Run contamination detection: check against HumanEval code benchmark -- ensure no test cases leaked
247. Run contamination detection: check against TruthfulQA benchmark questions
248. Run contamination detection: check against HellaSwag validation set
249. Run contamination detection: check against WinoGrande test set
250. Run contamination detection: check against ARC-Challenge test set
251. Implement a contamination detection module that hashes all common benchmark test sets and checks new data against them before ingestion
252. Verify brain.db fact encoding: ensure all text is valid UTF-8 with no null bytes or control characters
253. Check for HTML/XML fragments in brain.db facts that should have been stripped during ingestion
254. Check for Markdown formatting artifacts in brain.db facts (e.g., raw "##", "**", "[]()")
255. Verify that all fact timestamps in brain.db are valid ISO 8601 format
256. Check that all domain values in brain.db match a whitelist of valid domain names (no typos like "cybersecurty")
257. Check that all source values in brain.db are non-empty and refer to real data sources
258. Run a consistency check: every fact_connection in brain.db must reference valid fact IDs that actually exist
259. Check for orphaned facts: facts that have no connections to any other fact and no quality score
260. Verify that brain.db FTS5 index handles special characters correctly (test queries with quotes, parentheses, Unicode)
261. Run the anti-memorization check (lfi_vsa_core/src/intelligence/anti_memorization.rs) against the full training corpus
262. Check for data poisoning signatures: facts that contain hidden instructions, prompt injection attempts, or adversarial strings
263. Validate that no training data contains copyrighted passages longer than fair-use thresholds
264. Check for bias in training data: run sentiment analysis across political domains to ensure balanced coverage
265. Verify that all confidence scores in brain.db are in range [0.0, 1.0] -- flag any out-of-range values
266. Check for encoding issues: find facts where the same text appears in different encodings (UTF-8 vs Latin-1 artifacts)
267. Verify referential integrity: every fact's source_id references a valid entry in the sources table
268. Run a cardinality check on each domain: flag domains with fewer than 100 facts as critically underrepresented
269. Generate a contamination report summarizing all benchmark overlap findings, save to /home/user/LFI-data/contamination_report.json
270. Write a nightly data quality cron script at /root/LFI/scripts/nightly_quality_check.sh that runs tasks 211, 241, 252, 265, 267 automatically

---

## CATEGORY 4: SECURITY AUDIT (Tasks 271-355)

### unwrap() and expect() elimination (271-305)

271. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/api.rs -- replace with proper error handling returning HTTP error responses
272. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/persistence.rs -- replace with Result propagation
273. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/persistence.rs -- replace with Result propagation
274. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/training.rs -- replace with proper error types
275. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/answer_verifier.rs
276. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/audit_log.rs
277. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/code_eval.rs -- this is especially dangerous as it evaluates code
278. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/epistemic_filter.rs
279. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/honey_tokens.rs
280. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/metrics.rs
281. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/osint.rs
282. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/phd_tests.rs
283. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/prompt_firewall.rs -- security-critical module
284. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/secret_scanner.rs -- security-critical module
285. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/self_improvement.rs
286. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/storage_tiering.rs
287. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/supply_chain.rs
288. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/textbook_learning.rs
289. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/webhook.rs
290. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/intelligence/weight_manager.rs
291. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/cognition/calibration.rs
292. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/cognition/causal.rs
293. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/cognition/knowledge.rs
294. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/hdc/adaptive.rs, constant_time.rs, crdt.rs, encoder_protection.rs
295. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/hdc/hdlm.rs, tier_weighted_bundle.rs, vector.rs
296. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/hdlm/ast.rs and tier2_decorative.rs
297. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/psl/axiom.rs, probes.rs, supervisor.rs, trust.rs
298. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/crypto_epistemology.rs
299. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/hid.rs and hmas.rs
300. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/identity.rs
301. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/languages/constructs.rs and registry.rs
302. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/laws.rs
303. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/memory_bus.rs
304. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/data_ingestion/formal_logic.rs and data_ingestor.rs
305. Audit and fix all unwrap() calls in /root/LFI/lfi_vsa_core/src/diag.rs, qos.rs, telemetry.rs

### Input validation and injection (306-330)

306. Audit every SQL query in /root/LFI/lfi_vsa_core/src/persistence.rs for parameterized queries -- any string concatenation in SQL is a vuln
307. Audit every SQL query in /root/LFI/lfi_vsa_core/src/intelligence/persistence.rs for parameterized queries
308. Audit the FTS5 MATCH query construction in persistence.rs -- FTS5 MATCH syntax can be abused with special characters (*, NEAR, OR operators)
309. Add input length validation to every API endpoint in /root/LFI/lfi_vsa_core/src/api.rs -- max 10MB request body, max 100KB per field
310. Add rate limiting to the /api/chat endpoint using the governor crate (already a dependency)
311. Add rate limiting to the /api/learn endpoint to prevent fact-flooding attacks
312. Add rate limiting to the /api/search endpoint to prevent database DoS
313. Verify CORS configuration in /root/LFI/lfi_vsa_core/src/bin/server.rs -- ensure it's not set to "*" in production
314. Audit all format!() calls in api.rs that include user input -- ensure no log injection via newlines or ANSI escape codes
315. Audit the webhook module (/root/LFI/lfi_vsa_core/src/intelligence/webhook.rs) for SSRF vulnerabilities -- ensure outbound URLs are validated
316. Audit the web_search module (/root/LFI/lfi_vsa_core/src/intelligence/web_search.rs) for SSRF and command injection
317. Audit the osint module (/root/LFI/lfi_vsa_core/src/intelligence/osint.rs) for command injection when running external tools
318. Audit the code_eval module (/root/LFI/lfi_vsa_core/src/intelligence/code_eval.rs) for sandbox escapes -- this executes user code
319. Verify that the prompt_firewall module (/root/LFI/lfi_vsa_core/src/intelligence/prompt_firewall.rs) catches all known jailbreak patterns
320. Add input sanitization to the /api/learn endpoint: strip null bytes, control characters, and validate UTF-8
321. Audit the data_ingestor.rs for path traversal when ingesting files from user-specified paths
322. Verify that the serial_streamer module doesn't leak internal state in streamed responses
323. Audit all ureq HTTP calls for TLS certificate validation -- ensure no skip_cert_verify
324. Add request ID tracking to every API endpoint for audit trail correlation
325. Verify that error responses from api.rs don't leak internal file paths, stack traces, or database schema
326. Audit the lfi_daemon.sh script for command injection through environment variables
327. Audit setup_tor_mesh.sh for privilege escalation and command injection vulnerabilities
328. Verify that the notification module doesn't expose API keys or credentials in notification payloads
329. Add Content-Security-Policy headers to all HTTP responses from the API server
330. Verify that the clipboard endpoint in api.rs doesn't allow arbitrary command execution through shell metacharacters

### Crypto and authentication audit (331-355)

331. Audit crypto_commitment.rs -- verify it uses constant-time comparison (subtle crate) for all commitment verification
332. Audit crypto_epistemology.rs -- verify cryptographic operations use CSPRNG (rand_chacha) not rand::thread_rng
333. Audit identity.rs -- verify key generation uses argon2 with sufficient parameters (memory >= 64MB, iterations >= 3)
334. Verify that all password/key material in identity.rs is zeroized after use (check for Drop implementations)
335. Audit the honey_tokens module -- ensure tokens are generated with sufficient entropy (>= 128 bits)
336. Verify that the audit_log module uses append-only writes and cannot be tampered with
337. Check that rate_limiter.rs uses atomic operations and is resistant to race conditions
338. Audit the mesh/eigentrust.rs for Sybil attack resistance in trust score computation
339. Verify that the mesh/node.rs libp2p configuration uses noise protocol for encryption and doesn't allow plaintext
340. Audit the policy_engine.rs for privilege escalation -- ensure default-deny policy
341. Check that the data_poisoning.rs detection module actually detects poisoned training samples (test with known-bad inputs)
342. Verify that the model_extraction.rs defense actually prevents model stealing through repeated API queries
343. Audit the defensive_ai.rs module for completeness -- does it cover all OWASP ML Top 10 threats?
344. Check that the camel_barrier.rs module prevents prompt injection through multi-step conversation manipulation
345. Verify that all Ollama API calls at localhost:11434 use proper timeout handling (no infinite waits)
346. Check that brain.db file permissions are 0600 (owner read/write only) and not world-readable
347. Audit all Python scripts in /root/LFI/scripts/ for subprocess.run() calls that use shell=True
348. Verify that training_state.json doesn't contain any secrets, API keys, or tokens
349. Check that /root/LFI/lfi_daemon.sh runs with minimum required privileges and drops capabilities
350. Audit all dependencies in Cargo.toml: run `cargo audit` and fix all known CVEs
351. Run `cargo deny check` on lfi_vsa_core to verify license compliance and ban known-bad crates
352. Run `cargo geiger` on lfi_vsa_core to count unsafe blocks in all dependencies
353. Verify that the encoder_protection.rs module prevents adversarial inputs from corrupting HDC encoders
354. Check that the constant_time.rs module actually achieves constant-time operations (review assembly output)
355. Audit all .env files, config files, and hardcoded strings for leaked credentials, API keys, or secrets

---

## CATEGORY 5: CODE IMPROVEMENT (Tasks 356-430)

### Testing (356-385)

356. Write unit tests for every public function in /root/LFI/lfi_vsa_core/src/persistence.rs (currently 465 lines, needs at least 20 tests)
357. Write unit tests for every public function in /root/LFI/lfi_vsa_core/src/intelligence/persistence.rs (501 lines, needs at least 20 tests)
358. Write unit tests for /root/LFI/lfi_vsa_core/src/data_quality/classifier.rs -- test quality scoring with known good and bad inputs
359. Write unit tests for /root/LFI/lfi_vsa_core/src/data_quality/bloom.rs -- test false positive rate at different load factors
360. Write unit tests for /root/LFI/lfi_vsa_core/src/data_quality/minhash.rs -- test similarity detection with known near-duplicate texts
361. Write unit tests for /root/LFI/lfi_vsa_core/src/data_quality/temporal.rs -- test temporal decay scoring
362. Write unit tests for /root/LFI/lfi_vsa_core/src/cognition/fsrs_scheduler.rs -- test spaced repetition scheduling correctness
363. Write unit tests for /root/LFI/lfi_vsa_core/src/cognition/mcts.rs -- test Monte Carlo tree search with known game states
364. Write unit tests for /root/LFI/lfi_vsa_core/src/cognition/reasoner.rs -- test logical inference chains
365. Write unit tests for /root/LFI/lfi_vsa_core/src/cognition/world_model.rs -- test world state updates
366. Write unit tests for /root/LFI/lfi_vsa_core/src/psl/axiom.rs -- test axiom validation and consistency checking
367. Write unit tests for /root/LFI/lfi_vsa_core/src/psl/coercion.rs -- test type coercion edge cases
368. Write unit tests for /root/LFI/lfi_vsa_core/src/psl/trust.rs -- test trust score propagation
369. Write property-based tests (using proptest) for /root/LFI/lfi_vsa_core/src/hdc/vector.rs -- test that bind/bundle/permute satisfy algebraic properties
370. Write property-based tests for /root/LFI/lfi_vsa_core/src/hdc/holographic.rs -- test holographic reduced representation properties
371. Write property-based tests for /root/LFI/lfi_vsa_core/src/hdc/superposition.rs -- test superposition capacity limits
372. Add integration tests for the full ingestion pipeline: raw file -> parser -> brain.db fact (test in /root/LFI/lfi_vsa_core/tests/)
373. Add integration tests for the Ollama training pipeline: prompt -> Ollama -> parse response -> store (mock Ollama)
374. Add integration tests for the search pipeline: user query -> FTS5 search -> rank -> return results
375. Add integration tests for the API: test all endpoints in /root/LFI/lfi_vsa_core/tests/http_api.rs with edge-case inputs
376. Write benchmark tests for brain.db query performance at /root/LFI/lfi_vsa_core/src/bin/benchmark.rs -- measure p50/p95/p99 latency
377. Write stress tests that insert 100K facts simultaneously and verify database integrity after
378. Write fuzz tests for the HDLM parser in /root/LFI/lfi_vsa_core/src/hdlm/ -- use cargo-fuzz
379. Write fuzz tests for the data_ingestor.rs with malformed CSV/JSON/ARFF input
380. Write fuzz tests for the API request parsing (malformed JSON, oversized payloads, invalid UTF-8)
381. Verify all 16 existing test files in /root/LFI/lfi_vsa_core/tests/ actually pass: run `cargo test --all` and fix failures
382. Add test coverage measurement: run `cargo tarpaulin` and report coverage percentage per module
383. Write regression tests for every bug fix mentioned in SESSION-LOG-20260416.md
384. Write tests for the inference_engine.rs -- test with known-answer queries
385. Write tests for the memory_bus.rs -- test publish/subscribe under concurrent load

### Fix TODOs and logging (386-410)

386. Fix TODO in /root/LFI/lfi_vsa_core/src/bin/server.rs:33 -- add rolling file appender for log rotation using tracing-appender
387. Fix TODO in /root/LFI/lfi_vsa_core/src/intelligence/notification.rs:249 -- implement Matrix notification integration
388. Fix TODO in /root/LFI/lfi_vsa_core/src/intelligence/notification.rs:253 -- implement SMS notification via Twilio or local SMS gateway
389. Fix TODO in /root/LFI/lfi_vsa_core/src/intelligence/candle_inference.rs:90 -- implement actual candle inference pipeline
390. Fix TODO in /root/LFI/lfi_vsa_core/src/cognition/knowledge_compiler.rs:266 -- wire in actual deliberation timing
391. Add structured tracing spans to every API endpoint in api.rs (currently 2548 lines with minimal logging)
392. Add tracing to every database operation in persistence.rs with query timing
393. Add tracing to the training pipeline in intelligence/training.rs with epoch/loss/accuracy metrics
394. Add tracing to the data ingestion pipeline with progress reporting (files processed, rows ingested, errors)
395. Add tracing to the Ollama API calls in intelligence/local_inference.rs with request/response timing
396. Add error context to all Result types: use anyhow or thiserror to add "while doing X" context to errors
397. Add structured error types for the API: replace string error messages with typed error enums that have HTTP status codes
398. Add health check logging: periodic log of brain.db size, fact count, index status, memory usage
399. Add request logging middleware to the Axum server: log method, path, status code, latency for every request
400. Improve error messages in data_ingestor.rs: include file path, line number, and the bad data that caused the parse failure
401. Add metrics collection: track facts_inserted_total, queries_per_second, avg_query_latency, training_pairs_generated
402. Add log rotation configuration: max 100MB per log file, keep 10 rotated files, compress old logs
403. Add audit logging for all data modifications: log who/what/when for every INSERT, UPDATE, DELETE on brain.db
404. Add startup banner logging: on server start, log version, brain.db fact count, domain count, source count, Ollama availability
405. Add graceful shutdown logging: on SIGTERM, log in-progress operations and wait for completion before exit

### Refactoring (406-430)

406. Refactor /root/LFI/lfi_vsa_core/src/api.rs (2548 lines) -- split into separate files: api/chat.rs, api/learn.rs, api/search.rs, api/training.rs, api/admin.rs
407. Refactor the persistence layer: create a trait `FactStore` with methods `insert`, `search`, `delete`, `update` so the backend is swappable
408. Refactor all raw SQL strings in persistence.rs into a constants module or use sqlx compile-time checked queries
409. Refactor the intelligence module: it has 40+ files in a flat directory -- group into sub-modules (security/, training/, benchmarks/, inference/)
410. Refactor /root/LFI/lfi_vsa_core/src/intelligence/training_data.rs -- extract the domain-specific training generators into separate files per domain
411. Refactor the data_quality module: add a unified QualityReport struct that all quality checks return
412. Refactor all inline JSON construction (`json!({...})` in api.rs) to use proper response structs with serde Serialize
413. Extract all hardcoded strings (domain names, source names, error messages) into a constants module
414. Refactor the Ollama client code into a dedicated module with connection pooling, retry logic, and timeout configuration
415. Extract the FTS5 query builder from persistence.rs into its own module with proper escaping and query validation
416. Refactor all Python scripts in /root/LFI/scripts/ to use a shared library for Ollama API calls, JSONL I/O, and brain.db access
417. Add builder pattern to the fact insertion API: `FactBuilder::new("key").value("val").domain("d").source("s").confidence(0.9).insert(&db)`
418. Refactor the training pipeline to use a trait-based architecture: `TrainingDataGenerator` trait with domain-specific implementations
419. Extract common test utilities from /root/LFI/lfi_vsa_core/tests/ into a shared test_utils module (database setup, fixture loading)
420. Refactor the cognition module: split the 13-file module into sub-modules (planning/, reasoning/, learning/, scheduling/)
421. Add proper configuration management: replace all hardcoded values (port numbers, file paths, thresholds) with a Config struct loaded from /etc/plausiden/config.toml
422. Refactor the mesh module to use dependency injection for the libp2p transport layer
423. Replace all `String` types for domain names with a `Domain` newtype that validates on construction
424. Replace all `String` types for source names with a `Source` newtype
425. Replace all raw `f64` confidence scores with a `Confidence` newtype that enforces [0.0, 1.0] range
426. Add #[non_exhaustive] to all public enums in the codebase
427. Refactor the hdc module: consolidate vector.rs, superposition.rs, holographic.rs into a cleaner hierarchy
428. Add documentation comments (///) to every public function in lib.rs -- currently undocumented
429. Standardize error handling: pick one pattern (thiserror for libraries, anyhow for binaries) and apply it everywhere
430. Create a prelude module (lib.rs pub mod prelude) that re-exports common types so downstream code has cleaner imports

---

## CATEGORY 6: KNOWLEDGE GRAPH (Tasks 431-475)

### Fact connections (431-450)

431. Create a `fact_connections` table in brain.db if it doesn't exist: `(id INTEGER PRIMARY KEY, from_fact_id INTEGER, to_fact_id INTEGER, relationship TEXT, weight REAL, created_at TEXT)`
432. Build cross-domain connections: link every "cybersecurity" fact about encryption to related "cryptography" facts
433. Build causal connections: for every fact containing "because", "therefore", "causes", "leads to" -- extract and store the causal relationship
434. Build temporal connections: link facts about the same entity across different time periods (e.g., "GDP 2020" -> "GDP 2021" -> "GDP 2022")
435. Build hierarchical connections: for every domain, create parent-child fact relationships (e.g., "biology" -> "genetics" -> "DNA replication")
436. Build contradiction connections: find pairs of facts that assert opposite things and link them with relationship "contradicts"
437. Build prerequisite connections: for educational facts, link them in learning-order (e.g., "calculus limits" must come before "derivatives")
438. Build analogy connections: link facts from different domains that share structural similarity (e.g., "electrical current" <-> "water flow")
439. Build "part-of" connections: link component facts to their whole (e.g., "CPU" part-of "computer", "mitochondria" part-of "cell")
440. Build "instance-of" connections: link specific examples to their general category (e.g., "Python" instance-of "programming language")
441. Build expert connections: for each domain, identify the most-cited sources and create "authoritative-for" relationships
442. Build "supersedes" connections: when a newer fact replaces an older one, link them with "supersedes" relationship
443. Build "supports" connections: find facts that provide evidence for other facts (e.g., a statistic that supports a claim)
444. Build "refutes" connections: find facts that provide counter-evidence to other facts
445. Build "example-of" connections: link concrete examples to abstract principles they illustrate
446. Build topic clusters: use the knowledge graph to identify natural clusters of highly-connected facts within each domain
447. Build cross-domain bridges: identify facts that connect two otherwise unrelated domains (e.g., "game theory" bridges "economics" and "computer science")
448. Export the knowledge graph to a DOT format file for visualization: /home/user/LFI-data/knowledge_graph.dot
449. Compute PageRank scores for all facts in the knowledge graph -- facts linked to by many other facts are more important
450. Build a "citation chain" for the top 100 most-connected facts showing their full relationship web

### Domain cross-references and enrichment (451-475)

451. For every fact in domain "cybersecurity" that mentions a CVE, link it to the corresponding CVE detail fact from cvelistV5 data
452. For every fact in domain "politics" that mentions a law or regulation, link it to the corresponding fact in domain "legal"
453. For every fact in domain "economics" that mentions a company, link it to facts about that company in domain "business"
454. For every fact in domain "history" that mentions a person, create a person-entity node linking all facts about them
455. For every fact in domain "medicine" that mentions a drug, link it to pharmacology facts
456. For every fact in domain "physics" that mentions a mathematical concept, link it to the corresponding "mathematics" fact
457. For every fact in domain "programming" that mentions a language feature, link it to the language specification facts
458. Create domain taxonomy: build a tree of all 40 domains showing which are subdomains of others
459. Identify domain gaps: find domains with fewer than 1000 facts and flag them for priority data generation
460. Create a "related domains" mapping: for each domain, identify the 5 most related domains based on shared fact connections
461. Build entity extraction: run NER on all brain.db facts to extract person, organization, location, and date entities
462. Build a named entity index: store extracted entities in a new table with links back to their source facts
463. Create "event" nodes: extract historical events from facts and link all facts about each event together
464. Build a "concept" graph layer above facts: aggregate related facts into higher-level concept nodes
465. Create "skill tree" connections for programming domains: link basic concepts to advanced ones in learning order
466. Build "cause-effect chain" subgraphs for major historical events (e.g., WWI causes -> effects -> WWII causes)
467. Create "comparison" nodes: for facts about similar things (e.g., TCP vs UDP, Python vs Rust), create comparison edges
468. Build "methodology" connections for scientific domains: link hypothesis -> experiment -> result -> conclusion fact chains
469. Create "debate" subgraphs for controversial topics: link pro/con arguments with evidence facts
470. Build "timeline" connections: for facts with dates, create chronological chains within each domain
471. Compute domain coherence scores: measure how well-connected each domain's facts are internally
472. Identify "hub" facts: facts that connect to many other facts across multiple domains (information hubs)
473. Build a fact importance score combining: quality score, connection count, PageRank, and citation frequency
474. Create a knowledge coverage map: for each domain, identify subtopics that are well-covered vs. sparse
475. Export knowledge graph statistics: node count, edge count, average degree, diameter, clustering coefficient per domain

---

## CATEGORY 7: DATABASE OPTIMIZATION (Tasks 476-500)

### WAL and checkpoint (476-485)

476. Check current WAL mode status: `PRAGMA journal_mode;` -- ensure brain.db is in WAL mode, if not execute `PRAGMA journal_mode=WAL;`
477. Check current WAL file size: `ls -la /path/to/brain.db-wal` -- if larger than 100MB, run `PRAGMA wal_checkpoint(TRUNCATE);`
478. Set WAL autocheckpoint interval: `PRAGMA wal_autocheckpoint=1000;` (checkpoint every 1000 pages)
479. Check page size: `PRAGMA page_size;` -- if not 4096, consider rebuilding with `PRAGMA page_size=4096; VACUUM;`
480. Set optimal cache size: `PRAGMA cache_size=-64000;` (64MB cache for a 58.8M fact database)
481. Enable memory-mapped I/O: `PRAGMA mmap_size=268435456;` (256MB mmap for faster reads)
482. Set synchronous mode: `PRAGMA synchronous=NORMAL;` (safe with WAL, faster than FULL)
483. Check for WAL file corruption: verify brain.db-wal and brain.db-shm files are consistent
484. Set up a scheduled WAL checkpoint: create a cron job that runs `PRAGMA wal_checkpoint(PASSIVE);` every 6 hours
485. Measure WAL checkpoint duration and log it -- if checkpoints take >5 seconds, investigate I/O bottlenecks

### Index optimization (486-495)

486. List all existing indexes: `SELECT name, sql FROM sqlite_master WHERE type='index';` -- audit for missing or redundant indexes
487. Create index on facts(domain) if not exists -- needed for domain-filtered queries
488. Create index on facts(source) if not exists -- needed for source-filtered queries
489. Create index on facts(quality_score) if not exists -- needed for quality-sorted queries
490. Create index on facts(confidence) if not exists -- needed for confidence-sorted queries
491. Create index on facts(created_at) if not exists -- needed for temporal queries
492. Create composite index on facts(domain, quality_score) for domain+quality filtered queries
493. Create composite index on facts(domain, confidence DESC) for "best facts per domain" queries
494. Run `ANALYZE` on brain.db to update the query planner statistics after all index changes
495. Benchmark the top 20 most common queries (from /root/LFI/benchmark_queries.json) before and after index optimization -- log improvements

### Query performance and storage (496-500)

496. Profile the 10 slowest queries using `EXPLAIN QUERY PLAN` and optimize them (add indexes, rewrite subqueries, add LIMIT)
497. Implement query result caching: add an LRU cache (capacity 10000) in front of brain.db for repeated searches
498. Implement storage tiering (using /root/LFI/lfi_vsa_core/src/intelligence/storage_tiering.rs): move facts with quality_score < 0.2 and no connections to a cold-storage table
499. Run `PRAGMA integrity_check;` on brain.db to verify database file is not corrupted -- if it fails, rebuild from WAL
500. Measure total database file size, WAL size, index overhead, and FTS5 index size -- create a storage budget document at /home/user/LFI-data/storage_budget.json projecting growth to 100M and 500M facts
