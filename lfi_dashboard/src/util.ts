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
