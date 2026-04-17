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
