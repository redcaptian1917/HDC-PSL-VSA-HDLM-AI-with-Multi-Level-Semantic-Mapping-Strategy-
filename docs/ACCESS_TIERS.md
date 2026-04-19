# Access tiers — owner / admin / subscriber / user

**Status:** design note, 2026-04-19. Not wired yet. This doc captures the model so future code can tag each surface and so the eventual auth layer has a clear spec.

## Why

PlausiDen ships two products out of the same codebase:

1. **Self-hosted** — operator runs `plausiden-server` on their own hardware. They are everything: owner, admin, user. They want to see substrate telemetry, audit state, drift metrics, orchestrator internals, raw diag logs. **This is the current build.**
2. **Cloud / subscription** — operator (you) runs the backend; subscribers log in and consume the AI. Subscribers should NOT see CPU temp, ingest runs, proof verifier state, audit chain, brain.db backup, etc. They get chat + their own facts + settings.

Same code, different surface depending on who's logged in.

## Tiers

| Tier            | Who                                      | Sees                                                                                           |
|-----------------|------------------------------------------|-----------------------------------------------------------------------------------------------|
| `owner`         | The operator running the server.         | EVERYTHING. Admin console, Fleet, Library trust sliders, Auditorium, Diag, Backup, Drift ops. |
| `admin`         | Operator-delegate with near-full access. | Everything except owner-scoped destructive ops (backup brain.db, revoke owner tokens, etc.).  |
| `subscriber`    | Paying cloud user with elevated quota.   | Chat + teach + browse-own-facts + settings + higher rate limits.                              |
| `user`          | Free-tier cloud user.                    | Chat + teach + browse-own-facts + settings. Basic rate limits.                                |
| `anonymous`     | Not logged in.                           | Welcome / login / marketing.                                                                   |

Default on self-hosted installs: every request is `owner`. The cloud build hydrates the tier from an auth token.

## Surface-by-surface classification

Each entry lists the required tier to SEE it. "Hidden" means UI element is absent, not disabled.

### Top-level views (nav strip)

| View             | Required tier | Notes                                                                   |
|------------------|--------------|-------------------------------------------------------------------------|
| Agora (chat)     | `user`       | All tiers. The core product.                                            |
| Classroom        | split        | Sub-tabs split — see below.                                             |
| Admin            | `admin`      | Hidden for subscriber/user.                                             |
| Fleet            | `owner`      | Orchestrator-level detail. Subscribers never see this.                  |
| Library          | `admin`      | Per-source trust sliders = operator call. Subscribers may later get a read-only "what sources LFI draws from" view, but not the sliders. |
| Auditorium       | `owner`      | AVP-2 audit state. Entirely operator-facing.                            |

### Classroom sub-tabs

| Sub-tab             | Required tier | Notes                                                   |
|---------------------|--------------|---------------------------------------------------------|
| Student Profile     | `user`       | Personal — who LFI thinks the user is.                  |
| Ingestion Control   | `owner`      | Kick corpus runs.                                       |
| Curriculum          | `admin`      | What's being trained on.                                |
| Gradebook           | `admin`      | Pass/fail metrics.                                      |
| Lesson Plans        | `admin`      |                                                         |
| Test Center         | `admin`      | Benchmarks.                                             |
| Report Cards        | `admin`      |                                                         |
| Office Hours        | `user`       | User's own feedback queue.                              |
| Library             | `admin`      |                                                         |
| Ledger              | `admin`      | Contradictions queue.                                   |
| Drift               | `owner`      | System health + Kick-ingest/Encode-HDC/Auto-resolve ops.|
| Ingest Runs         | `owner`      |                                                         |

### Admin sub-tabs (entire Admin view is `admin`, but some tabs are owner-only)

| Sub-tab    | Required tier | Notes                                                                 |
|------------|--------------|-----------------------------------------------------------------------|
| Dashboard  | `admin`      | Grade card, integrity banner.                                         |
| Backup brain.db card | `owner` | Destructive / data-gravity action.                              |
| Inventory  | `admin`      |                                                                       |
| Domains    | `admin`      |                                                                       |
| Training   | `admin`      |                                                                       |
| Quality    | `admin`      |                                                                       |
| System     | `owner`      | CPU / RAM / disk raw metrics.                                         |
| Logs       | `owner`      |                                                                       |
| Tokens     | `owner`      | Capability token CRUD.                                                |
| Proof      | `admin`      |                                                                       |
| Diag       | `owner`      | Raw ring buffer. Might be `admin` if diag is scrubbed of PII first.   |
| Docs       | `user`       | The user guide is for everyone. (Manager guide might be `admin`+.)    |

