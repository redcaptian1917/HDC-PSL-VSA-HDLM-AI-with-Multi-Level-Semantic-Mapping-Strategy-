import sqlite3, hashlib
DB = "/home/user/.local/share/plausiden/brain.db"
def get_conn():
    conn = sqlite3.connect(DB, timeout=300)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=300000")
    return conn
def mk(p,t): return f"{p}_{hashlib.md5(t.encode()).hexdigest()[:8]}"
def ins(conn, facts, src, dom, q):
    c = conn.cursor()
    n = 0
    for t in facts:
        try:
            c.execute("INSERT OR IGNORE INTO facts (key,value,source,confidence,domain,quality_score) VALUES (?,?,?,?,?,?)", (mk(src,t),t,src,q,dom,q))
            n += c.rowcount
        except: pass
    conn.commit()
    return n

distributed = [
    "CAP theorem (Brewer): A distributed system can provide at most 2 of 3: Consistency (all nodes see same data), Availability (every request gets a response), Partition tolerance (system works despite network splits). Since partitions are inevitable in distributed systems, the real choice is CP (consistent but may reject requests during partition — HBase, MongoDB) or AP (available but may return stale data — Cassandra, DynamoDB).",
    "Consensus algorithms: Paxos (Lamport — foundational, complex to implement), Raft (designed for understandability — leader election, log replication, safety — used in etcd, CockroachDB), PBFT (Byzantine fault tolerant — tolerates malicious nodes, used in permissioned blockchains), Viewstamped Replication. Key insight: impossible to achieve consensus with even one faulty node in an asynchronous system (FLP impossibility).",
    "Distributed system patterns: Circuit breaker (stop calling failing service, fail fast), Retry with exponential backoff + jitter, Bulkhead (isolate failures to prevent cascade), Saga (distributed transactions as compensating events), Event sourcing (append-only event log as source of truth), CQRS (separate read/write paths), Outbox pattern (reliable event publishing from DB transactions), Sidecar (co-located helper process).",
    "Consistent hashing: Distribute data across nodes in a ring. When a node is added/removed, only K/N keys need to move (vs all keys in modular hashing). Virtual nodes improve balance. Used in: DynamoDB, Cassandra, Memcached, CDNs. Enables elastic scaling without full data redistribution.",
]

blockchain = [
    "Blockchain fundamentals: Append-only linked list of blocks, each containing transactions + hash of previous block. Consensus mechanisms: Proof of Work (mining — energy-intensive, Bitcoin), Proof of Stake (validators stake tokens — Ethereum post-merge), Delegated PoS, Proof of Authority. Smart contracts: self-executing code on-chain (Solidity on Ethereum, Rust on Solana). Trilemma: decentralization, security, scalability — pick 2.",
    "Cryptocurrency concepts: Wallet (public key = address, private key = spending authority), Transaction (signed transfer of value, broadcast to network), Mining/Validating (adding blocks, earning rewards), Gas (computation fee on Ethereum — prevents infinite loops), DeFi (decentralized finance — lending, trading, yield farming without intermediaries), NFTs (unique tokens representing ownership of digital/physical assets).",
]

ux_design = [
    "UX design principles: Nielsen's 10 heuristics — visibility of system status, match between system and real world, user control and freedom, consistency and standards, error prevention, recognition over recall, flexibility and efficiency, aesthetic and minimalist design, help users recognize/diagnose/recover from errors, help and documentation. Most important: don't make users think (Steve Krug).",
    "Design system components: Typography scale (1.25 or 1.333 ratio), color palette (primary, secondary, neutral, semantic — success/warning/error/info), spacing scale (4/8px grid), component library (buttons, inputs, cards, modals, toasts, tables), motion tokens (duration, easing), elevation (shadow levels), breakpoints (mobile/tablet/desktop). Figma for design, Storybook for component documentation.",
    "Accessibility (WCAG 2.1 AA): Perceivable (alt text for images, captions for video, sufficient color contrast 4.5:1), Operable (keyboard navigable, no time limits, no seizure triggers), Understandable (clear language, predictable behavior, error suggestions), Robust (works with assistive tech — screen readers, proper ARIA labels, semantic HTML). Test with: axe-core, Lighthouse, real screen reader testing.",
    "Mobile UX patterns: Thumb zones (primary actions in easy reach area — bottom center), bottom navigation (max 5 items), pull-to-refresh, swipe gestures, skeleton screens (perceived performance > spinners), haptic feedback, adaptive layouts (not just responsive — different patterns for phone vs tablet), offline-first (cache, queue, sync), deep linking, onboarding (progressive disclosure, max 3 intro screens).",
]

product = [
    "Product management frameworks: RICE prioritization (Reach × Impact × Confidence / Effort), Jobs-to-be-Done (what job is the customer hiring your product for?), Kano model (basic/performance/delighter features), North Star Metric (single metric that captures core value delivery), Product-Market Fit canvas, opportunity scoring (importance vs satisfaction gap).",
    "Product discovery techniques: Customer interviews (open-ended, understand problems not solutions), Assumption mapping (most critical unknowns), Prototype testing (Figma clickable prototypes, Wizard of Oz), A/B testing (statistically significant experiments), Usability testing (5 users find 85% of issues — Nielsen), Analytics (funnel analysis, cohort retention, session recordings — Mixpanel, Amplitude, FullStory).",
    "Metrics that matter: Pirate metrics (AARRR — Acquisition, Activation, Retention, Revenue, Referral), engagement (DAU/MAU ratio — >25% is good for consumer), retention (Day 1, Day 7, Day 30 — the most important metric for product-market fit), NPS (Net Promoter Score — promoters minus detractors), time-to-value (how fast users reach 'aha moment').",
]

conn = get_conn()
t = 0
t += ins(conn, distributed, "curated_distributed", "technology", 0.95)
t += ins(conn, blockchain, "curated_blockchain", "technology", 0.93)
t += ins(conn, ux_design, "curated_ux", "technology", 0.95)
t += ins(conn, product, "curated_product", "business", 0.95)
conn.close()
print(f"Inserted {t} curated facts (distributed systems, blockchain, UX, product management)")
