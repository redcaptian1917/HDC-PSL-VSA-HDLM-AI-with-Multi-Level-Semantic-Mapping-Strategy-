import React, { useEffect, useState } from 'react';
import { T } from './tokens';
// c2-347: shared stat/summary card (replaces the local Stat helper).
import { StatCard } from './components/StatCard';
import { formatRelative } from './util';

// c0-037 #12 / c2-331: Auditorium — AVP-2 audit state surface.
// Stub in the sense that there is no dedicated backend endpoint yet.
// The page:
//  1) renders the 9-tier / 36-pass AVP-2 structure as static reference
//     data (the protocol itself is stable per AVP2_SUPERSOCIETY_PROTOCOL.md)
//  2) tries /api/avp/status (then /api/admin/avp/status) for any live
//     pass/findings data; if unavailable, falls through to the reference
//     view with an inline "live stats unavailable" notice.
// When the backend ships real AVP state, the same component renders it.

interface AvpStatus {
  passes_completed?: number;
  total_passes?: number;
  findings_total?: number;
  findings_fixed?: number;
  security_score?: number;       // 0..1 or 0..100 — normalised
  code_quality_score?: number;   // ditto
  last_run?: string | number;
  tier_progress?: Array<{ tier: number; name: string; status: 'pending' | 'in_progress' | 'passed' | 'failed' }>;
  recent_findings?: Array<{ id: string; title: string; severity: 'low' | 'medium' | 'high' | 'critical'; fixed?: boolean; ts?: number | string }>;
}

export interface AuditoriumViewProps {
  C: any;
  host: string;
  isDesktop: boolean;
}

// Reference data — Tier structure from AVP2_SUPERSOCIETY_PROTOCOL.md.
const TIERS: Array<{ tier: number; name: string; passes: number; range: string }> = [
  { tier: 1, name: 'Existence proof',    passes: 6, range: '1–6'   },
  { tier: 2, name: 'Failure resilience', passes: 6, range: '7–12'  },
  { tier: 3, name: 'Adversarial security', passes: 12, range: '13–24' },
  { tier: 4, name: 'UX/UI adversarial',  passes: 6, range: '25–30' },
  { tier: 5, name: 'Integration & ecosystem', passes: 3, range: '31–33' },
  { tier: 6, name: 'Meta-validation',    passes: 3, range: '34–36' },
];
const TOTAL_PASSES = TIERS.reduce((s, t) => s + t.passes, 0); // 36

const pctNorm = (raw: number | undefined): number | null => {
  if (typeof raw !== 'number' || !isFinite(raw)) return null;
  return raw <= 1.5 ? raw * 100 : raw;
};

