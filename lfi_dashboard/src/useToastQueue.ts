import { useCallback, useEffect, useRef, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 1): toast queue lifted from
// App.tsx. Owns the array, the per-id auto-dismiss schedule, and the
// showToast/dismiss callbacks. Returns the slice the renderer needs +
// stable callbacks. The auto-dismiss is a two-phase animation: display,
// flip to `exiting` for a 180ms fade-out, then unmount. Each toast id is
// scheduled exactly once (tracked in scheduledIds ref) so re-renders of
// the array don't double-schedule.
//
// Undo toasts hold 5s (long enough to read + react); plain toasts 1.5s.
// Click-to-dismiss is on the renderer side (just call dismiss(id)).

export type ToastEntry = { id: number; msg: string; exiting?: boolean; onUndo?: () => void };

export interface ToastQueueAPI {
  toasts: ToastEntry[];
  showToast: (msg: string, onUndo?: () => void) => void;
  dismiss: (id: number) => void;
  setToasts: React.Dispatch<React.SetStateAction<ToastEntry[]>>;
}

export function useToastQueue(): ToastQueueAPI {
  const [toasts, setToasts] = useState<ToastEntry[]>([]);
  const scheduledIds = useRef<Set<number>>(new Set());

  const showToast = useCallback((msg: string, onUndo?: () => void) => {
    // Date.now() + random to avoid id collisions when two toasts fire in
    // the same ms (e.g. async then sync path both landing).
    const id = Date.now() + Math.random();
    setToasts(prev => [...prev, { id, msg, onUndo }]);
  }, []);

  const dismiss = useCallback((id: number) => {
    setToasts(prev => prev.filter(t => t.id !== id));
    scheduledIds.current.delete(id);
  }, []);

  useEffect(() => {
    for (const t of toasts) {
      if (t.exiting || scheduledIds.current.has(t.id)) continue;
      scheduledIds.current.add(t.id);
      const hold = t.onUndo ? 5000 : 1500;
      setTimeout(() => {
        setToasts(prev => prev.map(tt => tt.id === t.id ? { ...tt, exiting: true } : tt));
      }, hold);
      setTimeout(() => {
        setToasts(prev => prev.filter(tt => tt.id !== t.id));
        scheduledIds.current.delete(t.id);
      }, hold + 180);
    }
  }, [toasts]);

  return { toasts, showToast, dismiss, setToasts };
}
