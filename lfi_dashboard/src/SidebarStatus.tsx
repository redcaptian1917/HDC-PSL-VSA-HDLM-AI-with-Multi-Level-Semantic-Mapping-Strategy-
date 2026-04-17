import React from 'react';
import { compactNum, diskPressure } from './util';
import { T } from './tokens';

// The "Status" panel that sits below Substrate Telemetry in the sidebar.
// Rows: Connection, Tier, Throttled, Logic Density, PSL Pass, Adversarial,
// Sources, Disk. Derivations (PSL percent auto-detect, disk-pressure pct)
// happen inline here so the parent just forwards raw state.

export interface SidebarStatusProps {
  C: any;
  isConnected: boolean;
  currentTier: string;
  tierColor: (t: string) => string;
  thermalThrottled: boolean;
  logicDensity: number;
  quality: {
    psl_pass_rate: number | null;
    stale: boolean;
    adversarial: number;
    distinct_sources: number;
  } | null;
  kgSources: number;
  diskFree?: number;
  diskTotal?: number;
}

export const SidebarStatus: React.FC<SidebarStatusProps> = ({
  C, isConnected, currentTier, tierColor, thermalThrottled, logicDensity,
  quality, kgSources, diskFree, diskTotal,
}) => {
  // PSL auto-detect: /api/admin/training/accuracy returns percent (e.g. 97.2);
  // /api/quality/report returns fraction. Normalise to percent here.
  const pslRaw = quality?.psl_pass_rate;
  const pslPct = pslRaw == null ? null : (pslRaw <= 1.5 ? pslRaw * 100 : pslRaw);
  const dp = diskPressure(diskFree, diskTotal);

  const rows: Array<{ label: string; value: string; color: string }> = [
    { label: 'Connection', value: isConnected ? 'LIVE' : 'DOWN', color: isConnected ? C.green : C.red },
    { label: 'Tier', value: currentTier, color: tierColor(currentTier) },
    { label: 'Throttled', value: thermalThrottled ? 'YES' : 'NO', color: thermalThrottled ? C.red : C.green },
    { label: 'Logic Density', value: logicDensity.toFixed(3), color: C.purple },
    {
      label: 'PSL Pass',
      value: pslPct != null ? `${pslPct.toFixed(1)}%${quality?.stale ? ' (stale)' : ''}` : '—',
      color: pslPct == null
        ? C.textMuted
        : (pslPct >= 95 ? C.green : pslPct >= 85 ? C.yellow : C.red),
    },
    {
      label: 'Adversarial',
      value: quality?.adversarial != null ? compactNum(quality.adversarial) : '—',
      color: C.accent,
    },
    {
      label: 'Sources',
      value: kgSources ? String(kgSources) : (quality?.distinct_sources ? String(quality.distinct_sources) : '—'),
      color: C.purple,
    },
    dp
      ? { label: 'Disk', value: `${dp.usedPct.toFixed(0)}% · ${dp.freeGb.toFixed(1)}G free`, color: dp.usedPct >= 90 ? C.red : dp.usedPct >= 75 ? C.yellow : C.green }
      : { label: 'Disk', value: '—', color: C.textMuted },
  ];

  return (
    <div style={{ padding: T.spacing.xl, borderBottom: `1px solid ${C.borderSubtle}` }}>
      <div style={{
        fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBlack,
        color: C.textMuted, textTransform: 'uppercase',
        letterSpacing: T.typography.trackingCap,
        marginBottom: T.spacing.lg,
      }}>
        Status
      </div>
      <div style={{ display: 'flex', flexDirection: 'column', gap: T.spacing.sm }}>
        {rows.map(row => (
          <div key={row.label} style={{
            display: 'flex', justifyContent: 'space-between', fontSize: T.typography.sizeMd,
          }}>
            <span style={{ color: C.textMuted }}>{row.label}</span>
            <span style={{ color: row.color, fontWeight: T.typography.weightBold }}>{row.value}</span>
          </div>
        ))}
      </div>
    </div>
  );
};
