import sqlite3, hashlib

DB = "/home/user/.local/share/plausiden/brain.db"
def get_conn():
    conn = sqlite3.connect(DB, timeout=300)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=300000")
    return conn

def make_key(prefix, text):
    return f"{prefix}_{hashlib.md5(text.encode()).hexdigest()[:8]}"

def insert_facts(conn, facts, source, domain, quality):
    cur = conn.cursor()
    count = 0
    for text in facts:
        try:
            cur.execute("INSERT OR IGNORE INTO facts (key, value, source, confidence, domain, quality_score) VALUES (?,?,?,?,?,?)",
                (make_key(source, text), text, source, quality, domain, quality))
            count += cur.rowcount
        except: pass
    conn.commit()
    return count

marketing = [
    "The marketing funnel: Awareness → Interest → Consideration → Intent → Evaluation → Purchase → Loyalty → Advocacy. Modern marketers focus on the full funnel, not just acquisition. CAC payback and retention metrics matter more than top-of-funnel vanity metrics.",
    "Content marketing strategy: Create valuable, relevant content to attract and retain a defined audience. Types: blog posts, whitepapers, case studies, videos, podcasts, infographics, webinars. Distribution: SEO (organic), social media, email, paid promotion. Measure: traffic, engagement, leads generated, conversion rate.",
    "SEO fundamentals: On-page (title tags, meta descriptions, headers, keyword placement, internal linking, page speed, mobile-friendly), Off-page (backlinks, domain authority, social signals), Technical (crawlability, XML sitemaps, schema markup, Core Web Vitals, HTTPS). Google E-E-A-T: Experience, Expertise, Authoritativeness, Trustworthiness.",
    "Product-market fit: Marc Andreessen's concept — being in a good market with a product that can satisfy that market. Signals: organic word-of-mouth growth, high retention (>40% weekly for consumer, >80% monthly for SaaS), pull from customers (they seek you out), sales cycle shortening. Sean Ellis test: >40% of users would be 'very disappointed' without your product.",
    "Pricing strategies: Cost-plus (cost + margin), Value-based (what customers will pay), Competitive (match market), Penetration (low to gain share), Skimming (high for early adopters, decrease over time), Freemium (free base + paid premium), Usage-based (pay per use). SaaS trend: moving from seat-based to usage-based pricing.",
]

accounting = [
    "The accounting equation: Assets = Liabilities + Equity. Every transaction must keep this balanced (double-entry bookkeeping). Assets are what you own, liabilities are what you owe, equity is the owner's residual interest.",
    "Three core financial statements: 1) Income Statement (revenue - expenses = net income, over a period), 2) Balance Sheet (assets = liabilities + equity, at a point in time), 3) Cash Flow Statement (operating + investing + financing activities, reconciles net income to cash). They are interconnected: net income flows to retained earnings on the balance sheet.",
    "Key financial ratios: Profitability (gross margin, net margin, ROE, ROA), Liquidity (current ratio = current assets/current liabilities, quick ratio), Leverage (debt-to-equity, interest coverage), Efficiency (inventory turnover, accounts receivable turnover, asset turnover). Investors compare ratios to industry benchmarks.",
    "Revenue recognition (ASC 606): 5-step model: 1) Identify contract, 2) Identify performance obligations, 3) Determine transaction price, 4) Allocate price to obligations, 5) Recognize revenue when obligation is satisfied. SaaS companies recognize subscription revenue ratably over the contract period, not upfront.",
    "EBITDA = Earnings Before Interest, Taxes, Depreciation, and Amortization. Used as a proxy for operating cash flow and for company valuation (EV/EBITDA multiples). Critics argue it ignores capex, working capital needs, and can be manipulated. Free Cash Flow (FCF) = operating cash flow - capex is often a better measure.",
]

history = [
    "The Industrial Revolution (1760-1840) transformed manufacturing from hand production to machine-based production. Key innovations: steam engine (Watt), spinning jenny, power loom, iron smelting with coke. Effects: urbanization, factory system, rise of capitalism, labor movements, environmental degradation, global economic transformation.",
    "World War I (1914-1918) was triggered by the assassination of Archduke Franz Ferdinand but caused by: alliance systems, militarism, imperialism, nationalism. Introduced: trench warfare, chemical weapons, tanks, aircraft combat. Consequences: fall of empires (Ottoman, Austro-Hungarian, Russian, German), Treaty of Versailles, League of Nations, conditions for WWII.",
    "The Cold War (1947-1991) was a geopolitical rivalry between the US and USSR. Key events: Berlin Blockade (1948), Korean War, Cuban Missile Crisis (1962), Vietnam War, Space Race, détente, Soviet-Afghan War, fall of the Berlin Wall (1989), dissolution of USSR (1991). Nuclear deterrence (MAD) prevented direct conflict between superpowers.",
    "The Scientific Revolution (1543-1687): Copernicus (heliocentric model), Galileo (telescopic observations, experimental method), Kepler (planetary motion laws), Newton (gravity, calculus, laws of motion), Bacon (empirical method), Descartes (rationalism, dualism). Shifted authority from scripture and Aristotle to observation and mathematics.",
    "The Renaissance (14th-17th century): Cultural rebirth originating in Italian city-states (Florence, Venice). Key figures: da Vinci, Michelangelo, Raphael (art), Machiavelli (political theory), Erasmus (humanism). Driven by: rediscovery of classical texts, printing press (Gutenberg, 1440), patronage system, growing merchant class, weakening of feudalism.",
]

