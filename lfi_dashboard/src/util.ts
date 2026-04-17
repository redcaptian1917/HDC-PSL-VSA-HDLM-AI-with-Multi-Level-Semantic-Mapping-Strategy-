// Shared formatting + browser helpers. Keep this file React-free so any
// component can import without pulling in the React tree.

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
