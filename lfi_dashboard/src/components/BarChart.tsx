import React from 'react';
import { T } from '../tokens';

// c0-auto-2 task 27 (CLAUDE2_500_TASKS.md): horizontal progress bar.
//
// Replaces 6+ inline bar-track implementations scattered across
// AdminModal and ClassroomView that all followed the same 2-div
// pattern (track with overflow hidden + inner fill with width %
// and a 0.4s width transition). Consolidated so callers render a
// bar with one JSX node instead of seven lines of inline style.
//
// Not a full chart widget -- no axes, no tooltips, no stacked
// series. Pure single-series progress. If a caller needs axes or
// stacks they should build a dedicated component.
//
// AVP-PASS-27 (a11y): callers wanting screen-reader friendliness
// should wrap this in aria-label context ("Pass rate, 87%"), since
// by itself the bar is purely visual.

export interface BarChartProps {
  C: any;
  /** 0-100 percentage. Clamped to [0, 100] on render. */
  value: number;
  /** Fill color -- typically a semantic threshold color (green/yellow/red)
   *  or an accent. */
  color: string;
  /** Track height. Task default is 16px; callers can pass a different
   *  value when they want a thinner bar (e.g. '10px' for compact
   *  breakdown lists). */
  height?: string;
  /** Track background. Default C.bgCard -- the subdued recess that
   *  matches most card layouts. Use C.bgInput when nested inside an
   *  already-recessed surface. */
  trackBg?: string;
  /** Pass-through for the outer track style; useful for flex: 1 in
   *  horizontal layouts. */
  style?: React.CSSProperties;
}

export const BarChart: React.FC<BarChartProps> = ({
  C, value, color, height = '16px', trackBg, style,
}) => {
  const pct = Math.max(0, Math.min(100, value));
  return (
    <div style={{
      background: trackBg ?? C.bgCard,
      height,
      borderRadius: T.radii.xs,
      overflow: 'hidden',
      ...style,
    }}>
      <div style={{
        width: `${pct}%`,
        height: '100%',
        background: color,
        transition: 'width 0.4s',
      }} />
    </div>
  );
};
