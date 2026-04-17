import React, { useEffect, useState } from 'react';
import { T } from './tokens';

// Training Domains panel (c0-016 B3). Fetches /api/admin/training/domains on
// mount + button-press and renders a sortable-by-count table. Color-coded by
// fact count: green >10K, yellow 1K-10K, red <1K.
//
// Kept presentational-ish: parent owns the host, we own the fetch + render.
// Reason: the panel is hidden unless user expands it, so there's no point
// polling at the app level.

interface DomainRow {
  domain: string;
  facts: number;
  avg_quality?: number;
  avg_length?: number;
}

export interface DomainsPanelProps {
  C: any;
  host: string;
}

export const DomainsPanel: React.FC<DomainsPanelProps> = ({ C, host }) => {
  const [rows, setRows] = useState<DomainRow[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const load = async () => {
    setLoading(true);
    setError(null);
    try {
      const ctrl = new AbortController();
      const to = setTimeout(() => ctrl.abort(), 8000);
      const res = await fetch(`http://${host}:3000/api/admin/training/domains`, { signal: ctrl.signal });
      clearTimeout(to);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      const arr: DomainRow[] = Array.isArray(data?.domains) ? data.domains : Array.isArray(data) ? data : [];
      setRows(arr.sort((a, b) => b.facts - a.facts));
    } catch (e: any) {
      setError(String(e?.message || e || 'fetch failed'));
    } finally {
      setLoading(false);
    }
  };
  // Lazy — first load triggered by button click, not mount. Admin panel is
  // already a heavy area; don't pay the cost until the user asks.
  const countColor = (n: number) => n > 10000 ? C.green : n > 1000 ? C.yellow : C.red;
  const thStyle = {
    textAlign: 'left' as const, padding: `${T.spacing.xs} ${T.spacing.sm}`,
    fontWeight: T.typography.weightBold,
    background: C.bgInput, borderBottom: `1px solid ${C.borderSubtle}`,
  };
  const tdStyle = {
    padding: `${T.spacing.xs} ${T.spacing.sm}`, textAlign: 'right' as const,
    fontFamily: 'ui-monospace, monospace',
  };
  return (
    <div style={{ marginTop: T.spacing.md }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: T.spacing.sm }}>
        <div style={{
          fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
          color: C.textMuted, textTransform: 'uppercase',
          letterSpacing: T.typography.trackingLoose,
        }}>
          Training domains
        </div>
        <button onClick={load} disabled={loading}
          style={{
            padding: `${T.spacing.xs} ${T.spacing.sm}`,
            fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
            background: C.accentBg, border: `1px solid ${C.accentBorder}`,
            color: C.accent, borderRadius: T.radii.md,
            cursor: loading ? 'wait' : 'pointer',
            fontFamily: 'inherit', textTransform: 'uppercase',
          }}>{loading ? 'Loading…' : rows ? 'Refresh' : 'Load'}</button>
      </div>
      {error && (
        <div role='alert' style={{
          fontSize: T.typography.sizeXs, color: C.red, background: C.redBg,
          border: `1px solid ${C.redBorder}`, borderRadius: T.radii.md,
          padding: `${T.spacing.xs} ${T.spacing.sm}`, marginBottom: T.spacing.xs,
        }}>{error}</div>
      )}
      {rows && rows.length === 0 && !error && (
        <div style={{
          fontSize: T.typography.sizeXs, color: C.textDim,
          padding: T.spacing.sm, textAlign: 'center',
        }}>
          No domain telemetry yet.
        </div>
      )}
      {rows && rows.length > 0 && (
        <div style={{ overflowX: 'auto', border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.md }}>
          <table style={{ borderCollapse: 'collapse', width: '100%', fontSize: T.typography.sizeXs, color: C.text }}>
            <thead>
              <tr>
                <th style={thStyle}>Domain</th>
                <th style={{ ...thStyle, textAlign: 'right' }}>Facts</th>
                <th style={{ ...thStyle, textAlign: 'right' }}>Quality</th>
                <th style={{ ...thStyle, textAlign: 'right' }}>Len</th>
              </tr>
            </thead>
            <tbody>
              {rows.map((r, i) => (
                <tr key={i} style={{ background: i % 2 === 0 ? 'transparent' : 'rgba(255,255,255,0.02)' }}>
                  <td style={{ ...tdStyle, textAlign: 'left', fontWeight: T.typography.weightSemibold, fontFamily: 'inherit' }}>{r.domain}</td>
                  <td style={{ ...tdStyle, color: countColor(r.facts), fontWeight: T.typography.weightBold }}>
                    {r.facts.toLocaleString()}
                  </td>
                  <td style={{ ...tdStyle, color: C.textMuted }}>
                    {typeof r.avg_quality === 'number' ? r.avg_quality.toFixed(2) : '—'}
                  </td>
                  <td style={{ ...tdStyle, color: C.textMuted }}>
                    {typeof r.avg_length === 'number' ? r.avg_length.toFixed(0) : '—'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};
