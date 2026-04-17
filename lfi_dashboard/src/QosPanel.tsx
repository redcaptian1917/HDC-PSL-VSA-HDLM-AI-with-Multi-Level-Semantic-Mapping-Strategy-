import React from 'react';

// Small presentational panel: shows the result of /api/qos — pass/fail summary
// + per-check rows, or an explicit error/empty state. Rendered inside the sidebar
// after the user clicks "QoS Report".

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
    <div style={{ marginTop: '14px' }}>
      <div style={{ fontSize: '10px', fontWeight: 700, color: C.textMuted, marginBottom: '8px', textTransform: 'uppercase', display: 'flex', justifyContent: 'space-between' }}>
        <span>QoS Report</span>
        <span style={{ color: C.textDim, fontWeight: 500 }}>{new Date(fetchedAt).toLocaleTimeString()}</span>
      </div>
      {error ? (
        <div style={{
          padding: '10px 12px', fontSize: '11px', lineHeight: 1.4,
          background: C.redBg, border: `1px solid ${C.redBorder}`,
          borderRadius: '6px', color: C.red,
        }}>Fetch failed: {error}</div>
      ) : !report ? (
        <div style={{
          padding: '10px 12px', fontSize: '11px',
          background: C.bgInput, border: `1px solid ${C.borderSubtle}`,
          borderRadius: '6px', color: C.textMuted,
        }}>QoS endpoint returned no data.</div>
      ) : (
        <>
          <div style={{
            padding: '10px', borderRadius: '8px', fontSize: '11px',
            background: report.passed ? C.greenBg : C.redBg,
            border: `1px solid ${report.passed ? C.greenBorder : C.redBorder}`,
            color: report.passed ? C.green : C.red,
            fontWeight: 700,
          }}>
            {report.passed
              ? `ALL ${report.checks?.length ?? 0} CHECKS PASS`
              : `${report.critical_failures ?? 0} CRITICAL \u00B7 ${report.warnings ?? 0} WARN`}
          </div>
          {report.checks && report.checks.length > 0 && (
            <div style={{ marginTop: '8px', display: 'flex', flexDirection: 'column', gap: '4px' }}>
              {report.checks.map((c, i) => (
                <div key={i} style={{
                  display: 'flex', justifyContent: 'space-between', gap: '8px',
                  fontSize: '11px', padding: '6px 8px', borderRadius: '6px',
                  background: c.passed ? C.greenBg : C.redBg,
                  border: `1px solid ${c.passed ? C.greenBorder : C.redBorder}`,
                }}>
                  <span style={{ color: C.textSecondary, flexShrink: 1 }}>{c.name}</span>
                  <span style={{ color: c.passed ? C.green : C.red, fontWeight: 700 }}>{c.value}</span>
                </div>
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
};
