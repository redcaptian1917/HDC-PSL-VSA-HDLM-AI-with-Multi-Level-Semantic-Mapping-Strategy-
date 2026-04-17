import React from 'react';

// Small presentational panel: shows the result of /api/facts — either the list,
// a "no facts returned" message, or the fetch error. Rendered inside the sidebar
// after the user clicks "View Facts".

export interface FactsPanelProps {
  C: any;
  facts: Array<{ key: string; value: string }>;
  fetchedAt: number | null;
  error: string | null;
}

export const FactsPanel: React.FC<FactsPanelProps> = ({ C, facts, fetchedAt, error }) => {
  if (fetchedAt === null) return null;
  return (
    <div style={{ marginTop: '14px' }}>
      <div style={{ fontSize: '10px', fontWeight: 700, color: C.textMuted, marginBottom: '8px', textTransform: 'uppercase', display: 'flex', justifyContent: 'space-between' }}>
        <span>Knowledge Facts ({facts.length})</span>
        <span style={{ color: C.textDim, fontWeight: 500 }}>{new Date(fetchedAt).toLocaleTimeString()}</span>
      </div>
      {error ? (
        <div style={{
          padding: '10px 12px', fontSize: '11px', lineHeight: 1.4,
          background: C.redBg, border: `1px solid ${C.redBorder}`,
          borderRadius: '6px', color: C.red,
        }}>Fetch failed: {error}</div>
      ) : facts.length === 0 ? (
        <div style={{
          padding: '10px 12px', fontSize: '11px', lineHeight: 1.4,
          background: C.bgInput, border: `1px solid ${C.borderSubtle}`,
          borderRadius: '6px', color: C.textMuted,
        }}>Server returned 0 facts. Knowledge base may still be hydrating — the live count is shown in Substrate Telemetry.</div>
      ) : (
        <div style={{ maxHeight: '200px', overflowY: 'auto' }}>
          {facts.map((f, i) => (
            <div key={i} style={{ fontSize: '11px', padding: '6px 8px', borderBottom: `1px solid ${C.borderSubtle}` }}>
              <span style={{ color: C.accent, fontWeight: 700 }}>{f.key}</span>
              <span style={{ color: C.textDim }}> = </span>
              <span style={{ color: C.textSecondary }}>{f.value}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
