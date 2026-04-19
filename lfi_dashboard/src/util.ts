// Shared formatting + browser helpers. Keep this file React-free so any
// component can import without pulling in the React tree.

// c2-433 / task 223: haptic tick for mobile interactions. Wraps navigator.
// vibrate behind a try/catch since (a) iOS Safari has no Vibration API, (b)
// some Android browsers gate it behind a user-activation check that throws
// from non-gesture contexts, (c) some users disable it via OS settings.
// Failing silently is the right behavior — vibration is a *bonus* signal,
// never a primary one. Default 15 ms is the "tick" pattern; pass an array
// for richer patterns (e.g. [10,40,10] for a confirm-double-tap).
export const hapticTick = (ms: number | number[] = 15): void => {
  try { (navigator as any).vibrate?.(ms); } catch { /* unsupported */ }
};

// c2-433 / task 284: humanize a millisecond duration. Used by the
// response-duration chip on assistant messages + the tool-message footer.
//   <1s  → "Nms"
//   <60s → "N.Ns"
//   else → "Nm Ns"
// Compact + scannable. Callers stash the exact ms in a tooltip if needed.
export const formatDuration = (ms: number): string => {
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60_000) return `${(ms / 1000).toFixed(1)}s`;
  return `${Math.floor(ms / 60_000)}m ${Math.floor((ms % 60_000) / 1000)}s`;
};

// c2-433 / task 245: flash a yellow box-shadow ring on the message bubble
// matching the given id. Used by branch-jump, chat-search jump, and the
// provenance system-message append — anywhere the user is sent to a
// specific message and we want a visual landing cue. The 80ms initial
// delay lets Virtuoso paint the row after a scrollToIndex; the 1100ms
// hold + 200ms transition matches the prior inline implementations.
//
// `colour` defaults to a usable yellow (#fbbf24). Caller can pass any CSS
// color string (e.g. C.yellow). Targets [data-msg-id="N"] which the chat
// list wrapper assigns per render. No-op when no element is found.
export const flashMessageById = (msgId: number | string, colour: string = '#fbbf24'): void => {
  window.setTimeout(() => {
    const el = document.querySelector(`[data-msg-id="${msgId}"]`) as HTMLElement | null;
    if (!el) return;
    const orig = el.style.boxShadow;
    el.style.boxShadow = `0 0 0 3px ${colour}`;
    el.style.transition = 'box-shadow 0.2s';
    window.setTimeout(() => { el.style.boxShadow = orig; }, 1100);
  }, 80);
};

// 56_750_622 → "56.7M", 168 → "168", 3945 → "3.9K", null/NaN → "—".
export const compactNum = (n: number | null | undefined): string => {
  if (n == null || Number.isNaN(n)) return '—';
  const abs = Math.abs(n);
  if (abs >= 1e9) return (n / 1e9).toFixed(1).replace(/\.0$/, '') + 'B';
  if (abs >= 1e6) return (n / 1e6).toFixed(1).replace(/\.0$/, '') + 'M';
  if (abs >= 1e3) return (n / 1e3).toFixed(1).replace(/\.0$/, '') + 'K';
  return String(n);
};

// RAM formatter: switches MB → GB at the obvious threshold.
export const formatRam = (mb: number): { value: string; unit: string } => {
  if (mb <= 0) return { value: '0', unit: 'MB' };
  if (mb >= 1024) return { value: (mb / 1024).toFixed(1), unit: 'GB' };
  return { value: String(mb), unit: 'MB' };
};

// Epoch-ms → "HH:MM" in the user's locale.
export const formatTime = (ts: number): string =>
  new Date(ts).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

// Epoch-ms → day-bucket label ("Today", "Yesterday", "Apr 15"). Used for the
// sticky day separators in the chat list.
export const formatDayBucket = (ts: number, now = Date.now()): string => {
  const d = new Date(ts);
  const today = new Date(now);
  const startOfDay = (x: Date) => new Date(x.getFullYear(), x.getMonth(), x.getDate()).getTime();
  const diffDays = Math.round((startOfDay(today) - startOfDay(d)) / 86400000);
  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Yesterday';
  if (diffDays < 7) return d.toLocaleDateString([], { weekday: 'long' });
  return d.toLocaleDateString([], { month: 'short', day: 'numeric', year: today.getFullYear() === d.getFullYear() ? undefined : 'numeric' });
};

