import React, { useRef, useState, useEffect } from 'react';
import { T } from '../tokens';

// c0-auto-2 task 65 (CLAUDE2_500_TASKS.md): shared copy-to-clipboard button
// with a 2-second checkmark flash after the click. Previously every copy
// button in MessageBubble managed its own local state; consolidating here
// keeps the flash duration + icon set consistent.
//
// AVP-PASS-27 (a11y): aria-label swaps between "Copy" and "Copied" while
// the checkmark is showing, so screen readers announce the state change
// without a separate aria-live region.
//
// Usage:
//   <CopyButton C={C} onCopy={() => copyToClipboard(text)}
//               title='Copy (Shift-click: plain text)' />

export interface CopyButtonProps {
  C: any;
  /** Callback invoked when the user clicks. Receives the event so the
   *  caller can branch on shiftKey, ctrlKey, etc. */
  onCopy: (e: React.MouseEvent<HTMLButtonElement>) => void;
  /** Hint text for both the title attribute and aria-label (pre-click). */
  title: string;
  /** Square button size. Default 28px to match existing user-message
   *  action bar; pass 30px for the assistant-message hover bar. */
  size?: number;
  /** SVG icon stroke size. Default 13 for 28px, 14 for 30px. Explicitly
   *  passed so callers keep visual parity with the icon they're replacing. */
  iconSize?: number;
  /** Style override for one-off tweaks. Merged last. */
  style?: React.CSSProperties;
}

export const CopyButton: React.FC<CopyButtonProps> = ({
  C, onCopy, title, size = 28, iconSize, style,
}) => {
  const [justCopied, setJustCopied] = useState(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  useEffect(() => {
    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, []);
  const icon = iconSize ?? (size >= 30 ? 14 : 13);
  const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    onCopy(e);
    setJustCopied(true);
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    timeoutRef.current = setTimeout(() => setJustCopied(false), 2000);
  };
  return (
    <button
      onClick={handleClick}
      title={title}
      aria-label={justCopied ? 'Copied' : title}
      aria-live='polite'
      style={{
        width: `${size}px`, height: `${size}px`,
        display: 'flex', alignItems: 'center', justifyContent: 'center',
        background: 'transparent', border: 'none',
        color: justCopied ? C.green : C.textMuted,
        cursor: 'pointer', borderRadius: T.radii.md,
        transition: 'color 0.12s',
        ...style,
      }}
      onMouseEnter={(e) => { e.currentTarget.style.background = C.bgHover; }}
      onMouseLeave={(e) => { e.currentTarget.style.background = 'transparent'; }}>
      {justCopied ? (
        // Checkmark -- swap for 2s after click.
        <svg width={icon} height={icon} viewBox='0 0 24 24' fill='none'
          stroke='currentColor' strokeWidth={2.4} strokeLinecap='round' strokeLinejoin='round'>
          <polyline points='20 6 9 17 4 12' />
        </svg>
      ) : (
        // Standard two-rectangle copy glyph used by all prior copy buttons.
        <svg width={icon} height={icon} viewBox='0 0 24 24' fill='none'
          stroke='currentColor' strokeWidth={2} strokeLinecap='round' strokeLinejoin='round'>
          <rect x='9' y='9' width='13' height='13' rx='2' ry='2' />
          <path d='M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1' />
        </svg>
      )}
    </button>
  );
};
