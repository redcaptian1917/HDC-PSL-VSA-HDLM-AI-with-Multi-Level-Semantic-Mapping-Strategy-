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

os_concepts = [
    "Operating system fundamentals: Kernel (manages hardware, processes, memory, I/O), Process (running program with own address space), Thread (lightweight execution unit within process, shares memory), Scheduler (decides which process runs — CFS in Linux, round-robin, priority-based), System calls (interface between user space and kernel — open, read, write, fork, exec, mmap).",
    "Memory management: Virtual memory (each process thinks it has full address space, MMU translates virtual→physical), Paging (4KB pages, page tables, TLB cache), Demand paging (load pages only when accessed — page fault triggers load), Swapping (move idle pages to disk), Memory-mapped files (mmap — map file contents to memory, kernel handles I/O). OOM killer terminates processes when memory exhausted.",
    "Filesystem concepts: Inodes (metadata: permissions, timestamps, block pointers — NOT the filename), Directories (map names to inode numbers), Hard links (multiple names for same inode), Soft/symbolic links (pointer to path, can break), Journaling (ext4, XFS — write intent log before data, prevents corruption on crash), Copy-on-Write (Btrfs, ZFS — never overwrite, always write new copy).",
    "Concurrency primitives: Mutex (mutual exclusion — only one thread holds lock), Semaphore (counter — allows N concurrent accesses), Read-Write lock (multiple readers OR one writer), Condition variable (wait for signal), Atomic operations (lock-free — compare-and-swap, fetch-and-add), Barrier (all threads must reach before any continue). Deadlock: circular wait — prevented by lock ordering or try-lock with timeout.",
]

compilers = [
    "Compiler pipeline: Lexing (source → tokens) → Parsing (tokens → AST) → Semantic analysis (type checking, name resolution) → IR generation (AST → intermediate representation) → Optimization (constant folding, dead code elimination, loop unrolling, inlining) → Code generation (IR → machine code). LLVM is the dominant backend: Clang, Rust, Swift all emit LLVM IR for optimization + codegen.",
    "Type systems: Static (types checked at compile time — Rust, Java, TypeScript) vs Dynamic (types checked at runtime — Python, JavaScript). Strong (no implicit coercion — Python, Rust) vs Weak (implicit coercion — JavaScript, C). Gradual typing (mix static + dynamic — TypeScript, Python with mypy). Dependent types (types depend on values — Idris, Agda — can express invariants in the type system).",
    "Garbage collection strategies: Reference counting (immediate, deterministic — Python, Swift — can't handle cycles without cycle detection), Tracing GC — Mark-and-sweep (pause, mark reachable, free unmarked), Generational (most objects die young — Gen0/1/2, minor/major collections), Concurrent/incremental (reduce pause times — Go's GC, ZGC). Rust avoids GC entirely via ownership + borrowing.",
]

embedded = [
    "Embedded systems constraints: Limited RAM (kilobytes to megabytes), limited flash (firmware storage), no OS or RTOS (real-time operating system — FreeRTOS, Zephyr), deterministic timing required, power constraints (battery life), physical environment (temperature, vibration). Programming: bare-metal C/C++, increasingly Rust (no_std). Debug: JTAG/SWD, logic analyzer, oscilloscope.",
    "RTOS concepts: Tasks/threads with priorities, preemptive scheduling (higher priority interrupts lower), ISR (Interrupt Service Routines — keep short, defer work to tasks), mutexes with priority inheritance (prevent priority inversion), message queues (inter-task communication), timers (periodic/one-shot). Hard real-time: missed deadline = system failure. Soft real-time: missed deadline = degraded performance.",
    "IoT security: Default credentials (Mirai botnet exploited this), unencrypted communications, no firmware signing (attacker can flash malicious firmware), physical access (UART/JTAG debug ports left enabled), no update mechanism, constrained crypto (limited compute for TLS), supply chain (compromised components). Defense: secure boot, encrypted comms (DTLS for constrained devices), OTA updates with signing, disable debug ports in production.",
]

more_cyber = [
    "Malware types: Virus (attaches to files, needs host), Worm (self-propagating, no host needed — WannaCry), Trojan (disguised as legitimate software), Ransomware (encrypts data, demands payment — RaaS model), Rootkit (hides presence, kernel-level), Bootkit (infects boot process, persists across OS reinstall), Fileless (lives in memory, uses legitimate tools — PowerShell, WMI), RAT (Remote Access Trojan — persistent backdoor).",
    "Threat intelligence: IOCs (Indicators of Compromise: IP addresses, domains, file hashes, URLs, email addresses), TTPs (Tactics, Techniques, Procedures — MITRE ATT&CK mapping), STIX/TAXII (standard formats for sharing threat intel), threat actor profiles (APT groups — nation-state vs criminal), diamond model (adversary, infrastructure, capability, victim), kill chain analysis, threat hunting (proactive search for undetected threats).",
    "Forensics methodology: Acquire (bit-for-bit image — dd, FTK Imager, preserve chain of custody), Preserve (hash verification, write-blockers), Analyze (timeline analysis, file carving, registry analysis, memory forensics — Volatility), Report (document findings, maintain evidence integrity). Memory forensics: extract running processes, network connections, encryption keys, injected code. Anti-forensics: timestomping, log clearing, secure deletion, steganography.",
    "Cryptographic attacks: Brute force (try all keys — impractical for AES-256), Birthday attack (find collisions in hash — need 2^(n/2) attempts for n-bit hash), Side-channel (timing, power analysis, electromagnetic emanation — extract keys from physical measurements), Padding oracle (decrypt without key by observing error responses — POODLE, Bleichenbacher), Downgrade attack (force weaker crypto — DROWN, Logjam), Supply chain (compromise crypto implementation — Dual_EC_DRBG backdoor).",
]

conn = get_conn()
t = 0
t += ins(conn, os_concepts, "curated_os", "technology", 0.95)
t += ins(conn, compilers, "curated_compilers", "technology", 0.95)
t += ins(conn, embedded, "curated_embedded", "technology", 0.95)
t += ins(conn, more_cyber, "curated_cyber_adv", "cybersecurity", 0.95)
conn.close()
print(f"Inserted {t} curated facts (OS, compilers, embedded, advanced cyber)")