// Epoch-ms → relative-time string ("3m ago", "2h ago", "Apr 17"). Thresholds
// tuned so a 'right now' feels responsive but older items fall back to dates.
export const formatRelative = (ts: number, now = Date.now()): string => {
  const diffSec = Math.max(0, Math.floor((now - ts) / 1000));
  if (diffSec < 10) return 'just now';
  if (diffSec < 60) return `${diffSec}s ago`;
  const diffMin = Math.floor(diffSec / 60);
  if (diffMin < 60) return `${diffMin}m ago`;
  const diffHr = Math.floor(diffMin / 60);
  if (diffHr < 24) return `${diffHr}h ago`;
  const diffDay = Math.floor(diffHr / 24);
  if (diffDay < 7) return `${diffDay}d ago`;
  return new Date(ts).toLocaleDateString([], { month: 'short', day: 'numeric' });
};

// Strip common markdown syntax to plain text. Used by the message copy
// button when the user Shift-clicks ("copy as plain"). Not a full markdown
// parser — just the syntax the renderer supports (bold, italic, code,
// links, fences, list markers, heading markers, blockquote, hr).
export const stripMarkdown = (src: string): string => {
  return src
    // Fenced code → keep inner code
    .replace(/```[a-zA-Z0-9_+-]*\n([\s\S]*?)```/g, (_m, body) => body)
    // Inline code
    .replace(/`([^`\n]+)`/g, '$1')
    // Bold **x** / __x__
    .replace(/\*\*([^*\n]+)\*\*/g, '$1')
    .replace(/__([^_\n]+)__/g, '$1')
    // Italic *x* / _x_
    .replace(/\*([^*\n]+)\*/g, '$1')
    .replace(/_([^_\n]+)_/g, '$1')
    // Strikethrough ~~x~~
    .replace(/~~([^~\n]+)~~/g, '$1')
    // Links [text](url) → "text (url)"
    .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '$1 ($2)')
    // Images ![alt](url) → "alt"
    .replace(/!\[([^\]]*)\]\([^)]*\)/g, '$1')
    // Heading markers at line start (# ## ###)
    .replace(/^#{1,6}\s+/gm, '')
    // Blockquote markers
    .replace(/^>\s?/gm, '')
    // Horizontal rules
    .replace(/^[-*_]{3,}\s*$/gm, '')
    // Bullet list markers
    .replace(/^[\s]*[-*]\s+/gm, '• ')
    // Numbered list markers (keep number + dot)
    .replace(/^(\s*)(\d+\.)\s+/gm, '$1$2 ')
    .trim();
};

// c2-265: platform modifier detection, shared across components so all
// shortcut chips render \u2318 on mac and Ctrl elsewhere. navigator.platform
// is deprecated but still the most reliable signal for this cosmetic choice
// (navigator.userAgentData isn't universally available in older Chromium).
// Evaluated once at module load — it's safe to cache because the platform
// doesn't change across a single session.
export const IS_MAC: boolean =
  typeof navigator !== 'undefined' &&
  /Mac|iPhone|iPad|iPod/.test(navigator.platform || '');

// Convenience for rendering a single-char modifier — e.g. mod() + 'N'.
export const mod = (): string => (IS_MAC ? '\u2318' : 'Ctrl');

// c2-417: separator between the modifier and the key. Mac's ⌘ glyph reads
// as a distinct graphic so "⌘K" is unambiguous; "CtrlK" reads as a word.
// Non-Mac renders with a '+' so users see "Ctrl+K".
export const modSep = (): string => (IS_MAC ? '' : '+');

// Compose mod+separator+key in one call. Replaces the common pattern
// `{mod()}{key}` which produced "CtrlK" on non-Mac.
export const modKey = (key: string): string => `${mod()}${modSep()}${key}`;

// Expand '$mod' placeholders in a shortcut descriptor string. Used by the
// Command Palette items and anywhere else that stores platform-agnostic
// shortcut strings in data.
export const formatShortcut = (s: string): string => s.replace(/\$mod/g, mod());

// Clipboard write with an execCommand fallback for browsers that block the
// async Clipboard API (e.g. insecure-context). Never throws.
export const copyToClipboard = async (text: string): Promise<void> => {
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    const ta = document.createElement('textarea');
    ta.value = text; document.body.appendChild(ta); ta.select();
    try { document.execCommand('copy'); } catch { /* no-op */ }
    ta.remove();
  }
};

// Serialise a conversation to a markdown file and trigger a browser download.
// Pure beyond the DOM side effects; kept dependency-free so tests can stub them.
export interface ExportableMessage { role: string; content: string; timestamp: number }
export interface ExportableConversation { title: string; messages: ExportableMessage[] }
export const exportConversationMd = (convo: ExportableConversation): void => {
  let md = `# ${convo.title}\n\nExported ${new Date().toISOString()}\n\n---\n\n`;
  for (const m of convo.messages) {
    const ts = new Date(m.timestamp).toLocaleString();
    if (m.role === 'user') md += `**You** (${ts}):\n${m.content}\n\n`;
    else if (m.role === 'assistant') md += `**PlausiDen AI** (${ts}):\n${m.content}\n\n`;
    else if (m.role === 'system') md += `*[system: ${m.content}]*\n\n`;
    else if (m.role === 'web') md += `**Web Search:**\n${m.content}\n\n`;
  }
  const blob = new Blob([md], { type: 'text/markdown' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${convo.title.replace(/[^a-zA-Z0-9]/g, '_').slice(0, 40)}.md`;
  document.body.appendChild(a); a.click(); a.remove();
  URL.revokeObjectURL(url);
};

// c2-395 / task 197: plain-text export. Role prefix + blank line between
// turns, no markdown. Targets analysis pipelines / diff tools that prefer
// untagged prose. Uses stripMarkdown on assistant turns so code fences and
// formatting don't contaminate the output.
export const exportConversationTxt = (convo: ExportableConversation): void => {
  let txt = `${convo.title}\nExported: ${new Date().toISOString()}\n${'='.repeat(60)}\n\n`;
  for (const m of convo.messages) {
    const ts = new Date(m.timestamp).toLocaleString();
    const who = m.role === 'user' ? 'You'
      : m.role === 'assistant' ? 'PlausiDen AI'
      : m.role === 'system' ? 'System'
      : m.role === 'web' ? 'Web Search'
      : m.role;
    const body = m.role === 'assistant' ? stripMarkdown(m.content) : m.content;
    txt += `[${who}] ${ts}\n${body}\n\n`;
  }
  const blob = new Blob([txt], { type: 'text/plain;charset=utf-8' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${convo.title.replace(/[^a-zA-Z0-9]/g, '_').slice(0, 40)}.txt`;
  document.body.appendChild(a); a.click(); a.remove();
  URL.revokeObjectURL(url);
};

// Open a print-friendly window for a single conversation and trigger the
// browser's print dialog. Users pick "Save as PDF" from the dialog. No PDF
// library bundled — the browser handles layout + fonts + paging.
export const exportConversationPdf = (convo: ExportableConversation): void => {
  const title = convo.title.replace(/[<>]/g, '');
  const escape = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  const rows = convo.messages.map(m => {
    const ts = new Date(m.timestamp).toLocaleString();
    const who = m.role === 'user' ? 'You' : m.role === 'assistant' ? 'PlausiDen AI' : m.role === 'system' ? 'System' : m.role;
    return `<div class="msg ${m.role}"><div class="meta"><strong>${who}</strong> <span class="ts">${ts}</span></div><div class="body">${escape(m.content)}</div></div>`;
  }).join('');
  const html = `<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>${escape(title)}</title>
<style>
  @page { size: Letter; margin: 0.6in; }
  body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; color: #111827; font-size: 12pt; line-height: 1.55; margin: 0; background: #ffffff; }
  h1 { font-size: 18pt; margin: 0 0 4pt; }
  .exported { color: #6b7280; font-size: 9pt; margin-bottom: 18pt; }
  .msg { margin-bottom: 14pt; page-break-inside: avoid; }
  .msg .meta { font-size: 9pt; color: #6b7280; margin-bottom: 4pt; }
  .msg .meta strong { color: #111827; }
  .msg .body { white-space: pre-wrap; word-break: break-word; }
  .msg.user .body { padding: 8pt 10pt; background: #eff6ff; border-left: 3pt solid #2563eb; }
  .msg.assistant .body { padding: 8pt 10pt; background: #f5f6f8; border-left: 3pt solid #16a34a; }
  .msg.system .body { font-style: italic; color: #6b7280; }
  .msg.web .body { padding: 8pt 10pt; background: #fef9c3; border-left: 3pt solid #ca8a04; }
  @media print { body { -webkit-print-color-adjust: exact; print-color-adjust: exact; } }
</style></head><body>
<h1>${escape(title)}</h1>
<div class="exported">Exported ${new Date().toLocaleString()} · ${convo.messages.length} messages</div>
${rows}
</body></html>`;
  const w = window.open('', '_blank', 'noopener');
  if (!w) {
    // Popup blocked — user needs to allow popups for this domain.
    alert('PDF export requires popups to be enabled for this site.');
    return;
  }
  w.document.open();
  w.document.write(html);
  w.document.close();
  // Wait for layout before firing print.
  setTimeout(() => { try { w.focus(); w.print(); } catch { /* ignore */ } }, 300);
};

// c2-421 / task 203: print-friendly grade-report export. Same pattern as
// exportConversationPdf — renders an HTML doc in a new window + triggers
// the browser's print dialog so the user can "Save as PDF". No library
// bundled; the browser paginates + fonts it. data is the shape returned
// by /api/admin/dashboard (overview + score + training + domains), all
// fields optional so partial backend responses still render cleanly.
export interface GradeReportData {
  overview?: { total_facts?: number; total_sources?: number; adversarial_facts?: number; total_training_pairs?: number };
  quality?: { average?: number; high_quality_count?: number; low_quality_count?: number };
  training?: { sessions?: number; total_tested?: number; total_correct?: number; pass_rate?: number };
  score?: { accuracy_score?: number; grade?: string; breakdown?: { quality?: number; adversarial?: number; coverage?: number; training?: number } };
  domains?: Array<{ domain: string; count: number }>;
}
export const exportGradeReportPdf = (data: GradeReportData): void => {
  const escape = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  const fmtNum = (n: number | undefined) =>
    typeof n === 'number' && isFinite(n) ? n.toLocaleString() : '—';
  const fmtPct = (n: number | undefined) => {
    if (typeof n !== 'number' || !isFinite(n)) return '—';
    const v = n <= 1.5 ? n * 100 : n;
    return `${v.toFixed(1)}%`;
  };
  const grade = data.score?.grade || '—';
  const accuracy = typeof data.score?.accuracy_score === 'number'
    ? `${(data.score.accuracy_score * 100).toFixed(1)}%` : '—';
  const domains = [...(data.domains || [])].sort((a, b) => b.count - a.count);
  const maxCount = Math.max(...domains.map(d => d.count), 1);
  const domainRows = domains.slice(0, 25).map(d => {
    const pct = Math.round((d.count / maxCount) * 100);
    return `<div class="drow"><span class="dname">${escape(d.domain)}</span>`
      + `<div class="dbar"><div class="dfill" style="width:${pct}%"></div></div>`
      + `<span class="dcount">${fmtNum(d.count)}</span></div>`;
  }).join('');
  const breakdown = data.score?.breakdown || {};
  const breakdownRows = (['quality', 'adversarial', 'coverage', 'training'] as const)
    .map(k => `<tr><td>${k}</td><td class="r">${fmtPct(breakdown[k])}</td></tr>`).join('');
  const html = `<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>PlausiDen grade report</title>
<style>
  @page { size: Letter; margin: 0.6in; }
  body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; color: #111827; font-size: 11pt; line-height: 1.5; margin: 0; background: #fff; }
  h1 { font-size: 20pt; margin: 0 0 4pt; }
  h2 { font-size: 13pt; margin: 20pt 0 6pt; border-bottom: 1pt solid #e5e7eb; padding-bottom: 3pt; }
  .exported { color: #6b7280; font-size: 9pt; margin-bottom: 18pt; }
  .grade-hero { display: flex; gap: 20pt; align-items: baseline; margin-bottom: 12pt; }
  .grade-hero .grade { font-size: 48pt; font-weight: 800; color: #2563eb; line-height: 1; }
  .grade-hero .accuracy { font-size: 14pt; color: #374151; }
  .stat-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10pt; margin-bottom: 12pt; }
  .stat { padding: 8pt 10pt; background: #f5f6f8; border-left: 3pt solid #2563eb; }
  .stat .label { font-size: 8pt; text-transform: uppercase; letter-spacing: 0.06em; color: #6b7280; }
  .stat .value { font-size: 14pt; font-weight: 700; margin-top: 2pt; font-variant-numeric: tabular-nums; }
  table { border-collapse: collapse; width: 100%; font-size: 10pt; }
  td, th { padding: 4pt 8pt; border-bottom: 1pt solid #e5e7eb; text-align: left; }
  .r { text-align: right; font-variant-numeric: tabular-nums; }
  .drow { display: flex; align-items: center; gap: 8pt; margin-bottom: 3pt; }
  .dname { width: 140pt; font-size: 9pt; color: #374151; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .dbar { flex: 1; height: 8pt; background: #f3f4f6; border-radius: 2pt; overflow: hidden; }
  .dfill { height: 100%; background: linear-gradient(90deg, #2563eb, #8b5cf6); }
  .dcount { width: 60pt; text-align: right; font-size: 9pt; font-variant-numeric: tabular-nums; color: #6b7280; }
  @media print { body { -webkit-print-color-adjust: exact; print-color-adjust: exact; } }
</style></head><body>
<h1>Grade Report</h1>
<div class="exported">PlausiDen AI &middot; Generated ${new Date().toLocaleString()}</div>
<div class="grade-hero">
  <div class="grade">${escape(grade)}</div>
  <div class="accuracy">Accuracy: <strong>${escape(accuracy)}</strong></div>
</div>
<h2>Key metrics</h2>
<div class="stat-grid">
  <div class="stat"><div class="label">Total facts</div><div class="value">${fmtNum(data.overview?.total_facts)}</div></div>
  <div class="stat"><div class="label">Sources</div><div class="value">${fmtNum(data.overview?.total_sources)}</div></div>
  <div class="stat"><div class="label">Training pairs</div><div class="value">${fmtNum(data.overview?.total_training_pairs)}</div></div>
  <div class="stat"><div class="label">Adversarial</div><div class="value">${fmtNum(data.overview?.adversarial_facts)}</div></div>
  <div class="stat"><div class="label">Pass rate</div><div class="value">${fmtPct(data.training?.pass_rate)}</div></div>
  <div class="stat"><div class="label">Tested</div><div class="value">${fmtNum(data.training?.total_tested)}</div></div>
  <div class="stat"><div class="label">Correct</div><div class="value">${fmtNum(data.training?.total_correct)}</div></div>
  <div class="stat"><div class="label">Avg quality</div><div class="value">${typeof data.quality?.average === 'number' ? data.quality.average.toFixed(2) : '—'}</div></div>
</div>
${breakdownRows ? `<h2>Score breakdown</h2><table><thead><tr><th>Component</th><th class="r">Value</th></tr></thead><tbody>${breakdownRows}</tbody></table>` : ''}
${domainRows ? `<h2>Top domains (${Math.min(25, domains.length)} of ${domains.length})</h2>${domainRows}` : ''}
</body></html>`;
  const w = window.open('', '_blank', 'noopener');
  if (!w) {
    alert('PDF export requires popups to be enabled for this site.');
    return;
  }
  w.document.open();
  w.document.write(html);
  w.document.close();
  setTimeout(() => { try { w.focus(); w.print(); } catch { /* ignore */ } }, 300);
};

// Full-backup export. Bundles all conversations, settings, and a schema
// version into a single JSON blob the user can re-import (or version-control
// outside the browser). Schema version lets the importer reject incompatible
// future formats safely.
export interface FullBackup {
  schemaVersion: 1;
  exportedAt: string;
  conversations: unknown[];
  settings: unknown;
}
export const exportAllAsJson = (conversations: unknown[], settings: unknown): void => {
  const payload: FullBackup = {
    schemaVersion: 1,
    exportedAt: new Date().toISOString(),
    conversations,
    settings,
  };
  const blob = new Blob([JSON.stringify(payload, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  const stamp = new Date().toISOString().slice(0, 19).replace(/[:T]/g, '-');
  a.download = `plausiden-backup-${stamp}.json`;
  document.body.appendChild(a); a.click(); a.remove();
  URL.revokeObjectURL(url);
};

// Auto-title a conversation from its message list. Used once, but extracted so
// the heuristic (questions preserved, first clause preferred, 52-char cap) is
// easy to test + tune without scrolling through App.tsx.
export const smartTitle = (msgs: Array<{ role: string; content: string }>): string => {
  const firstUser = msgs.find(m => m.role === 'user');
  if (!firstUser) return 'New chat';
  const raw = firstUser.content.replace(/\s+/g, ' ').trim();
  if (/\?\s*$/.test(raw)) return raw.slice(0, 52);
  const words = raw.split(' ');
  if (words.length <= 7) return raw.slice(0, 52);
  const clause = raw.split(/[.,;!?]/)[0].trim();
  if (clause.length >= 6 && clause.length <= 60) return clause;
  return raw.slice(0, 52);
};

// Summarise disk pressure from /api/system/info byte counts. Returns null when
// the inputs aren't usable. Used by sidebar banner + status row so the pct +
// GB conversion stays in one place; callers pick their own thresholds.
export interface DiskPressure { usedPct: number; freeGb: number }
export const diskPressure = (freeBytes?: number, totalBytes?: number): DiskPressure | null => {
  if (!freeBytes || !totalBytes || totalBytes <= 0) return null;
  return {
    usedPct: ((totalBytes - freeBytes) / totalBytes) * 100,
    freeGb: freeBytes / (1024 ** 3),
  };
};