### Chat surface fixtures

| Element                     | Required tier | Notes                                                                  |
|-----------------------------|--------------|------------------------------------------------------------------------|
| Chat input                  | `user`       |                                                                        |
| Teach button (refusal CTA)  | `user`       |                                                                        |
| `/teach` slash              | `user`       |                                                                        |
| Cmd+K → Teach LFI           | `user`       |                                                                        |
| Cmd+K → Open Admin → *      | `admin`      |                                                                        |
| Cmd+K → Start tour          | `user`       |                                                                        |
| Cmd+K → Export diag logs    | `owner`      |                                                                        |
| Citation chips `[fact:KEY]` | `user`       | Clickable popover.                                                     |
| Fact popover Verify-now     | `admin`      | Triggers backend proof run.                                            |
| Fact popover Dismiss        | `user`       | User can dismiss a fact LFI applied to their own convo.                |
| Turn-trace diag entries     | `owner`      | Currently fires for everyone; cloud should strip to session only.      |

### Sidebar

| Element                     | Required tier | Notes                                                              |
|-----------------------------|--------------|--------------------------------------------------------------------|
| Substrate Telemetry card    | `owner`      | CPU / RAM / Facts / Sources. Cloud users: drop entirely or show only "facts" count scoped to their knowledge. |
| Conversation list           | `user`       |                                                                    |
| Branch tree `↪`             | `user`       | User's own branches only.                                          |
| Help & guide button         | `user`       |                                                                    |
| Settings gear               | `user`       |                                                                    |
| Connection chip (tri-state) | `owner`      | Subscribers don't care if the server is reconnecting at WS layer.  |
| Stats-age `· Ns` badge      | `owner`      |                                                                    |

### Settings

| Section       | Required tier | Notes                                                                |
|---------------|--------------|----------------------------------------------------------------------|
| Profile       | `user`       | Display name, avatar.                                                |
| Appearance    | `user`       | Theme, font size, compact mode, auto-theme.                          |
| Behavior      | `user`       | Send-on-Enter, notify-on-reply.                                      |
| Developer mode| `owner`      | Gates telemetry + plan panel + workstation ID.                       |
| Eruda toggle  | `owner`      |                                                                      |
| Workspace capacity slider | `owner` | Mutates backend config.                                          |
| Data tab (import/export, destroy) | `user` (export) / `admin` (destroy) | Fine-grain inside the tab. |

## Implementation plan (when we build it)

### Minimum viable

1. Backend issues JWT on login with `tier` claim.
2. Client `useAccessTier()` hook reads the claim (from localStorage or a `/api/me` call).
3. Every `data-access` prop on a surface gates visibility:
   ```tsx
   <div data-access="owner">{/* Drift ops */}</div>
   ```
4. A `<Gate tier="admin">` component that renders children only when `useAccessTier() >= 'admin'`.
5. Nav strip filters itself by tier.

### Nice-to-have

- Feature flags per tier (subscriber might get "unlimited corpus ingest" vs user's 10/day).
- Route guard: deep-linking to an owner-only URL redirects to chat with a gentle "not available on your plan" toast.
- Rate-limit UI: quota meter in sidebar for subscribers.

### Backend co-ordination (claude-0)

Needs `/api/me` returning `{tier, quota, expires_at}`. Every mutation endpoint checks tier server-side — the client gate is UX, not security. Auth token in `Authorization: Bearer <jwt>` header.

## Tagging convention for new code

Until the gate ships, annotate surfaces that WILL need gating:

```tsx
// ACCESS: owner — CPU/RAM readout, subscribers don't need to see this
<SubstrateTelemetry ... />

// ACCESS: admin — trust-slider mutates backend state
<SourceTrustSlider ... />

// ACCESS: user — personal fact browser, all tiers
<KnowledgeBrowser ... />
```

Grep for `ACCESS:` on audit day to find every surface that needs a gate.

## Open questions

- **Fact ownership** — in cloud mode, user A teaches LFI a fact. Should user B benefit? Per-user facts vs shared substrate is a product call, not a technical one. Recommend: shared by default, with an "only on my account" toggle.
- **Telemetry scoping** — `/ws/telemetry` currently broadcasts server metrics. Cloud needs per-user metrics instead (their quota, their request latency). New endpoint or namespaced messages.
- **Backup visibility** — subscribers shouldn't see "Backup brain.db" since brain.db contains everyone's data. Operator-only; owner-only per this table.
- **Audit chain** — subscribers might need read access to entries *they caused* (feedback they gave, facts they taught). Owner sees everything.
