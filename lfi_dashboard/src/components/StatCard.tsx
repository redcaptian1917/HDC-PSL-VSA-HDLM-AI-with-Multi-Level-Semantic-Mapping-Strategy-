import React from 'react';
import { T } from '../tokens';
import { typography as dsType } from '../design-system';
import { Label } from './Label';

// c0-auto-2 task 25 (CLAUDE2_500_TASKS.md): canonical stat/summary card.
//
// FleetView, LibraryView, and AuditoriumView each had their own `Stat`
// helper -- identical shape, different file. Consolidate here so the
// per-page helpers become a one-line import. AdminModal's `DashCard`
// and ClassroomView's `Stat` use visibly different sizes (bgInput vs
// bgCard, 6px vs 4px margin, sizeXl vs size2xl) and stay local -- a
// caller that wants a different look should build its own rather than
// forcing variants into this component.
//
// Layout:
//   border, bg, and card padding -> tokens
//   label      -> <Label>      (sizeXs, bold, uppercase, trackingLoose)
//   value      -> 2xl, black weight, monospace, caller-provided color
//
// AVP-PASS-34 (design-system consistency): the three consumers had
// drifted by 1-2px in previous refactors; anchoring on one definition
// prevents future drift and makes tile layouts pixel-identical across
// pages.

export interface StatCardProps {
  C: any;
  label: string;
  /** Large value display. Accepts ReactNode so callers can pass spans /
   *  icons / formatted numbers without stringifying. */
  value: React.ReactNode;
  /** Value color. Typically C.text, or a semantic color for threshold
   *  states (C.green / C.yellow / C.red / C.accent). */
  color: string;
  /** Escape hatch for a per-call-site padding/margin tweak. Merged onto
   *  the outer wrapper, so callers can widen / add a border color. */
  style?: React.CSSProperties;
}

export const StatCard: React.FC<StatCardProps> = ({ C, label, value, color, style }) => (
  <div style={{
    padding: `${T.spacing.md} ${T.spacing.lg}`,
    borderRadius: T.radii.md,
    background: C.bgCard,
    border: `1px solid ${C.borderSubtle}`,
    ...style,
  }}>
    <Label color={C.textMuted}>{label}</Label>
    <div style={{
      fontSize: dsType.sizes['2xl'],
      fontWeight: T.typography.weightBlack,
      color,
      marginTop: T.spacing.xs,
      fontFamily: T.typography.fontMono,
    }}>{value}</div>
  </div>
);
