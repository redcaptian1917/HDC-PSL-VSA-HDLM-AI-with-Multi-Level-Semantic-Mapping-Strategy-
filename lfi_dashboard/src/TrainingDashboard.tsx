import React from 'react';
import { compactNum, formatRelative } from './util';
import { T } from './tokens';
// c2-340 / c0-auto-2 task 50: 20px + 24px heading sizes sourced from the
// cross-platform design-system (T.typography caps at 22).
import { typography as dsType } from './design-system';
// c2-347: shared uppercase meta-label.
import { Label } from './components/Label';

// c2-240 / #20: typography weights, tracking, and the most repeated radii
// migrated to tokens.ts. Card padding stays in px — the 14/10/8 cascade is
// used as an information-density tell, not a generic spacing scale, so
// snapping to the 4/8 grid would flatten the hierarchy. Keeping literals here
// is deliberate (AVP-2 §30 allows off-grid values when they encode meaning).

// Animated shimmer placeholder used while a fetch is in flight. Relies on the
// `@keyframes lfi-shimmer` defined in App.tsx's global <style> block.
export const Skeleton: React.FC<{
  w?: string | number;
  h?: string | number;
  radius?: number;
  style?: React.CSSProperties;
}> = ({ w = '100%', h = 14, radius = 6, style }) => (
  <div style={{
    width: typeof w === 'number' ? `${w}px` : w,
    height: typeof h === 'number' ? `${h}px` : h,
    borderRadius: radius,
    background: 'linear-gradient(90deg, rgba(255,255,255,0.04) 0%, rgba(255,255,255,0.12) 50%, rgba(255,255,255,0.04) 100%)',
    backgroundSize: '200% 100%',
    animation: 'lfi-shimmer 1.4s infinite linear',
    ...style,
  }} />
);

export interface TrainingDashboardProps {
  host: string;
  C: any;
  // Fallback for total facts when /api/admin/training/accuracy doesn't return it
  // (backend inconsistency — the field is documented but missing in practice).
  // Parent should pass /api/status.facts_count.
  totalFactsFallback?: number;
}

type DomainRow = {
  domain: string;
  fact_count: number;
  avg_quality: number | null;
  avg_length: number | null;
};

