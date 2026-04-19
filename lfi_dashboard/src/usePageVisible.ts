/**
 * usePageVisible — returns true when document.visibilityState === 'visible'.
 *
 * Paired with polling hooks, it freezes intervals when the user tabs
 * away — zero wasted requests, zero wasted battery on mobile, and
 * backend load drops while nobody's looking. Re-becoming-visible fires
 * an immediate state flip so consumers can optionally kick a fresh
 * fetch on return (they already do via the effect re-run that the
 * boolean deps flip triggers).
 */
import { useEffect, useState } from 'react';

export const usePageVisible = (): boolean => {
  const [visible, setVisible] = useState<boolean>(() => {
    if (typeof document === 'undefined') return true;
    return document.visibilityState !== 'hidden';
  });
  useEffect(() => {
    if (typeof document === 'undefined') return;
    const onChange = () => setVisible(document.visibilityState !== 'hidden');
    document.addEventListener('visibilitychange', onChange);
    return () => document.removeEventListener('visibilitychange', onChange);
  }, []);
  return visible;
};
