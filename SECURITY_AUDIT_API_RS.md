# Security Audit: api.rs — AVP-2 Tier 3 Adversarial Security
**Auditor:** Claude-1 (The Refiner)
**Date:** 2026-04-17
**File:** /root/LFI/lfi_vsa_core/src/api.rs (2548 lines)
**Findings:** 21 total — 2 Critical, 4 High, 6 Medium, 5 Medium-Low, 4 Low

## CRITICAL (Ship-Blocking)

### C1: Missing Authentication on Admin Endpoints
- **Lines:** 2164, 2184, 2217, 2270, 2401
- **Endpoints:** `/api/admin/training/sessions`, `/domains`, `/accuracy`, `/dashboard`, `/control/:action`
- **Impact:** Anyone on LAN (per CORS whitelist 192.168.1.186) can read training state, accuracy metrics, and control training start/stop
- **Fix:** Add auth middleware consistent with other protected endpoints (e.g., line 607-609 pattern)

### C2: Information Disclosure in Error Messages
- **Lines:** 1190-1191, 1213-1214, 1235-1236, 1275-1276, 2412
- **Impact:** System command stderr (xdotool, scrot, etc.) echoed to client — reveals paths, versions, capabilities
- **Fix:** Apply error scrubbing pattern from line 384-385 everywhere: `"An internal error occurred"`

## HIGH

### H1: No Rate Limiting on Any Endpoint
- `/api/research` spawns multiple web searches per request
- WebSocket `/api/chat` has no per-connection throttle
- `/api/think` is CPU-intensive with no limits
- **Fix:** Implement tower rate-limiting middleware (per-IP token bucket)

### H2: Shell Edge Cases in System Type Handler (Line 1208-1216)
- User text sent to `xdotool type` — special sequences may be interpreted
- **Fix:** Validate/escape input before passing to xdotool

### H3: Clipboard Size (Line 844)
- Accepts 1MB — should cap to 100KB per industry practice

### H4: Experience Learning — Unsanitized Storage (Lines 269-292)
- User input stored directly in LearningSignals without sanitization
- Could cause prompt injection if training pipeline trusts this data

## MEDIUM

### M1-M6: Lock poisoning recovery, deserialization gaps, conversation sync without HMAC, research handler namespace pollution, custom base64 encoder, inconsistent auth patterns

## SECURITY POSITIVES

1. **SQL injection mitigated** — all queries use parameterized `params![]`
2. **CORS properly restricted** — whitelist-based (localhost + LAN IP)
3. **Input size limits enforced** — 16KB chat, 2KB prompts, 64KB opsec
4. **Command injection fixed** — stdin piping to curl (AVP-PASS-13)
5. **System commands use argv arrays** — not shell strings

## RECOMMENDATIONS (Priority Order)

1. Add authentication to all admin endpoints (CRITICAL)
2. Standardize error message scrubbing (CRITICAL)
3. Implement rate limiting middleware (HIGH)
4. Sanitize xdotool input (HIGH)
5. Reduce clipboard size limit (HIGH)
6. Add HMAC to conversation sync (MEDIUM)
7. Implement fuzzing on deserialization endpoints (FUTURE)
