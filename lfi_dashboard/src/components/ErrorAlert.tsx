import React from 'react';
import { T } from '../tokens';

// c0-auto-2 task 28 (CLAUDE2_500_TASKS.md): shared error banner.
//
// AdminModal had `AdminErr` and ClassroomView had an identically-shaped
// inline alert div -- same padding, same red palette, same layout, same
// Retry button. Promoted to a single component so:
//   1) the copy ("Could not load:") is canonical,
//   2) the Retry chip stays consistent across tab errors + page errors,
//   3) the alert role is always correct for screen readers.
//
// AVP-PASS-27 (WCAG 2.1 AA / error experience): role='alert' ensures
// screen readers announce the error; the retry chip has an accessible
// name via its visible label; the contrast palette goes through C so
// CONTRAST theme lands on the 7:1 pass rate.
//
// Not a generic toast -- toasts are single-shot transient notifications
// and belong in a different component (future task).

export interface ErrorAlertProps {
  C: any;
  /** Body text. Rendered after the "Could not load:" prefix by default. */
  message: string;
  /** Override prefix (e.g. 'Sources unavailable:'). Omit or pass '' to
   *  skip the bold prefix entirely. */
  prefix?: string;
  /** Optional retry callback. If provided, a chip button renders on
   *  the right-hand side and invokes this on click. */
  onRetry?: () => void;
  /** Set true while the retry is in flight to disable the chip + swap
   *  the label. */
  retrying?: boolean;
  /** Extra margin-bottom override (default T.spacing.md). */
  mb?: string;
}

export const ErrorAlert: React.FC<ErrorAlertProps> = ({
  C, message, prefix = 'Could not load:', onRetry, retrying, mb,
}) => (
  <div role='alert' style={{
    padding: `${T.spacing.md} 14px`,
    marginBottom: mb ?? T.spacing.md,
    background: C.redBg, border: `1px solid ${C.redBorder}`,
    color: C.red, borderRadius: T.radii.md, fontSize: T.typography.sizeMd,
    display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: T.spacing.md,
  }}>
    <span>{prefix && <strong>{prefix} </strong>}{message}</span>
    {onRetry && (
      <button onClick={onRetry} disabled={retrying}
        style={{
          background: 'transparent', border: `1px solid ${C.redBorder}`,
          color: C.red, borderRadius: T.radii.sm,
          padding: `${T.spacing.xs} ${T.spacing.md}`,
          cursor: retrying ? 'wait' : 'pointer',
          fontFamily: 'inherit', fontSize: T.typography.sizeXs,
          fontWeight: T.typography.weightBold, textTransform: 'uppercase',
          letterSpacing: '0.06em', flexShrink: 0,
        }}>{retrying ? 'Retrying…' : 'Retry'}</button>
    )}
  </div>
);
