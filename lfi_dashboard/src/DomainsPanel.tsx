import React, { useEffect, useState } from 'react';
import { T } from './tokens';
// c2-380 / BIG #180: shared sortable table.
import { DataTable } from './components';
import type { Column } from './components';

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
      {rows && rows.length > 0 && (() => {
        // c2-380 / BIG #180: sidebar DomainsPanel now uses DataTable. Four
        // sortable columns; facts default desc so the heavy hitters land
        // on top, matching the previous fixed sort. Uses the compact sizeXs
        // cell font since this is an inline sidebar surface.
        const cols: ReadonlyArray<Column<DomainRow>> = [
          {
            id: 'domain', header: 'Domain', align: 'left',
            sortKey: (r) => r.domain.toLowerCase(),
            accessor: (r) => <span style={{ fontWeight: T.typography.weightSemibold }}>{r.domain}</span>,
          },
          {
            id: 'facts', header: 'Facts', align: 'right',
            sortKey: (r) => r.facts,
            accessor: (r) => (
              <span style={{ color: countColor(r.facts), fontWeight: T.typography.weightBold, fontFamily: T.typography.fontMono }}>
                {r.facts.toLocaleString()}
              </span>
            ),
          },
          {
            id: 'quality', header: 'Quality', align: 'right',
            sortKey: (r) => typeof r.avg_quality === 'number' ? r.avg_quality : -1,
            accessor: (r) => (
              <span style={{ color: C.textMuted, fontFamily: T.typography.fontMono }}>
                {typeof r.avg_quality === 'number' ? r.avg_quality.toFixed(2) : '\u2014'}
              </span>
            ),
          },
          {
            id: 'len', header: 'Len', align: 'right',
            sortKey: (r) => typeof r.avg_length === 'number' ? r.avg_length : -1,
            accessor: (r) => (
              <span style={{ color: C.textMuted, fontFamily: T.typography.fontMono }}>
                {typeof r.avg_length === 'number' ? r.avg_length.toFixed(0) : '\u2014'}
              </span>
            ),
          },
        ];
        return (
          <DataTable<DomainRow> C={C} rows={rows} columns={cols}
            rowKey={(r) => r.domain}
            sort={{ col: 'facts', dir: 'desc' }}
            cellFontSize={T.typography.sizeXs} />
        );
      })()}
    </div>
  );
};