export function TrainingDashboardContent({ host, C, totalFactsFallback }: TrainingDashboardProps) {
  // Three independent fetches — one slow/failed endpoint should not black out the
  // whole panel. Each slice tracks its own state so we can render partial data.
  const [accuracy, setAccuracy] = React.useState<any | null>(null);
  const [domains, setDomains] = React.useState<DomainRow[] | null>(null);
  const [sessions, setSessions] = React.useState<any | null>(null);
  const [errors, setErrors] = React.useState<{ accuracy?: string; domains?: string; sessions?: string }>({});
  const [lastUpdated, setLastUpdated] = React.useState<number | null>(null);
  const [control, setControl] = React.useState<{ busy: 'start' | 'stop' | null; msg: string | null; ok: boolean }>({ busy: null, msg: null, ok: true });
  const [refreshing, setRefreshing] = React.useState(false);

  const refetch = React.useCallback(async () => {
    setRefreshing(true);
    const mk = (path: string, timeoutMs = 15000) => async () => {
      const ctrl = new AbortController();
      const to = setTimeout(() => ctrl.abort(), timeoutMs);
      try {
        const r = await fetch(`http://${host}:3000${path}`, { signal: ctrl.signal });
        if (!r.ok) throw new Error(`HTTP ${r.status}`);
        return await r.json();
      } finally { clearTimeout(to); }
    };
    const [a, d, s] = await Promise.allSettled([
      mk('/api/admin/training/accuracy')(),
      mk('/api/admin/training/domains')(),
      mk('/api/admin/training/sessions')(),
    ]);
    const nextErrors: { accuracy?: string; domains?: string; sessions?: string } = {};
    if (a.status === 'fulfilled') setAccuracy(a.value); else nextErrors.accuracy = String((a.reason as any)?.message || a.reason);
    if (d.status === 'fulfilled') setDomains(Array.isArray(d.value?.domains) ? d.value.domains : []); else nextErrors.domains = String((d.reason as any)?.message || d.reason);
    if (s.status === 'fulfilled') setSessions(s.value); else nextErrors.sessions = String((s.reason as any)?.message || s.reason);
    setErrors(nextErrors);
    setLastUpdated(Date.now());
    setRefreshing(false);
  }, [host]);

  const controlTrainer = React.useCallback(async (action: 'start' | 'stop') => {
    setControl({ busy: action, msg: null, ok: true });
    try {
      const ctrl = new AbortController();
      const to = setTimeout(() => ctrl.abort(), 10000);
      const r = await fetch(`http://${host}:3000/api/admin/training/${action}`, { method: 'POST', signal: ctrl.signal });
      clearTimeout(to);
      if (!r.ok) throw new Error(`HTTP ${r.status}`);
      const body = await r.json().catch(() => ({}));
      setControl({ busy: null, msg: body?.message || `Trainer ${action} requested`, ok: true });
      setTimeout(refetch, 500);
    } catch (e: any) {
      setControl({ busy: null, msg: `Failed to ${action}: ${e?.message || e}`, ok: false });
    }
  }, [host, refetch]);

  React.useEffect(() => {
    refetch();
    const id = setInterval(refetch, 30000);
    return () => clearInterval(id);
  }, [refetch]);

  // Parse the latest line of recent_training_log, which looks like:
  //   "[2026-04-16T20:16:21-04:00] cycle=696 domain=physics done"
  // Returns the parsed pieces plus an age in seconds so the UI can show
  // "2m ago · domain=physics cycle=696".
  const lastCycle = (() => {
    const log: string[] | undefined = accuracy?.recent_training_log;
    if (!Array.isArray(log) || log.length === 0) return null;
    // Log entries are one of:
    //   "[ts] cycle=N domain=X done"
    //   "[ts] cycle=N domain=X batch=50 priority=P sessions=S"
    // Capture the state tail so we can distinguish a completed cycle from an
    // in-progress batch. Scan from the end so the newest event wins.
    for (let i = log.length - 1; i >= 0; i--) {
      const line = log[i];
      const m = line.match(/^\[([^\]]+)\] cycle=(\d+) domain=(\w+) (.+)$/);
      if (m) {
        const when = Date.parse(m[1]);
        if (!Number.isNaN(when)) {
          const tail = m[4].trim();
          const state = tail.startsWith('batch=') ? 'in progress'
            : tail === 'done' ? 'done'
            : tail.length <= 12 ? tail
            : 'active';
          return {
            ts: when,
            ageSec: Math.max(0, Math.floor((Date.now() - when) / 1000)),
            cycle: m[2], domain: m[3], state,
          };
        }
      }
    }
    return null;
  })();

  const totalFacts: number | null = accuracy?.total_facts ?? totalFactsFallback ?? null;
  const adversarialFacts: number | null = accuracy?.adversarial_facts ?? null;
  const psl = accuracy?.psl_calibration || null;
  // Backend inconsistency: /api/admin/training/accuracy returns pass_rate as a
  // percent (e.g. 97.2); /api/quality/report returns it as a fraction (0..1).
  // Detect heuristically: values <= 1.5 are fractions, otherwise already percent.
  const passRatePct: number | null = typeof psl?.pass_rate === 'number'
    ? (psl.pass_rate <= 1.5 ? psl.pass_rate * 100 : psl.pass_rate)
    : null;
  const reasoningChains: number | null = accuracy?.reasoning_chains ?? null;

  const trainingState = sessions?.training_state || {};
  const domainStateEntries: [string, any][] = Object.entries(trainingState);
  const anyRecentlyTrained = domainStateEntries.some(([, st]: any) => st?.last_trained && (Date.now() / 1000 - Number(st.last_trained)) < 300);
  const trainerActive = !!(sessions?.trainer_running) || anyRecentlyTrained;

  const allFailed = errors.accuracy && errors.domains && errors.sessions;
  const firstLoad = lastUpdated == null;

  if (firstLoad) {
    return (
      <div>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px', marginBottom: '20px' }}>
          {[0, 1, 2].map(i => (
            <div key={i} style={{ padding: '14px', background: C.bgInput, border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.xl }}>
              <Skeleton w={80} h={24} style={{ margin: '0 auto 8px' }} />
              <Skeleton w={96} h={10} style={{ margin: '0 auto' }} />
            </div>
          ))}
        </div>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '10px', marginBottom: '20px' }}>
          {[0, 1, 2, 3].map(i => (
            <div key={i} style={{ padding: '14px', background: C.bgInput, border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.xl }}>
              <Skeleton w={64} h={20} style={{ margin: '0 auto 6px' }} />
              <Skeleton w={88} h={8} style={{ margin: '0 auto 4px' }} />
              <Skeleton w={72} h={6} style={{ margin: '0 auto' }} />
            </div>
          ))}
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: T.spacing.xs }}>
          {[0, 1, 2, 3, 4, 5].map(i => (
            <div key={i} style={{ padding: '8px 14px', background: C.bgInput, borderRadius: T.radii.lg, border: `1px solid ${C.borderSubtle}` }}>
              <Skeleton h={14} />
            </div>
          ))}
        </div>
      </div>
    );
  }
  if (allFailed) {
    return (
      <div style={{ padding: '24px', textAlign: 'center', color: C.textMuted }}>
        <div style={{ fontSize: T.typography.sizeBody, color: C.red, marginBottom: '6px' }}>Training endpoints unreachable</div>
        <div style={{ fontSize: T.typography.sizeSm, color: C.textDim }}>Backend may be restarting or the DB is in a write-lock window.</div>
        <button onClick={refetch} style={{
          marginTop: T.spacing.md, padding: '6px 14px', background: C.bgInput,
          border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.md, color: C.text,
          fontSize: T.typography.sizeSm, cursor: 'pointer',
        }}>Retry now</button>
      </div>
    );
  }


  return (
    <div>
      {/* Most recent training cycle — surfaces a single concrete "thing just happened"
          signal so the dashboard feels alive even when the other counters haven't moved. */}
      {lastCycle && (
        <div style={{
          padding: '10px 14px', marginBottom: '14px', borderRadius: T.radii.lg,
          background: lastCycle.ageSec < 300 ? C.greenBg : C.bgInput,
          border: `1px solid ${lastCycle.ageSec < 300 ? C.greenBorder : C.borderSubtle}`,
          display: 'flex', alignItems: 'center', gap: '10px', fontSize: T.typography.sizeSm,
        }}>
          <span
            className={lastCycle.ageSec < 300 ? 'lfi-trainer-pulse' : undefined}
            style={{
              display: 'inline-block', width: '8px', height: '8px', borderRadius: '50%',
              background: lastCycle.ageSec < 300 ? C.green : C.textDim,
            }}
          />
          <span style={{ color: C.textMuted, fontWeight: T.typography.weightSemibold }}>Most recent cycle</span>
          <span style={{ color: C.text }}>#{lastCycle.cycle}</span>
          <span style={{ color: C.accent, fontWeight: T.typography.weightSemibold }}>{lastCycle.domain}</span>
          <span style={{ color: C.textMuted }}>{lastCycle.state}</span>
          <span style={{ marginLeft: 'auto', color: C.textDim, fontSize: T.typography.sizeXs }}>{formatRelative(lastCycle.ts)}</span>
        </div>
      )}

      {/* Summary cards */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px', marginBottom: '20px' }}>
        <div style={{ padding: '14px', background: C.accentBg, border: `1px solid ${C.accentBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes['2xl'], fontWeight: T.typography.weightBlack, color: C.accent }}>{compactNum(totalFacts)}</div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: T.spacing.xs }}>Facts in DB</div>
        </div>
        <div style={{ padding: '14px', background: C.greenBg, border: `1px solid ${C.greenBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes['2xl'], fontWeight: T.typography.weightBlack, color: C.green }}>{domains?.length ?? domainStateEntries.length ?? '—'}</div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: T.spacing.xs }}>Domains</div>
        </div>
        <div style={{ padding: '14px', background: trainerActive ? C.greenBg : C.redBg, border: `1px solid ${trainerActive ? C.greenBorder : C.redBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes['2xl'], fontWeight: T.typography.weightBlack, color: trainerActive ? C.green : C.red, lineHeight: 1 }}>
            <span
              className={trainerActive ? 'lfi-trainer-pulse' : undefined}
              style={{
                display: 'inline-block', width: '14px', height: '14px', borderRadius: '50%',
                background: trainerActive ? C.green : 'transparent',
                border: trainerActive ? 'none' : `2px solid ${C.red}`,
                verticalAlign: 'middle',
              }}
            />
          </div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: '6px' }}>
            {trainerActive ? 'Trainer Active' : 'Trainer Idle'}
          </div>
          <div style={{ display: 'flex', gap: '6px', justifyContent: 'center', marginTop: T.spacing.sm }}>
            <button
              onClick={() => controlTrainer('start')}
              disabled={control.busy !== null || trainerActive}
              aria-label='Start training'
              style={{
                flex: 1, padding: '5px 8px', fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
                textTransform: 'uppercase', letterSpacing: '0.06em',
                color: trainerActive ? C.textDim : C.green,
                background: trainerActive ? 'transparent' : C.greenBg,
                border: `1px solid ${trainerActive ? C.borderSubtle : C.greenBorder}`,
                borderRadius: T.radii.md,
                cursor: (control.busy !== null || trainerActive) ? 'not-allowed' : 'pointer',
                opacity: control.busy === 'start' ? 0.55 : 1,
              }}
            >{control.busy === 'start' ? '…' : 'Start'}</button>
            <button
              onClick={() => controlTrainer('stop')}
              disabled={control.busy !== null || !trainerActive}
              aria-label='Stop training'
              style={{
                flex: 1, padding: '5px 8px', fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBold,
                textTransform: 'uppercase', letterSpacing: '0.06em',
                color: !trainerActive ? C.textDim : C.red,
                background: !trainerActive ? 'transparent' : C.redBg,
                border: `1px solid ${!trainerActive ? C.borderSubtle : C.redBorder}`,
                borderRadius: T.radii.md,
                cursor: (control.busy !== null || !trainerActive) ? 'not-allowed' : 'pointer',
                opacity: control.busy === 'stop' ? 0.55 : 1,
              }}
            >{control.busy === 'stop' ? '…' : 'Stop'}</button>
          </div>
          {control.msg && (
            <div style={{
              marginTop: '6px', fontSize: T.typography.sizeXs,
              color: control.ok ? C.textMuted : C.red,
              lineHeight: 1.3,
            }}>{control.msg}</div>
          )}
        </div>
      </div>

      {/* Quality & Security metrics row — all values live from /api/admin/training/accuracy */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '10px', marginBottom: '20px' }}>
        <div style={{ padding: '14px', background: C.greenBg, border: `1px solid ${C.greenBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes.xl, fontWeight: T.typography.weightBlack, color: C.green }}>
            {passRatePct != null ? `${passRatePct.toFixed(1)}%` : '—'}
          </div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: T.spacing.xs }}>PSL Pass Rate</div>
          <div style={{ fontSize: '9px', color: C.textDim, marginTop: '2px' }}>
            {psl?.target ? `Target: ${psl.target}` : 'Target: 95-98%'}
          </div>
          <div style={{ height: '4px', marginTop: T.spacing.sm, background: 'rgba(255,255,255,0.08)', borderRadius: T.radii.xs, overflow: 'hidden' }}>
            <div
              className="lfi-progress-fill"
              style={{
                height: '100%',
                width: passRatePct != null ? `${Math.max(0, Math.min(100, passRatePct))}%` : '0%',
                background: passRatePct == null ? C.textDim : (passRatePct >= 95 ? C.green : passRatePct >= 85 ? C.yellow : C.red),
              }}
            />
          </div>
        </div>
        <div style={{ padding: '14px', background: C.accentBg, border: `1px solid ${C.accentBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes.xl, fontWeight: T.typography.weightBlack, color: C.accent }}>{compactNum(adversarialFacts)}</div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: T.spacing.xs }}>Adversarial Facts</div>
          <div style={{ fontSize: '9px', color: C.textDim, marginTop: '2px' }}>ANLI + FEVER + TruthfulQA</div>
        </div>
        <div style={{ padding: '14px', background: C.accentBg, border: `1px solid ${C.accentBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes.xl, fontWeight: T.typography.weightBlack, color: C.accent }}>{compactNum(reasoningChains)}</div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: T.spacing.xs }}>Reasoning Chains</div>
          <div style={{ fontSize: '9px', color: C.textDim, marginTop: '2px' }}>Self-play + teacher</div>
        </div>
        <div style={{ padding: '14px', background: C.greenBg, border: `1px solid ${C.greenBorder}`, borderRadius: T.radii.xl, textAlign: 'center' }}>
          <div style={{ fontSize: dsType.sizes.xl, fontWeight: T.typography.weightBlack, color: C.green }}>
            {accuracy?.learning_signals != null ? compactNum(accuracy.learning_signals) : '—'}
          </div>
          <div style={{ fontSize: T.typography.sizeXs, color: C.textMuted, textTransform: 'uppercase', letterSpacing: T.typography.trackingLoose, marginTop: T.spacing.xs }}>Learning Signals</div>
          <div style={{ fontSize: '9px', color: C.textDim, marginTop: '2px' }}>Corrections + gaps + PSL</div>
        </div>
      </div>

      {/* Per-domain breakdown + heatmap (from /api/admin/training/domains + session timestamps) */}
      {domains && domains.length > 0 && (() => {
        const maxFacts = Math.max(...domains.map((d) => d.fact_count || 0), 1);
        const nowSec = Date.now() / 1000;
        const heatColor = (fc: number, q: number | null) => {
          const share = fc / maxFacts;
          const qMul = q == null ? 1 : Math.max(0.5, Math.min(1, q));
          const alpha = Math.max(0.12, Math.min(0.9, share * qMul));
          return `rgba(139, 123, 247, ${alpha.toFixed(2)})`;
        };
        return (
          <div style={{ marginBottom: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
              <Label color={C.textMuted}>
                Per-Domain Coverage ({domains.length})
              </Label>
              <div style={{ fontSize: '9px', color: C.textDim }}>bar width = fact share · tint = quality</div>
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: T.spacing.xs }}>
              {[...domains].sort((a, b) => b.fact_count - a.fact_count).map((d) => {
                const st = trainingState[d.domain] || {};
                const pct = Math.round((d.fact_count / maxFacts) * 100);
                const recent = st.last_trained && (nowSec - Number(st.last_trained)) < 300;
                return (
                  <div key={d.domain} style={{
                    position: 'relative',
                    padding: '8px 14px', background: C.bgInput, borderRadius: T.radii.lg,
                    border: `1px solid ${recent ? C.greenBorder : C.borderSubtle}`,
                    overflow: 'hidden',
                  }}>
                    <div
                      className="lfi-progress-fill"
                      style={{
                        position: 'absolute', inset: 0,
                        width: `${pct}%`, background: heatColor(d.fact_count, d.avg_quality),
                        pointerEvents: 'none',
                      }}
                    />
                    <div style={{ position: 'relative', display: 'flex', alignItems: 'center', gap: T.spacing.md, fontSize: T.typography.sizeSm }}>
                      <span style={{ fontWeight: T.typography.weightSemibold, color: C.text, minWidth: '110px' }}>{d.domain}</span>
                      <span style={{ color: C.textMuted }}>{compactNum(d.fact_count)} facts</span>
                      {d.avg_quality != null && (
                        <span style={{ color: C.textMuted }}>q={d.avg_quality.toFixed(2)}</span>
                      )}
                      {st.sessions != null && (
                        <span style={{ color: C.textMuted }}>{st.sessions} sessions</span>
                      )}
                      <div style={{ flex: 1 }} />
                      {recent && (
                        <span style={{ fontSize: '9px', color: C.green, fontWeight: T.typography.weightBold, letterSpacing: T.typography.trackingLoose }}>LIVE</span>
                      )}
                      <span style={{ fontSize: T.typography.sizeXs, color: C.textDim }}>
                        {st.last_trained ? new Date(st.last_trained * 1000).toLocaleTimeString() : 'never'}
                      </span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        );
      })()}

      {/* Recent training log (from /api/admin/training/accuracy) */}
      {Array.isArray(accuracy?.recent_training_log) && accuracy.recent_training_log.length > 0 && (
        <div>
          <Label color={C.textMuted} mb={'10px'}>
            Recent Training Log
          </Label>
          <pre style={{
            padding: '12px', background: C.bgInput, borderRadius: T.radii.lg,
            fontSize: T.typography.sizeXs, color: C.textSecondary,
            fontFamily: "'JetBrains Mono', monospace",
            whiteSpace: 'pre-wrap', maxHeight: '200px', overflowY: 'auto',
            margin: 0,
          }}>
            {accuracy.recent_training_log.slice(-40).join('\n')}
          </pre>
        </div>
      )}

      {/* Freshness + error footnote */}
      <div style={{ marginTop: T.spacing.lg, display: 'flex', justifyContent: 'space-between', alignItems: 'center', fontSize: T.typography.sizeXs, color: C.textDim }}>
        <span>
          {lastUpdated ? `Updated ${new Date(lastUpdated).toLocaleTimeString()}` : ''}
          {(errors.accuracy || errors.domains || errors.sessions) ? ` \u00B7 partial (${Object.keys(errors).join(', ')} failed)` : ''}
        </span>
        <button onClick={refetch} disabled={refreshing} style={{
          padding: '4px 10px', background: 'transparent',
          border: `1px solid ${C.borderSubtle}`, borderRadius: T.radii.md, color: C.textMuted,
          fontSize: T.typography.sizeXs, cursor: refreshing ? 'wait' : 'pointer',
          opacity: refreshing ? 0.5 : 1,
        }}>{refreshing ? 'Refreshing…' : 'Refresh'}</button>
      </div>
    </div>
  );
}
