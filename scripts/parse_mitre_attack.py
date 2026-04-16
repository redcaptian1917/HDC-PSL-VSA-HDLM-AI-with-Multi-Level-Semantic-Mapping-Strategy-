#!/usr/bin/env python3
"""Parse MITRE ATT&CK STIX JSON into structured security facts for brain.db staging."""

import json, sqlite3, os, glob, hashlib

DB = "/home/user/.local/share/plausiden/brain.db"
STIX_DIR = "/data/raw/mitre/attack-stix-data"

def get_conn():
    conn = sqlite3.connect(DB, timeout=300)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=300000")
    return conn

def mk(prefix, text):
    return f"{prefix}_{hashlib.md5(text.encode()).hexdigest()[:10]}"

def parse_stix_bundle(filepath):
    """Parse a STIX 2.1 bundle and extract techniques, mitigations, relationships."""
    with open(filepath) as f:
        bundle = json.load(f)
    
    facts = []
    objects = bundle.get("objects", [])
    
    # Index by ID for relationship resolution
    id_map = {}
    for obj in objects:
        id_map[obj.get("id", "")] = obj
    
    for obj in objects:
        obj_type = obj.get("type", "")
        
        if obj_type == "attack-pattern":
            # Technique
            name = obj.get("name", "")
            desc = obj.get("description", "")[:500]
            external_refs = obj.get("external_references", [])
            mitre_id = ""
            for ref in external_refs:
                if ref.get("source_name") == "mitre-attack":
                    mitre_id = ref.get("external_id", "")
                    break
            
            kill_chain = [kc.get("phase_name", "") for kc in obj.get("kill_chain_phases", [])]
            
            if name and desc:
                fact = f"MITRE ATT&CK {mitre_id}: {name} — {desc}"
                if kill_chain:
                    fact += f" Tactics: {', '.join(kill_chain)}."
                facts.append(("attack_technique", fact))
        
        elif obj_type == "course-of-action":
            # Mitigation
            name = obj.get("name", "")
            desc = obj.get("description", "")[:500]
            if name and desc:
                facts.append(("mitigation", f"Mitigation: {name} — {desc}"))
        
        elif obj_type == "malware" or obj_type == "tool":
            name = obj.get("name", "")
            desc = obj.get("description", "")[:500]
            if name and desc:
                facts.append(("tool", f"{'Malware' if obj_type == 'malware' else 'Tool'}: {name} — {desc}"))
        
        elif obj_type == "intrusion-set":
            name = obj.get("name", "")
            desc = obj.get("description", "")[:500]
            aliases = obj.get("aliases", [])
            if name and desc:
                fact = f"Threat Group: {name}"
                if aliases:
                    fact += f" (aliases: {', '.join(aliases[:5])})"
                fact += f" — {desc}"
                facts.append(("threat_group", fact))
        
        elif obj_type == "relationship":
            src = id_map.get(obj.get("source_ref", ""), {})
            tgt = id_map.get(obj.get("target_ref", ""), {})
            rel_type = obj.get("relationship_type", "")
            src_name = src.get("name", "")
            tgt_name = tgt.get("name", "")
            if src_name and tgt_name and rel_type:
                facts.append(("relationship", f"{src_name} {rel_type} {tgt_name}"))
    
    return facts

def main():
    conn = get_conn()
    cur = conn.cursor()
    total = 0
    
    # Find all STIX JSON bundles
    patterns = [
        os.path.join(STIX_DIR, "**", "*.json"),
    ]
    
    files = []
    for pattern in patterns:
        files.extend(glob.glob(pattern, recursive=True))
    
    print(f"Found {len(files)} STIX JSON files", flush=True)
    
    for filepath in files:
        if "enterprise-attack" not in filepath and "ics-attack" not in filepath and "mobile-attack" not in filepath:
            continue
        try:
            facts = parse_stix_bundle(filepath)
            for adv_type, fact_text in facts:
                key = mk("mitre", fact_text)
                cur.execute(
                    "INSERT OR IGNORE INTO facts_staging (key, value, source, confidence, domain, quality_score, subject, predicate) VALUES (?,?,?,?,?,?,?,?)",
                    (key, fact_text, "mitre_attack", 0.95, "cybersecurity", 0.95, "mitre_attack", adv_type)
                )
                total += cur.rowcount
            conn.commit()
            print(f"  {os.path.basename(filepath)}: {len(facts)} facts extracted, {total} total staged", flush=True)
        except Exception as e:
            print(f"  {os.path.basename(filepath)}: FAILED — {e}", flush=True)
    
    conn.close()
    print(f"\nMITRE ATT&CK TOTAL: {total} facts staged", flush=True)

if __name__ == "__main__":
    if os.path.exists(STIX_DIR):
        main()
    else:
        print(f"STIX data not yet downloaded at {STIX_DIR}")