linux_admin = [
    "Linux file permissions: rwx for owner, group, others. Numeric: r=4, w=2, x=1. Common: 755 (rwxr-xr-x, executables), 644 (rw-r--r--, files), 600 (rw-------, secrets). Special: SUID (4xxx, run as owner), SGID (2xxx, run as group), sticky bit (1xxx, only owner can delete in shared dirs like /tmp).",
    "Systemd service management: systemctl start/stop/restart/enable/disable/status <service>. Unit files in /etc/systemd/system/ (admin) or /lib/systemd/system/ (packages). Key sections: [Unit] (dependencies), [Service] (ExecStart, Type, Restart), [Install] (WantedBy). journalctl -u <service> for logs.",
    "Linux networking: ip addr (show interfaces), ip route (routing table), ss -tlnp (listening ports), nftables/iptables (firewall), /etc/resolv.conf (DNS), NetworkManager or systemd-networkd. Troubleshooting: ping, traceroute, dig/nslookup, tcpdump, wireshark, curl -v, netcat.",
    "Process management: ps aux (list all), top/htop (real-time), kill -SIGTERM <pid> (graceful), kill -9 <pid> (force), nice/renice (priority), nohup (survive logout), & (background), jobs/fg/bg, systemd for services. /proc/<pid>/ has process details. cgroups limit resources.",
    "Disk management: lsblk (list block devices), fdisk/gdisk (partition), mkfs (create filesystem), mount/umount, /etc/fstab (persistent mounts), df -h (disk usage), du -sh (directory size), LVM (flexible volume management: pvcreate/vgcreate/lvcreate), RAID (mdadm), smartctl (disk health).",
    "SSH hardening: Disable root login (PermitRootLogin no), use key-based auth (disable PasswordAuthentication), change default port, use fail2ban, AllowUsers/AllowGroups directive, use ed25519 keys, enable 2FA with Google Authenticator PAM module, SSH jump hosts for network segmentation.",
]

social_eng = [
    "Social engineering attack lifecycle: 1) Research (OSINT on target — LinkedIn, social media, company info), 2) Develop relationship/pretext, 3) Exploit trust (urgency, authority, reciprocity, scarcity, social proof, likability — Cialdini's principles), 4) Execute attack (credential harvest, malware delivery, physical access), 5) Debrief (cover tracks or maintain access).",
    "Phishing indicators: Urgency/fear language ('Your account will be suspended'), generic greetings ('Dear Customer'), misspelled domains (microsofft.com), mismatched links (hover to check), suspicious attachments (.exe, .scr, macro-enabled docs), requests for credentials or financial info, unusual sender address. SPF/DKIM/DMARC headers help verify sender authenticity.",
    "Physical social engineering: Tailgating (following authorized person through door), impersonation (delivery person, IT tech, executive), shoulder surfing, dumpster diving, badge cloning (Proxmark3), USB drop attack (malicious USB in parking lot), phone pretexting (help desk calls). Defense: security awareness, visitor management, clean desk policy, badge policies.",
    "Psychological manipulation techniques used in social engineering: Authority (pretending to be IT/management), Urgency (deadline pressure), Social proof (everyone else has done this), Reciprocity (I did something for you, now...), Commitment/consistency (small yes leads to big yes), Likability (build rapport first). Recognizing these patterns is the primary defense.",
]

conn = get_conn()
total = 0
total += insert_facts(conn, marketing, "curated_marketing", "business", 0.95)
total += insert_facts(conn, accounting, "curated_accounting", "finance", 0.95)
total += insert_facts(conn, history, "curated_history", "history", 0.95)
total += insert_facts(conn, linux_admin, "curated_linux", "technology", 0.95)
total += insert_facts(conn, social_eng, "curated_social_eng", "cybersecurity", 0.95)
conn.close()
print(f"Inserted {total} curated domain facts (marketing, accounting, history, Linux, social eng)")