export const AuditoriumView: React.FC<AuditoriumViewProps> = ({ C, host, isDesktop }) => {
  const [status, setStatus] = useState<AvpStatus | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [lastUpdated, setLastUpdated] = useState<number | null>(null);

  const load = async () => {
    setLoading(true);
    setErr(null);
    const tryPath = async (path: string) => {
      const ctrl = new AbortController();
      const to = setTimeout(() => ctrl.abort(), 4000);
      try {
        const r = await fetch(`http://${host}:3000${path}`, { signal: ctrl.signal });
        if (!r.ok) throw new Error(`HTTP ${r.status}`);
        return (await r.json()) as AvpStatus;
      } finally { clearTimeout(to); }
    };
    try {
      let data: AvpStatus;
      try { data = await tryPath('/api/avp/status'); }
      catch { data = await tryPath('/api/admin/avp/status'); }
      setStatus(data);
      setLastUpdated(Date.now());
    } catch (e: any) {
      const m = String(e?.message || e || 'fetch failed');
      setErr(m.includes('abort') ? 'AVP status endpoint timed out.' : m);
      // Not a blocker — we still render the reference view below.
    } finally {
      setLoading(false);
    }
  };
  useEffect(() => { load(); /* eslint-disable-next-line */ }, []);

  const passesCompleted = status?.passes_completed ?? 0;
  const totalPasses = status?.total_passes ?? TOTAL_PASSES;
  const findingsTotal = status?.findings_total ?? null;
  const findingsFixed = status?.findings_fixed ?? null;
  const securityPct = pctNorm(status?.security_score);
  const qualityPct = pctNorm(status?.code_quality_score);

  const scoreColor = (v: number | null) => {
    if (v == null) return C.textMuted;
    return v >= 80 ? C.green : v >= 60 ? C.yellow : C.red;
  };

  return (
    <div style={{
      flex: 1, display: 'flex', flexDirection: 'column', minWidth: 0,
      background: C.bg, color: C.text, overflow: 'hidden',
      animation: 'lfi-fadein 0.18s ease-out',
    }}>
      <div style={{
        display: 'flex', alignItems: 'center', gap: T.spacing.md,
        padding: `${T.spacing.lg} ${T.spacing.xl}`,
        borderBottom: `1px solid ${C.borderSubtle}`, background: C.bgCard,
      }}>
        <h1 style={{
          margin: 0, fontSize: T.typography.sizeXl,
          fontWeight: T.typography.weightBlack, color: C.text,
          letterSpacing: T.typography.trackingCap, textTransform: 'uppercase',
        }}>Auditorium</h1>
        <span style={{
          fontSize: T.typography.sizeXs, color: C.textMuted,
          padding: '2px 8px', border: `1px solid ${C.borderSubtle}`,
          borderRadius: T.radii.sm, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose,
        }}>AVP-2</span>
        <div style={{ flex: 1 }} />
        {lastUpdated != null && !err && (
          <span style={{ fontSize: T.typography.sizeXs, color: C.textDim, fontFamily: 'ui-monospace, monospace' }}>
            Updated {formatRelative(lastUpdated)}
          </span>
        )}
        <button onClick={load} disabled={loading} aria-label='Refresh AVP status'
          style={{
            background: 'transparent', border: `1px solid ${C.borderSubtle}`,
            color: C.textMuted, borderRadius: T.radii.sm,
            cursor: loading ? 'wait' : 'pointer',
            padding: '4px 8px', display: 'flex', alignItems: 'center',
            fontFamily: 'inherit',
          }}>
          <svg width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='currentColor'
            strokeWidth='2.2' strokeLinecap='round' strokeLinejoin='round'
            style={loading ? { animation: 'scc-aud-spin 0.8s linear infinite' } : undefined}>
            <polyline points='23 4 23 10 17 10' />
            <polyline points='1 20 1 14 7 14' />
            <path d='M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15' />
          </svg>
        </button>
        <style>{`@keyframes scc-aud-spin { to { transform: rotate(360deg); } }`}</style>
      </div>

      <div style={{ flex: 1, overflowY: 'auto', padding: T.spacing.xl, maxWidth: '1200px', width: '100%', margin: '0 auto' }}>
        {err && (
          <div role='status' style={{
            padding: `${T.spacing.sm} ${T.spacing.md}`, marginBottom: T.spacing.lg,
            background: C.yellowBg, border: `1px solid ${C.yellowBorder || C.yellow}`,
            color: C.yellow, borderRadius: T.radii.md, fontSize: T.typography.sizeSm,
          }}>
            Live AVP status unavailable ({err}). Showing protocol reference below.
          </div>
        )}

        {/* Headline stats — live if available, otherwise protocol totals. */}
        <div style={{
          display: 'grid', gridTemplateColumns: isDesktop ? 'repeat(auto-fit, minmax(200px, 1fr))' : 'repeat(2, 1fr)',
          gap: T.spacing.md, marginBottom: T.spacing.xl,
        }}>
          <StatCard C={C} label='Passes'
            value={`${passesCompleted} / ${totalPasses}`}
            color={passesCompleted >= totalPasses ? C.green : passesCompleted > 0 ? C.yellow : C.textMuted} />
          <StatCard C={C} label='Findings fixed'
            value={findingsTotal != null ? `${findingsFixed ?? 0} / ${findingsTotal}` : '—'}
            color={findingsTotal != null && findingsFixed != null
              ? (findingsFixed >= findingsTotal ? C.green
                 : findingsFixed / Math.max(findingsTotal, 1) >= 0.5 ? C.yellow : C.red)
              : C.textMuted} />
          <StatCard C={C} label='Security score'
            value={securityPct != null ? `${securityPct.toFixed(1)}%` : '—'}
            color={scoreColor(securityPct)} />
          <StatCard C={C} label='Code quality'
            value={qualityPct != null ? `${qualityPct.toFixed(1)}%` : '—'}
            color={scoreColor(qualityPct)} />
        </div>

        {/* Tier structure — always rendered from the protocol. */}
        <div style={{ marginBottom: T.spacing.xl }}>
          <div style={{
            fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
            color: C.textMuted, textTransform: 'uppercase',
            letterSpacing: T.typography.trackingLoose, marginBottom: T.spacing.sm,
          }}>
            Tier structure (AVP-2 §Loop)
          </div>
          <div style={{ border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.md, overflow: 'hidden' }}>
            <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: T.typography.sizeSm }}>
              <thead>
                <tr>
                  <Th C={C} align='left'>Tier</Th>
                  <Th C={C} align='left'>Name</Th>
                  <Th C={C} align='right'>Passes</Th>
                  <Th C={C} align='right'>Range</Th>
                  <Th C={C} align='center'>Status</Th>
                </tr>
              </thead>
              <tbody>
                {TIERS.map((t, i) => {
                  const live = status?.tier_progress?.find(x => x.tier === t.tier);
                  const statusLabel = live?.status ?? (passesCompleted >= TIERS.slice(0, i + 1).reduce((s, tt) => s + tt.passes, 0) ? 'passed'
                    : passesCompleted >= TIERS.slice(0, i).reduce((s, tt) => s + tt.passes, 0) ? 'in_progress' : 'pending');
                  const statusColor = statusLabel === 'passed' ? C.green
                    : statusLabel === 'in_progress' ? C.yellow
                    : statusLabel === 'failed' ? C.red : C.textDim;
                  return (
                    <tr key={t.tier} style={{ background: i % 2 === 0 ? 'transparent' : C.bgHover }}>
                      <td style={{ padding: '8px 12px', fontFamily: 'ui-monospace, monospace', color: C.accent, width: '56px' }}>T{t.tier}</td>
                      <td style={{ padding: '8px 12px', color: C.text }}>{t.name}</td>
                      <td style={{ padding: '8px 12px', textAlign: 'right', color: C.textMuted, fontFamily: 'ui-monospace, monospace' }}>{t.passes}</td>
                      <td style={{ padding: '8px 12px', textAlign: 'right', color: C.textMuted, fontFamily: 'ui-monospace, monospace' }}>{t.range}</td>
                      <td style={{ padding: '8px 12px', textAlign: 'center', color: statusColor, fontSize: '11px', fontWeight: T.typography.weightBold, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose }}>
                        {statusLabel.replace('_', ' ')}
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>

        {/* Recent findings — live-only; hidden when backend has none. */}
        {status?.recent_findings && status.recent_findings.length > 0 && (
          <div>
            <div style={{
              fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
              color: C.textMuted, textTransform: 'uppercase',
              letterSpacing: T.typography.trackingLoose, marginBottom: T.spacing.sm,
            }}>Recent findings ({status.recent_findings.length})</div>
            <div style={{ border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.md, overflow: 'hidden', maxHeight: '320px', overflowY: 'auto' }}>
              <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: T.typography.sizeSm }}>
                <thead>
                  <tr>
                    <Th C={C} align='left'>ID</Th>
                    <Th C={C} align='left'>Title</Th>
                    <Th C={C} align='center'>Severity</Th>
                    <Th C={C} align='center'>Status</Th>
                  </tr>
                </thead>
                <tbody>
                  {status.recent_findings.slice(0, 100).map((f, i) => {
                    const sevColor = f.severity === 'critical' ? C.red
                      : f.severity === 'high' ? C.red
                      : f.severity === 'medium' ? C.yellow : C.textMuted;
                    return (
                      <tr key={f.id || i} style={{ background: i % 2 === 0 ? 'transparent' : C.bgHover }}>
                        <td style={{ padding: '6px 12px', color: C.accent, fontFamily: 'ui-monospace, monospace' }}>{f.id || `#${i + 1}`}</td>
                        <td style={{ padding: '6px 12px', color: C.text }}>{f.title}</td>
                        <td style={{ padding: '6px 12px', textAlign: 'center', color: sevColor, fontSize: '10px', fontWeight: T.typography.weightBold, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose }}>{f.severity}</td>
                        <td style={{ padding: '6px 12px', textAlign: 'center', color: f.fixed ? C.green : C.textMuted, fontSize: '14px' }}>
                          {f.fixed ? '\u2714' : '\u2022'}
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {/* Verdict — always. AVP-2 §Ship verdict is always "STILL BROKEN". */}
        <div style={{
          marginTop: T.spacing.xl,
          padding: `${T.spacing.md} ${T.spacing.lg}`,
          background: C.bgCard, border: `1px solid ${C.borderSubtle}`,
          borderRadius: T.radii.md, fontSize: T.typography.sizeSm,
          color: C.textSecondary, lineHeight: T.typography.lineLoose,
        }}>
          <strong style={{ color: C.text }}>Ship verdict:</strong> per AVP-2 §Ship verdict, the
          answer is always <em>STILL BROKEN</em>. Shipping is explicit risk
          acceptance, not a declaration of correctness. The loop resumes on
          the next commit.
        </div>
      </div>
    </div>
  );
};

// ---- Private helpers ----

// c2-347: the local Stat helper moved to components/StatCard.tsx.

const Th: React.FC<{ C: any; children: React.ReactNode; align?: 'left' | 'right' | 'center' }> = ({ C, children, align = 'left' }) => (
  <th style={{
    textAlign: align, padding: '8px 12px',
    fontWeight: T.typography.weightBold, color: C.textSecondary,
    background: C.bgInput, borderBottom: `1px solid ${C.borderSubtle}`,
    position: 'sticky', top: 0,
  }}>{children}</th>
);
