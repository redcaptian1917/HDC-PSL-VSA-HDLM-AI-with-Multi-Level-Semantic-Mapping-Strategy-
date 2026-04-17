import React from 'react';
import { T } from './tokens';

// Small presentational panel: shows the result of /api/facts — either the list,
// a "no facts returned" message, or the fetch error. Rendered inside the sidebar
// after the user clicks "View Facts".
//
// c2-236 / #20: migrated hardcoded spacing/radii/typography to tokens.ts so
// the visual rhythm snaps to the same grid as the rest of the app.

export interface FactsPanelProps {
  C: any;
  facts: Array<{ key: string; value: string }>;
  fetchedAt: number | null;
  error: string | null;
}

export const FactsPanel: React.FC<FactsPanelProps> = ({ C, facts, fetchedAt, error }) => {
  if (fetchedAt === null) return null;
  return (
    <div style={{ marginTop: T.spacing.lg }}>
      <div style={{
        fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
        color: C.textMuted, marginBottom: T.spacing.sm,
        textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose,
        display: 'flex', justifyContent: 'space-between',
      }}>
        <span>Knowledge Facts ({facts.length})</span>
        <span style={{ color: C.textDim, fontWeight: T.typography.weightMedium }}>{new Date(fetchedAt).toLocaleTimeString()}</span>
      </div>
      {error ? (
        <div style={{
          padding: `${T.spacing.sm} ${T.spacing.md}`,
          fontSize: T.typography.sizeXs, lineHeight: T.typography.lineTight,
          background: C.redBg, border: `1px solid ${C.redBorder}`,
          borderRadius: T.radii.md, color: C.red,
        }}>Fetch failed: {error}</div>
      ) : facts.length === 0 ? (
        <div style={{
          padding: `${T.spacing.sm} ${T.spacing.md}`,
          fontSize: T.typography.sizeXs, lineHeight: T.typography.lineTight,
          background: C.bgInput, border: `1px solid ${C.borderSubtle}`,
          borderRadius: T.radii.md, color: C.textMuted,
        }}>Server returned 0 facts. Knowledge base may still be hydrating — the live count is shown in Substrate Telemetry.</div>
      ) : (
        <div style={{ maxHeight: '200px', overflowY: 'auto' }}>
          {facts.map((f, i) => (
            <div key={i} style={{
              fontSize: T.typography.sizeXs,
              padding: `${T.spacing.xs} ${T.spacing.sm}`,
              borderBottom: `1px solid ${C.borderSubtle}`,
            }}>
              <span style={{ color: C.accent, fontWeight: T.typography.weightBold }}>{f.key}</span>
              <span style={{ color: C.textDim }}> = </span>
              <span style={{ color: C.textSecondary }}>{f.value}</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
