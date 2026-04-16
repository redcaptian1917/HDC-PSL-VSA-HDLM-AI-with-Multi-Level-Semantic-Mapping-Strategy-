#!/usr/bin/env python3
"""Curated domain knowledge — business, economics, law, philosophy"""
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
        key = make_key(source, text)
        try:
            cur.execute("INSERT OR IGNORE INTO facts (key, value, source, confidence, domain, quality_score) VALUES (?,?,?,?,?,?)",
                (key, text, source, quality, domain, quality))
            count += cur.rowcount
        except: pass
    conn.commit()
    return count

business = [
    "Porter's Five Forces framework analyzes industry competitiveness: 1) Threat of new entrants (barriers to entry), 2) Bargaining power of suppliers, 3) Bargaining power of buyers, 4) Threat of substitutes, 5) Industry rivalry. Use it to assess whether an industry is attractive for investment or entry.",
    "The Business Model Canvas has 9 blocks: Key Partners, Key Activities, Key Resources, Value Propositions, Customer Relationships, Channels, Customer Segments, Cost Structure, Revenue Streams. It maps how a company creates, delivers, and captures value on a single page.",
    "Unit economics metrics: CAC (Customer Acquisition Cost) = total marketing spend / new customers. LTV (Lifetime Value) = average revenue per customer × average customer lifespan. LTV:CAC ratio should be >3:1 for a healthy SaaS business. Payback period = CAC / monthly revenue per customer.",
    "Startup funding stages: Pre-seed ($10K-$500K, friends/family/angels), Seed ($500K-$2M, angels/micro-VCs), Series A ($2M-$15M, VCs), Series B ($15M-$50M), Series C+ ($50M+, growth equity/PE). At each stage, investors expect increasing traction: idea → MVP → product-market fit → scalable growth → profitability path.",
    "The lean startup methodology: Build-Measure-Learn loop. Start with a Minimum Viable Product (MVP), measure customer behavior with actionable metrics (not vanity metrics), learn whether to pivot or persevere. Key concepts: validated learning, innovation accounting, pivot (zoom-in, zoom-out, customer segment, platform, business architecture).",
    "SaaS metrics: MRR (Monthly Recurring Revenue), ARR (Annual), churn rate (% customers lost per period), net revenue retention (>100% means expansion exceeds churn), gross margin (should be >70%), burn rate, runway (cash / monthly burn), magic number (net new ARR / S&M spend).",
    "Competitive moats (Warren Buffett concept): Network effects (Facebook, marketplace), switching costs (enterprise SaaS), economies of scale (Walmart), brand (Apple), regulatory/legal barriers (patents, licenses), proprietary technology, data advantages, counter-positioning (incumbent can't copy without hurting core business).",
]

economics = [
    "Supply and demand: Price is determined by the intersection of supply (quantity producers will sell at each price) and demand (quantity consumers will buy). When demand exceeds supply, prices rise. When supply exceeds demand, prices fall. Elasticity measures how responsive quantity is to price changes.",
    "GDP (Gross Domestic Product) = C + I + G + (X - M), where C = consumer spending, I = business investment, G = government spending, X = exports, M = imports. GDP measures total economic output. Real GDP adjusts for inflation; nominal GDP does not.",
    "Monetary policy tools: Interest rates (federal funds rate), open market operations (buying/selling government bonds), reserve requirements, quantitative easing (QE — buying assets to inject money). Lowering rates stimulates borrowing/spending; raising rates cools inflation. Central banks (Fed, ECB, BoE) set policy.",
    "Inflation types: Demand-pull (too much money chasing too few goods), cost-push (rising production costs passed to consumers), built-in (wage-price spiral). Measured by CPI (Consumer Price Index) and PCE (Personal Consumption Expenditures). Central banks typically target 2% annual inflation.",
    "Game theory fundamentals: Nash Equilibrium (no player can improve by unilaterally changing strategy), Prisoner's Dilemma (individual rationality leads to collectively worse outcome), Pareto Efficiency (no one can be made better off without making someone worse off), Mechanism Design (designing rules to achieve desired outcomes).",
    "Market structures: Perfect competition (many sellers, identical products, price takers), Monopolistic competition (many sellers, differentiated products), Oligopoly (few sellers, interdependent pricing — game theory applies), Monopoly (one seller, price maker). Real markets usually fall between these ideals.",
]

