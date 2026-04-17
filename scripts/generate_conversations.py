#!/usr/bin/env python3
"""
Synthetic Conversation Generator — Full multi-turn dialogues.

Generates 5-10 turn conversations on diverse topics using Ollama.
Each conversation has natural flow: greeting → question → follow-up → deeper → wrap-up.

Usage:
    python3 generate_conversations.py --count 10 --domain cybersecurity
    python3 generate_conversations.py --count 20  # all domains
"""

import argparse
import json
import hashlib
import os
import sqlite3
import time
import requests

BRAIN_DB = os.path.expanduser("~/.local/share/plausiden/brain.db")
MODEL = os.environ.get("PLAUSIDEN_MODEL", "qwen2.5-coder:7b")
OUTPUT_DIR = os.path.expanduser("~/LFI-data/synthetic_conversations")

SCENARIOS = {
    "cybersecurity": [
        {"opener": "I think my system might be compromised. What should I check first?",
         "follow_ups": ["What about checking for rootkits?", "How do I analyze network traffic for suspicious activity?", "What tools should I use for forensics?"]},
        {"opener": "Can you explain how TLS 1.3 works and why it's more secure than 1.2?",
         "follow_ups": ["What about the 0-RTT feature?", "How does it handle key exchange?", "What vulnerabilities still exist?"]},
    ],
    "programming": [
        {"opener": "I'm building a web API in Rust. Should I use Axum or Actix-web?",
         "follow_ups": ["What about performance differences?", "How do I handle database connections?", "What's the best error handling pattern?"]},
        {"opener": "I keep getting data races in my concurrent code. Help me understand ownership.",
         "follow_ups": ["What's the difference between Rc and Arc?", "When should I use Mutex vs RwLock?", "Can you show me a lock-free alternative?"]},
    ],
    "physics": [
        {"opener": "Why can't anything travel faster than light?",
         "follow_ups": ["What about quantum entanglement?", "Does time dilation make it feel different?", "What about theoretical warp drives?"]},
    ],
    "conversational": [
        {"opener": "I'm feeling stuck in my career. Any advice?",
         "follow_ups": ["How do I know if I should switch fields?", "What skills are most transferable?", "How do I deal with imposter syndrome?"]},
        {"opener": "What's a good way to learn a new language?",
         "follow_ups": ["Is immersion really the best method?", "How long does it take to become conversational?", "Any apps you'd recommend?"]},
    ],
    "mathematics": [
        {"opener": "Explain the intuition behind eigenvalues and eigenvectors.",
         "follow_ups": ["How are they used in machine learning?", "Can you give a geometric interpretation?", "What about the power method for computing them?"]},
    ],
}


def generate_response(prompt: str, context: str = "") -> str:
    """Get a response from Ollama."""
    full_prompt = f"{context}\nUser: {prompt}\nAssistant:" if context else f"User: {prompt}\nAssistant:"
    try:
        r = requests.post("http://localhost:11434/api/generate", json={
            "model": MODEL, "stream": False,
            "prompt": full_prompt,
            "options": {"temperature": 0.7, "num_predict": 250}
        }, timeout=60)
        return r.json().get("response", "").strip()
    except Exception:
        return ""


def generate_conversation(scenario: dict, domain: str) -> list:
    """Generate a full multi-turn conversation from a scenario."""
    messages = []
    context = f"You are PlausiDen AI, a knowledgeable assistant. Have a natural conversation about {domain}."

    # Opening message
    opener = scenario["opener"]
    messages.append({"role": "user", "content": opener})

    response = generate_response(opener, context)
    if not response:
        return []
    messages.append({"role": "assistant", "content": response})
    context += f"\nUser: {opener}\nAssistant: {response}"

    # Follow-up turns
    for follow_up in scenario.get("follow_ups", []):
        messages.append({"role": "user", "content": follow_up})
        response = generate_response(follow_up, context)
        if not response:
            break
        messages.append({"role": "assistant", "content": response})
        context += f"\nUser: {follow_up}\nAssistant: {response}"

    return messages


def conversation_to_training_pairs(messages: list, domain: str) -> list:
    """Convert a conversation into training pairs."""
    pairs = []
    context_so_far = ""

    for i in range(0, len(messages) - 1, 2):
        if i + 1 >= len(messages):
            break
        user_msg = messages[i]["content"]
        ai_msg = messages[i + 1]["content"]

        # Each turn becomes a training pair with accumulated context
        instruction = f"{context_so_far}User: {user_msg}" if context_so_far else user_msg
        pairs.append({
            "instruction": instruction[-1000:],  # Cap context length
            "output": ai_msg,
            "domain": domain,
            "source": "synthetic_conversation",
            "quality": 0.75,
            "turn": i // 2 + 1,
        })
        context_so_far += f"User: {user_msg}\nAssistant: {ai_msg}\n"

    return pairs


def main():
    parser = argparse.ArgumentParser(description="Synthetic Conversation Generator")
    parser.add_argument("--count", type=int, default=5, help="Conversations per domain")
    parser.add_argument("--domain", type=str, default=None, help="Specific domain")
    parser.add_argument("--ingest", action="store_true", help="Ingest to brain.db")
    args = parser.parse_args()

    os.makedirs(OUTPUT_DIR, exist_ok=True)
    domains = [args.domain] if args.domain else list(SCENARIOS.keys())

    all_pairs = []
    total_convos = 0

    for domain in domains:
        scenarios = SCENARIOS.get(domain, [])
        for i, scenario in enumerate(scenarios[:args.count]):
            print(f"  [{domain}] Conversation {i+1}...", end=" ", flush=True)
            messages = generate_conversation(scenario, domain)
            if len(messages) >= 4:  # At least 2 turns
                pairs = conversation_to_training_pairs(messages, domain)
                all_pairs.extend(pairs)
                total_convos += 1
                print(f"{len(messages)} messages, {len(pairs)} pairs")
            else:
                print("too short, skipped")

    # Save
    ts = time.strftime("%Y%m%d_%H%M%S")
    outpath = os.path.join(OUTPUT_DIR, f"conversations_{ts}.jsonl")
    with open(outpath, "w") as f:
        for p in all_pairs:
            f.write(json.dumps(p) + "\n")
    print(f"\nGenerated {total_convos} conversations, {len(all_pairs)} training pairs → {outpath}")

    # Ingest
    if args.ingest and all_pairs:
        conn = sqlite3.connect(BRAIN_DB, timeout=30)
        conn.execute("PRAGMA busy_timeout=30000")
        added = 0
        for p in all_pairs:
            key = f"synconv_{hashlib.sha256((p['instruction']+p['output']).encode()).hexdigest()[:16]}"
            conn.execute(
                "INSERT OR IGNORE INTO facts (key,value,source,confidence,domain,quality_score) VALUES (?,?,?,?,?,?)",
                (key, f"Q: {p['instruction']}\nA: {p['output']}"[:5000],
                 "synthetic_conversation", 0.75, p["domain"], 0.75))
            added += 1
        conn.commit()
        conn.close()
        print(f"Ingested {added} pairs into brain.db")


if __name__ == "__main__":
    main()
