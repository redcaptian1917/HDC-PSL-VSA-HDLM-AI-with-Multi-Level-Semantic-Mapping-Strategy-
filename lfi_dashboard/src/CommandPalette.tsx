import React from 'react';
import { T } from './tokens';
import { formatShortcut } from './util';

// Cmd+K command palette. Parent builds the full items list (items capture lots
// of closure state like tier handlers, conversations, skills) and passes it
// in; component owns the filtering, keyboard navigation, and render.

export interface CmdPaletteItem {
  id: string;
  label: string;
  hint: string;
  group: string;
  onRun: () => void;
  // c2-243 / #104: optional keyboard hint. Use '$mod' as a placeholder for
  // the platform modifier ('⌘' on mac, 'Ctrl' elsewhere) and '+' as the
  // separator, e.g. '$mod+N', '$mod+Shift+D'. Single-key hints like '?' pass
  // through verbatim. c2-265: formatShortcut now lives in util.ts.
  shortcut?: string;
  // c2-422 / task 209: extra text scored by the fuzzy matcher but never
  // rendered. Conversation items populate this with recent-message
  // excerpts so "find that chat about X" works without X being in the
  // title.
  searchBody?: string;
}

export interface CommandPaletteProps {
  C: any;
  isMobile: boolean;
  items: CmdPaletteItem[];
  query: string;
  setQuery: (s: string) => void;
  index: number;
  setIndex: React.Dispatch<React.SetStateAction<number>>;
  onClose: () => void;
  onItemRun?: (id: string) => void;
  // Map of item.id → run-count. When empty query, items with higher counts
  // bubble to the top; otherwise recency boosts fuzzy match score.
  recency?: Record<string, number>;
}

