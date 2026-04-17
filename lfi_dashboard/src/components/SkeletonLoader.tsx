import React, { useEffect } from 'react';
import { T } from '../tokens';

// c0-auto-2 task 29 (CLAUDE2_500_TASKS.md): shared shimmer skeleton.
//
// Replaces 5 near-identical skeleton implementations in AdminModal,
// ClassroomView, KnowledgeBrowser, TrainingDashboard that all shared
// the same 1.3s horizontal shimmer but diverged on the base color
// (bgCard vs bgInput) and per-file @keyframes names. One canonical
// keyframes declaration, two base variants, optional stagger delay.
//
// AVP-PASS-27 (a11y): callers should still wrap groups of skeletons
// in a parent with aria-busy='true' aria-live='polite' so AT announces
// "loading" once per group instead of per-shimmer. This component is
// presentational only -- ARIA state lives on the group.

// Inject the keyframes once per page load. React may call this hook
// many times but the style tag's textContent is idempotent -- writing
// the same string is a no-op, and we guard with a module-level flag
// so we don't even touch the DOM after the first instance mounts.
let stylesInjected = false;
function useShimmerKeyframes(): void {
  useEffect(() => {
    if (stylesInjected) return;
    const el = document.createElement('style');
    el.setAttribute('data-scc-skeleton', 'true');
    el.textContent = '@keyframes scc-skel-shimmer { 0% { background-position: 200% 0 } 100% { background-position: -200% 0 } }';
    document.head.appendChild(el);
    stylesInjected = true;
  }, []);
}

export interface SkeletonLoaderProps {
  C: any;
  /** Which palette base to shimmer against. `'card'` (default) for blocks
   *  rendered on the page background; `'input'` for blocks nested inside
   *  already-recessed surfaces. */
  base?: 'card' | 'input';
  width?: string | number;
  height?: string | number;
  /** Corner radius; defaults to T.radii.lg (matches cards and modals). */
  borderRadius?: string;
  /** Stagger offset in seconds (useful when rendering a grid of skeletons
   *  so the shimmer runs like a wave instead of all columns in phase). */
  delay?: number;
  /** Escape hatch for one-off layout overrides (margin, alignSelf, etc). */
  style?: React.CSSProperties;
}

export const SkeletonLoader: React.FC<SkeletonLoaderProps> = ({
  C, base = 'card', width, height, borderRadius, delay, style,
}) => {
  useShimmerKeyframes();
  const baseColor = base === 'input' ? C.bgInput : C.bgCard;
  return (
    <div style={{
      width,
      height,
      borderRadius: borderRadius ?? T.radii.lg,
      background: `linear-gradient(90deg, ${baseColor} 0%, ${C.bgHover} 50%, ${baseColor} 100%)`,
      backgroundSize: '200% 100%',
      animation: 'scc-skel-shimmer 1.3s ease-in-out infinite',
      animationDelay: delay ? `${delay}s` : undefined,
      ...style,
    }} />
  );
};
