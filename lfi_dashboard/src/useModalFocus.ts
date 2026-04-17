import { useEffect, useRef } from 'react';

const FOCUSABLE = 'button:not([disabled]), [href], input:not([type="hidden"]):not([disabled]), textarea:not([disabled]), select:not([disabled]), [tabindex]:not([tabindex="-1"])';

// Modal focus management:
//   1. On open: move focus into dialog (first focusable descendant).
//   2. While open: Tab / Shift-Tab cycles within dialog; focus can't escape.
//   3. On close: restore focus to whatever was focused before opening.
//
// Usage:
//   const dialogRef = useRef<HTMLDivElement>(null);
//   useModalFocus(open, dialogRef);
//   <div ref={dialogRef} role='dialog' aria-modal>...</div>
export const useModalFocus = (open: boolean, ref: React.RefObject<HTMLElement>) => {
  const previousFocused = useRef<Element | null>(null);
  useEffect(() => {
    if (!open) return;
    previousFocused.current = document.activeElement;
    // Next microtask so the modal's inputs have mounted.
    queueMicrotask(() => {
      const node = ref.current;
      if (!node) return;
      const first = node.querySelector<HTMLElement>(FOCUSABLE);
      (first ?? node).focus?.();
    });
    // Tab / Shift-Tab cycle trap.
    const onKey = (e: KeyboardEvent) => {
      if (e.key !== 'Tab') return;
      const node = ref.current;
      if (!node) return;
      const focusables = Array.from(node.querySelectorAll<HTMLElement>(FOCUSABLE))
        .filter(el => !el.hasAttribute('inert'));
      if (focusables.length === 0) return;
      const first = focusables[0];
      const last = focusables[focusables.length - 1];
      const active = document.activeElement as HTMLElement | null;
      // If Shift-Tab on first, wrap to last. Plain Tab on last, wrap to first.
      if (e.shiftKey && active === first) {
        e.preventDefault();
        last.focus();
      } else if (!e.shiftKey && active === last) {
        e.preventDefault();
        first.focus();
      }
      // If focus somehow escaped the dialog entirely, pull it back to first.
      if (active && !node.contains(active)) {
        e.preventDefault();
        first.focus();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('keydown', onKey);
      const prev = previousFocused.current as HTMLElement | null;
      prev?.focus?.();
    };
  }, [open, ref]);
};
