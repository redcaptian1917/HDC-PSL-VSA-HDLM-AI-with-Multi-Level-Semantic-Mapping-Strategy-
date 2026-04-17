# PlausiDen AI Operations Platform — The Vision

## What This Becomes

PlausiDen isn't just an AI. It's an **AI operations platform** that manages, orchestrates, 
and trains multiple AI agents across multiple providers, with LFI as the sovereign 
intelligence core that watches, learns, and improves from everything.

## The Architecture

```
┌──────────────────────────────────────────────────────────┐
│                    PlausiDen Platform                      │
│                                                            │
│  ┌──────┐ ┌───────────┐ ┌───────┐ ┌───────┐ ┌─────────┐│
│  │ Chat │ │ Classroom │ │ Fleet │ │ Admin │ │ Missions ││
│  └──────┘ └───────────┘ └───────┘ └───────┘ └─────────┘│
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌──────────────────┐│
│  │ Vault  │ │ Security │ │ Policy │ │ Knowledge Graph  ││
│  └────────┘ └──────────┘ └────────┘ └──────────────────┘│
╠══════════════════════════════════════════════════════════╣
│              ORCHESTRATOR + TASK ENGINE                    │
│                                                            │
│  Task Queue ← Assignments ← Policies ← Missions           │
│       │                                                    │
│       ├── Idle Detection (heartbeat + activity monitor)    │
│       ├── Auto-Assignment (round-robin / skill-based)      │
│       ├── Priority Queue (urgent → normal → background)    │
│       └── Dependency Tracking (task A blocks task B)       │
╠══════════════════════════════════════════════════════════╣
│              AI AGENT CONNECTORS                           │
│                                                            │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐            │
│  │ Claude Code │ │  Gemini    │ │  GPT/OpenAI│            │
│  │ (CLI + API) │ │ (API+MCP)  │ │ (API+MCP)  │            │
│  └──────┬─────┘ └──────┬─────┘ └──────┬─────┘            │
│         │               │               │                  │
│  ┌──────┴─────┐ ┌──────┴─────┐ ┌──────┴─────┐            │
│  │ Ollama     │ │ Local LLMs │ │ Custom     │            │
│  │ (local)    │ │ (llama.cpp)│ │ Agents     │            │
│  └──────┬─────┘ └──────┬─────┘ └──────┬─────┘            │
│         │               │               │                  │
│         └───────────────┴───────┬───────┘                  │
│                                 │                          │
│                    Unified Agent Interface                  │
│                    • register()                            │
│                    • heartbeat()                           │
│                    • receive_task()                        │
│                    • report_progress()                     │
│                    • complete_task()                       │
│                    • watch_file()                          │
│                    • watch_shell()                         │
│                    • query_lfi()                           │
╠══════════════════════════════════════════════════════════╣
│              LFI (The Watcher)                             │
│                                                            │
│  LFI observes ALL agent activity and learns:               │
│                                                            │
│  • Watches file changes agents make → learns code patterns │
│  • Watches shell output → learns command patterns          │
│  • Watches task success/failure → learns what works        │
│  • Watches conversations → learns communication patterns   │
│  • Watches errors → learns failure modes                   │
│  • Watches user corrections → learns preferences           │
│                                                            │
│  LFI then:                                                 │
│  • Suggests better task assignments based on agent skills  │
│  • Detects when agents are stuck and suggests solutions    │
│  • Provides RAG context to agents from 57M+ facts          │
│  • Trains on successful patterns → improves over time      │
│  • Flags compliance violations in real-time                │
│  • Generates training data FROM agent interactions         │
╠══════════════════════════════════════════════════════════╣
│              MANAGEMENT LAYER                              │
│                                                            │
│  Roles:     Developer, Reviewer, Tester, Deployer, Admin   │
│  Groups:    Frontend Team, Backend Team, Security Team      │
│  Projects:  PlausiDen AI, Sacred.Vote, Sentinel            │
│  Missions:  "Ship v1.0", "Fix all security bugs",          │
│             "Reach 100M facts", "Pass HIPAA audit"         │
│  Policies:  Rate limits, cost caps, allowed operations,    │
│             required reviews, mandatory tests               │
│  Compliance: AVP-2, GDPR, SOX, HIPAA (per project)        │
│                                                            │
│  Queue: [Idle AIs] → [Pending Tasks] → [Auto-Assign]     │
│         Idle detection via heartbeat + activity monitor     │
│         If AI idle >30s, assign next priority task          │
╠══════════════════════════════════════════════════════════╣
│              MCP INTEGRATIONS                              │
│                                                            │
│  MCP Servers (built-in):                                   │
│  • filesystem — read/write/watch files                     │
│  • shell — execute commands, stream output                 │
│  • git — commits, diffs, branches                          │
│  • sqlite — query brain.db, orchestrator.db                │
│  • web — fetch URLs, search                                │
│  • ollama — local model inference                          │
│                                                            │
│  MCP Clients (connecting to external):                     │
│  • GitHub — issues, PRs, actions                           │
│  • Slack/Discord — team communication                      │
│  • Jira/Linear — project management                        │
│  • Docker — container management                           │
│  • Kubernetes — cluster orchestration                      │
│  • AWS/GCP — cloud resources                               │
╠══════════════════════════════════════════════════════════╣
│              FILE + SHELL WATCHERS                          │
│                                                            │
│  inotify watchers on:                                      │
│  • Source code directories → detect changes → notify LFI   │
│  • Config files → detect drift → alert                     │
│  • Log files → detect errors → auto-assign fix task        │
│                                                            │
│  Shell session capture:                                     │
│  • tmux/screen session recording                           │
│  • Command history analysis                                │
│  • Error pattern detection                                 │
│  • Success pattern learning                                │
│  • All captured as LFI training data                       │
╚══════════════════════════════════════════════════════════╝
```

## Key Principles

1. **Every AI is a worker** — Claude, Gemini, GPT, Ollama, custom. Same interface.
2. **LFI watches everything** — learns from all agent activity, files, shells.
3. **No idle agents** — heartbeat detection auto-assigns work from the queue.
4. **Policy-driven** — rate limits, cost caps, allowed operations per role.
5. **Mission-oriented** — define goals, let the platform decompose into tasks.
6. **Training is continuous** — every interaction becomes training data for LFI.
7. **MCP for everything** — standardized tool access across all agents.

## Build Order

Phase 1 (now): Orchestrator + Claude fleet management
Phase 2: Ollama + local model connectors  
Phase 3: External API connectors (OpenAI, Gemini)
Phase 4: MCP server/client infrastructure
Phase 5: File/shell watchers + LFI learning integration
Phase 6: Mission engine + policy enforcement
Phase 7: Multi-project management + team roles
