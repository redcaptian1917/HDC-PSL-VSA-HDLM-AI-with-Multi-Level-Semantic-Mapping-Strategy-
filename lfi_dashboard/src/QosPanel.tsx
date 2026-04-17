import React from 'react';
import { T } from './tokens';

// Small presentational panel: shows the result of /api/qos — pass/fail summary
// + per-check rows, or an explicit error/empty state. Rendered inside the sidebar
// after the user clicks "QoS Report".
//
// c2-236 / #20: migrated hardcoded spacing/radii/typography to tokens.ts.

export interface QosCheck {
  name: string;
  passed: boolean;
  value: string;
  threshold?: string;
  severity?: string;
}

export interface QosPanelProps {
  C: any;
  report: {
    passed?: boolean;
    critical_failures?: number;
    warnings?: number;
    checks?: QosCheck[];
  } | null;
  fetchedAt: number | null;
  error: string | null;
}

export const QosPanel: React.FC<QosPanelProps> = ({ C, report, fetchedAt, error }) => {
  if (fetchedAt === null) return null;
  return (
    <div style={{ marginTop: T.spacing.lg }}>
      <div style={{
        fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
        color: C.textMuted, marginBottom: T.spacing.sm,
        textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose,
        display: 'flex', justifyContent: 'space-between',
      }}>
        <span>QoS Report</span>
        <span style={{ color: C.textDim, fontWeight: T.typography.weightMedium }}>{new Date(fetchedAt).toLocaleTimeString()}</span>
      </div>
      {error ? (
        <div style={{
          padding: `${T.spacing.sm} ${T.spacing.md}`,
          fontSize: T.typography.sizeXs, lineHeight: T.typography.lineTight,
          background: C.redBg, border: `1px solid ${C.redBorder}`,
          borderRadius: T.radii.md, color: C.red,
        }}>Fetch failed: {error}</div>
      ) : !report ? (
        <div style={{
          padding: `${T.spacing.sm} ${T.spacing.md}`, fontSize: T.typography.sizeXs,
          background: C.bgInput, border: `1px solid ${C.borderSubtle}`,
          borderRadius: T.radii.md, color: C.textMuted,
        }}>QoS endpoint returned no data.</div>
      ) : (
        <>
          <div style={{
            padding: T.spacing.sm, borderRadius: T.radii.lg, fontSize: T.typography.sizeXs,
            background: report.passed ? C.greenBg : C.redBg,
            border: `1px solid ${report.passed ? C.greenBorder : C.redBorder}`,
            color: report.passed ? C.green : C.red,
            fontWeight: T.typography.weightBold,
          }}>
            {report.passed
              ? `ALL ${report.checks?.length ?? 0} CHECKS PASS`
              : `${report.critical_failures ?? 0} CRITICAL \u00B7 ${report.warnings ?? 0} WARN`}
          </div>
          {report.checks && report.checks.length > 0 && (
            <div style={{ marginTop: T.spacing.sm, display: 'flex', flexDirection: 'column', gap: T.spacing.xs }}>
              {report.checks.map((c, i) => (
                <div key={i} style={{
                  display: 'flex', justifyContent: 'space-between', gap: T.spacing.sm,
                  fontSize: T.typography.sizeXs, padding: `${T.spacing.xs} ${T.spacing.sm}`,
                  borderRadius: T.radii.md,
                  background: c.passed ? C.greenBg : C.redBg,
                  border: `1px solid ${c.passed ? C.greenBorder : C.redBorder}`,
                }}>
                  <span style={{ color: C.textSecondary, flexShrink: 1 }}>{c.name}</span>
                  <span style={{ color: c.passed ? C.green : C.red, fontWeight: T.typography.weightBold }}>{c.value}</span>
                </div>
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
};