law = [
    "Contract law essentials: A valid contract requires 1) Offer, 2) Acceptance, 3) Consideration (something of value exchanged), 4) Capacity (parties are competent), 5) Legality (lawful purpose). Breach remedies: damages (compensatory, consequential, punitive), specific performance, rescission.",
    "Intellectual property types: Patents (inventions, 20 years, must be novel/non-obvious/useful), Copyrights (creative works, life+70 years, automatic upon creation), Trademarks (brand identifiers, renewable indefinitely, must be distinctive), Trade Secrets (confidential business information, no registration, protected as long as secret).",
    "The Fourth Amendment protects against unreasonable searches and seizures. Key concepts: probable cause, warrant requirement, exceptions (consent, plain view, exigent circumstances, search incident to arrest, automobile exception, stop and frisk/Terry stop). Digital privacy: Carpenter v. US (2018) — cell location data requires a warrant.",
    "Corporate structures: Sole proprietorship (simple, unlimited liability), LLC (limited liability, pass-through taxation, flexible management), C-Corp (double taxation but unlimited shareholders, preferred for VC funding), S-Corp (pass-through, max 100 shareholders), Partnership (general or limited). Delaware is preferred for incorporation due to business-friendly courts and law.",
    "GDPR core principles: Lawfulness/fairness/transparency, purpose limitation, data minimization, accuracy, storage limitation, integrity/confidentiality, accountability. Key rights: access, rectification, erasure (right to be forgotten), portability, objection. Penalties: up to 4% of global annual revenue or 20M EUR.",
]

philosophy = [
    "Epistemology — the study of knowledge. Key questions: What can we know? How do we know it? Major positions: Empiricism (knowledge from experience — Locke, Hume), Rationalism (knowledge from reason — Descartes, Leibniz), Kant's synthesis (experience provides content, reason provides structure), Pragmatism (truth is what works — James, Dewey).",
    "Ethics frameworks: Consequentialism/Utilitarianism (actions judged by outcomes — Mill, Bentham), Deontology (actions judged by rules/duties — Kant's categorical imperative), Virtue Ethics (character-based — Aristotle), Care Ethics (relationships and context — Gilligan, Noddings). No single framework is universally correct; each captures something important.",
    "The Mind-Body Problem: How does subjective experience arise from physical matter? Dualism (mind and body are separate substances — Descartes), Physicalism (everything is physical — consciousness is brain activity), Functionalism (mental states are functional roles, not physical states), Panpsychism (consciousness is fundamental to all matter).",
    "Existentialism: Existence precedes essence — we are not born with a fixed nature but create ourselves through choices. Key thinkers: Kierkegaard (anxiety, leap of faith), Nietzsche (will to power, eternal recurrence, death of God), Heidegger (being-in-the-world, authenticity), Sartre (radical freedom, bad faith), Camus (absurdism, revolt against meaninglessness).",
    "Political philosophy spectrum: Libertarianism (minimal state, maximum individual freedom — Nozick), Classical Liberalism (individual rights, limited government — Locke, Mill), Social Liberalism (positive rights, welfare state — Rawls), Socialism (collective ownership, equality — Marx), Communitarianism (community values over individual — Taylor, Sandel).",
    "Philosophy of science: Falsificationism (a theory is scientific only if it can be proven false — Popper), Paradigm shifts (science progresses through revolutions, not accumulation — Kuhn), Research programmes (competing frameworks evaluated by progressiveness — Lakatos), Anarchism (no single scientific method — Feyerabend).",
]

conn = get_conn()
total = 0
total += insert_facts(conn, business, "curated_business", "business", 0.95)
total += insert_facts(conn, economics, "curated_economics", "economics", 0.95)
total += insert_facts(conn, law, "curated_law", "legal", 0.95)
total += insert_facts(conn, philosophy, "curated_philosophy", "philosophy", 0.95)
conn.close()
print(f"Inserted {total} curated domain facts (business, economics, law, philosophy)")