export const CommandPalette: React.FC<CommandPaletteProps> = ({
  C, isMobile, items, query, setQuery, index, setIndex, onClose, onItemRun, recency,
}) => {
  const q = query.trim().toLowerCase();
  const score = (t: string) => {
    if (!q) return 1;
    const lt = t.toLowerCase();
    if (lt === q) return 1000;
    if (lt.startsWith(q)) return 500;
    if (lt.includes(q)) return 200;
    // fuzzy subsequence
    let j = 0;
    for (let i = 0; i < lt.length && j < q.length; i++) if (lt[i] === q[j]) j++;
    return j === q.length ? 50 : 0;
  };
  // Recency boost: log-scaled, capped to avoid dominating pure matches.
  const recencyBoost = (id: string): number => {
    const n = recency?.[id] ?? 0;
    if (n <= 0) return 0;
    return Math.min(40, Math.log2(n + 1) * 10);
  };
  // c2-403 / task 202: when the query is empty, surface a "Recent" group
  // at the top with the top-5 most-run commands. Clones the items with
  // group='Recent' so the grouping render picks them up; the originals
  // still appear in their natural group below for discoverability.
  // c2-422 / task 209: body text gets a smaller score weight (0.25) than
  // hint (0.4) which is smaller than label (1.0). This preserves title-
  // match precedence while letting a search like "psl" find a chat whose
  // title is "Monday notes" but whose body mentions PSL.
  const scored = items
    .map(it => ({ it, s: score(it.label) + score(it.hint) * 0.4 + score(it.searchBody ?? '') * 0.25 + recencyBoost(it.id) }))
    .filter(x => x.s > 0)
    .sort((a, b) => b.s - a.s)
    .slice(0, 24)
    .map(x => x.it);
  const filtered = (() => {
    if (q) return scored;
    const topRecent = items
      .filter(it => (recency?.[it.id] ?? 0) > 0)
      .sort((a, b) => (recency?.[b.id] ?? 0) - (recency?.[a.id] ?? 0))
      .slice(0, 5)
      .map(it => ({ ...it, group: 'Recent' }));
    if (topRecent.length === 0) return scored;
    return [...topRecent, ...scored];
  })();
  const runSelected = () => {
    const picked = filtered[index];
    if (!picked) return;
    picked.onRun();
    onItemRun?.(picked.id);
    onClose();
  };

  return (
    <div onClick={onClose}
      style={{
        position: 'fixed', inset: 0, zIndex: T.z.palette,
        background: 'rgba(0,0,0,0.55)',
        display: 'flex', alignItems: 'flex-start', justifyContent: 'center',
        padding: isMobile ? T.spacing.lg : `10vh ${T.spacing.lg}`,
      }}>
      <div onClick={(e) => e.stopPropagation()}
        style={{
          width: '100%', maxWidth: '560px',
          background: C.bgCard, border: `1px solid ${C.border}`,
          borderRadius: T.radii.xl, boxShadow: T.shadows.modal,
          overflow: 'hidden', display: 'flex', flexDirection: 'column',
        }}>
        <input autoFocus
          role='combobox'
          aria-expanded='true'
          aria-controls='lfi-cmd-listbox'
          aria-activedescendant={filtered[index] ? `lfi-cmd-opt-${filtered[index].group}-${filtered[index].id}` : undefined}
          aria-label='Type a command'
          autoComplete='off'
          autoCorrect='off'
          autoCapitalize='off'
          spellCheck={false}
          value={query}
          onChange={(e) => { setQuery(e.target.value); setIndex(0); }}
          onKeyDown={(e) => {
            if (e.key === 'ArrowDown') { e.preventDefault(); setIndex(i => Math.min(i + 1, filtered.length - 1)); }
            else if (e.key === 'ArrowUp') { e.preventDefault(); setIndex(i => Math.max(i - 1, 0)); }
            else if (e.key === 'Enter') { e.preventDefault(); runSelected(); }
          }}
          placeholder='Type a command or search conversations...'
          style={{
            width: '100%', padding: '16px 18px', background: 'transparent',
            border: 'none', borderBottom: `1px solid ${C.borderSubtle}`,
            outline: 'none', color: C.text, fontFamily: 'inherit',
            fontSize: T.typography.sizeLg, boxSizing: 'border-box',
          }} />

        <div id='lfi-cmd-listbox' role='listbox' aria-label='Command palette results' style={{ maxHeight: '60vh', overflowY: 'auto', padding: '6px' }}>
          {filtered.length === 0 && (
            <div style={{ padding: T.spacing.xl, color: C.textMuted, fontSize: T.typography.sizeMd, textAlign: 'center' }}>
              No matches for "{query}"
            </div>
          )}
          {filtered.map((it, i) => {
            const picked = i === index;
            const prev = i > 0 ? filtered[i - 1].group : null;
            // c2-403 / task 202: compound key + option id so a Recent-group
            // clone doesn't collide with the original's id for React + a11y.
            const rowKey = `${it.group}-${it.id}`;
            return (
              <div key={rowKey}>
                {it.group !== prev && (
                  <div role='presentation' style={{
                    padding: '10px 12px 4px', fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
                    color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose,
                  }}>{it.group}</div>
                )}
                <button
                  id={`lfi-cmd-opt-${rowKey}`}
                  role='option' aria-selected={picked}
                  onClick={() => { setIndex(i); runSelected(); }}
                  onMouseEnter={() => setIndex(i)}
                  style={{
                    width: '100%', textAlign: 'left', cursor: 'pointer',
                    padding: '10px 12px', background: picked ? C.accentBg : 'transparent',
                    border: 'none', borderRadius: T.radii.md, fontFamily: 'inherit',
                    color: C.text, display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                  }}>
                  <div style={{ minWidth: 0, overflow: 'hidden' }}>
                    <div style={{ fontSize: '13.5px', fontWeight: 600, color: picked ? C.accent : C.text, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                      {it.label}
                    </div>
                    <div style={{ fontSize: '11.5px', color: C.textMuted, marginTop: '2px', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                      {it.hint}
                    </div>
                  </div>
                  <div style={{ display: 'flex', alignItems: 'center', gap: T.spacing.sm, marginLeft: '10px', flexShrink: 0 }}>
                    {it.shortcut && (
                      <kbd className='lfi-shortcut-chip' style={{
                        fontFamily: T.typography.fontMono,
                        fontSize: '10.5px', color: picked ? C.accent : C.textMuted,
                        background: picked ? 'transparent' : C.bgInput,
                        border: `1px solid ${picked ? C.accentBorder : C.borderSubtle}`,
                        borderRadius: T.radii.sm, padding: '1px 6px',
                        letterSpacing: '0.02em', whiteSpace: 'nowrap',
                      }}>{formatShortcut(it.shortcut)}</kbd>
                    )}
                    {picked && (
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke={C.accent} strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
                        <polyline points="9 18 15 12 9 6"/>
                      </svg>
                    )}
                  </div>
                </button>
              </div>
            );
          })}
        </div>
        <div style={{
          display: 'flex', gap: '14px', padding: '8px 14px',
          borderTop: `1px solid ${C.borderSubtle}`,
          fontSize: T.typography.sizeXs, color: C.textDim,
        }}>
          <span>{'\u2191\u2193'} navigate</span>
          <span>{'\u21B5'} select</span>
          <span>esc close</span>
          <span style={{ marginLeft: 'auto' }}>{filtered.length} of {items.length}</span>
        </div>
      </div>
    </div>
  );
};
