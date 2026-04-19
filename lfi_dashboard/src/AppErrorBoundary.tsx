import React from 'react';
import { T } from './tokens';
import { diag } from './diag';
import { clearChunkReloadFlag } from './lazyWithRetry';

// Class component because React 18 still requires class for error boundaries.
// Shows a helpful recovery surface instead of a blank page when any child throws
// during render. Resets on button click; offers reload as escape hatch.
//
// `inlineMode` shrinks the surface to a contained card (no 100dvh) so a
// per-modal boundary doesn't cover the whole app when an inner chunk fails.
// `label` lets a parent identify which surface failed in diag exports.
export class AppErrorBoundary extends React.Component<
  {
    children: React.ReactNode;
    themeBg?: string;
    themeText?: string;
    themeAccent?: string;
    inlineMode?: boolean;
    label?: string;
    onReset?: () => void;
  },
  { error: Error | null; componentStack: string | null; retryCount: number }
> {
  state = { error: null as Error | null, componentStack: null as string | null, retryCount: 0 };
  static getDerivedStateFromError(error: Error) { return { error, componentStack: null }; }
  componentDidCatch(error: Error, info: React.ErrorInfo) {
    const label = this.props.label || (this.props.inlineMode ? 'inline' : 'root');
    console.error(`[AppErrorBoundary:${label}]`, error, info?.componentStack);
    try {
      diag.error('error-boundary', `${label} caught: ${error.message || error}`, {
        label,
        inlineMode: !!this.props.inlineMode,
        message: error.message,
        stack: error.stack,
        componentStack: info?.componentStack,
        retryCount: this.state.retryCount,
      });
    } catch { /* diag must never break the boundary */ }
    this.setState({ componentStack: info?.componentStack ?? null });
  }
  reset = () => {
    try { clearChunkReloadFlag(); } catch { /* silent */ }
    this.setState((s) => ({ error: null, componentStack: null, retryCount: s.retryCount + 1 }));
    if (this.props.onReset) {
      try { this.props.onReset(); } catch { /* silent */ }
    }
  };
  render() {
    if (!this.state.error) return this.props.children;
    const bg = this.props.themeBg || '#0b0d14';
    const fg = this.props.themeText || '#e8e6f0';
    const accent = this.props.themeAccent || '#8b7bf7';
    const err = this.state.error;
    const inline = !!this.props.inlineMode;
    // Distinguish a lazy-chunk load failure from a generic render error — the
    // former usually means the user is offline or their cache is stale, not
    // that the UI is broken.
    const rawMsg = String(err?.message || err);
    const isChunkLoadError = /Failed to fetch dynamically imported module|Loading chunk|Loading CSS chunk|ChunkLoadError|Importing a module script failed/i.test(rawMsg);
    const btnBase: React.CSSProperties = {
      padding: inline ? `${T.spacing.xs} ${T.spacing.md}` : `${T.spacing.sm} 18px`,
      fontSize: inline ? T.typography.sizeSm : T.typography.sizeMd,
      fontWeight: T.typography.weightBold,
      borderRadius: T.radii.lg,
      cursor: 'pointer', fontFamily: 'inherit',
    };
    const wrapperStyle: React.CSSProperties = inline
      ? {
          // Inline mode: contained card. Caller controls outer container.
          padding: T.spacing.lg, background: bg, color: fg,
          borderRadius: T.radii.lg, border: '1px solid rgba(255,255,255,0.08)',
          fontFamily: "'DM Sans', -apple-system, sans-serif",
          maxWidth: '560px', margin: '0 auto',
        }
      : {
          minHeight: '100dvh', display: 'flex', alignItems: 'center', justifyContent: 'center',
          background: bg, color: fg,
          padding: '40px', fontFamily: "'DM Sans', -apple-system, sans-serif",
        };
    const innerWrapper: React.CSSProperties = inline
      ? { width: '100%' }
      : { maxWidth: '560px', width: '100%' };
    return (
      <div role="alert" style={wrapperStyle}>
        <div style={innerWrapper}>
          <div style={{
            fontSize: inline ? T.typography.sizeXs : T.typography.sizeMd, color: accent,
            fontWeight: T.typography.weightBold,
            letterSpacing: '0.14em', textTransform: 'uppercase',
            marginBottom: T.spacing.sm,
          }}>
            {isChunkLoadError ? 'Module load failed' : 'UI Error'}
            {this.props.label && <span style={{ opacity: 0.6, marginLeft: 8 }}>· {this.props.label}</span>}
          </div>
          <h2 style={{
            fontSize: inline ? T.typography.sizeLg : T.typography.size3xl,
            fontWeight: T.typography.weightBold,
            margin: `0 0 ${T.spacing.sm}`, letterSpacing: T.typography.trackingTight,
          }}>
            {isChunkLoadError ? 'A code chunk could not be fetched' : (inline ? 'This panel hit an error' : 'Something broke — but your work is safe')}
          </h2>
          <p style={{
            fontSize: inline ? T.typography.sizeSm : T.typography.sizeBody, lineHeight: 1.6,
            opacity: 0.8, margin: `0 0 ${T.spacing.md}`,
          }}>
            {isChunkLoadError
              ? 'Often a stale cache after a redeploy. Click "Try again" to retry, or "Reload page" to fetch the fresh build. Your conversations and settings are untouched.'
              : (inline
                ? 'The rest of the dashboard is still usable. Try again, or close and reopen this surface.'
                : 'The dashboard hit a rendering error. Conversations and settings live in localStorage and are untouched. Try again to re-mount the UI; if that fails, reload.')}
          </p>
          <pre style={{
            background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)',
            borderRadius: T.radii.md, padding: `${T.spacing.sm} ${T.spacing.md}`,
            fontSize: T.typography.sizeXs, lineHeight: 1.5,
            color: fg, overflow: 'auto', maxHeight: inline ? '120px' : '200px',
            margin: `0 0 ${T.spacing.md}`,
            fontFamily: "'JetBrains Mono', monospace",
            whiteSpace: 'pre-wrap', wordBreak: 'break-word',
          }}>{String(err?.message || err)}</pre>
          <div style={{ display: 'flex', gap: T.spacing.sm, flexWrap: 'wrap' }}>
            <button onClick={this.reset} style={{
              ...btnBase, color: '#fff', background: accent, border: 'none',
            }}>Try again</button>
            <button onClick={() => { try { clearChunkReloadFlag(); } catch { /* silent */ } window.location.reload(); }} style={{
              ...btnBase, color: fg, background: 'transparent',
              border: '1px solid rgba(255,255,255,0.12)',
            }}>Reload page</button>
            <button onClick={() => {
              try {
                const blob = diag.export();
                navigator.clipboard?.writeText(blob);
              } catch { /* silent */ }
            }} style={{
              ...btnBase, color: fg, background: 'transparent',
              border: '1px solid rgba(255,255,255,0.12)',
            }}>Copy diag log</button>
          </div>
          {this.state.componentStack && (
            <details style={{ marginTop: T.spacing.md, fontSize: T.typography.sizeXs, opacity: 0.6 }}>
              <summary style={{ cursor: 'pointer' }}>Component stack</summary>
              <pre style={{
                background: 'rgba(255,255,255,0.03)', padding: T.spacing.sm,
                borderRadius: T.radii.md, overflow: 'auto', maxHeight: '160px',
                fontFamily: "'JetBrains Mono', monospace", fontSize: '10px',
                whiteSpace: 'pre-wrap', wordBreak: 'break-word',
              }}>{this.state.componentStack}</pre>
            </details>
          )}
        </div>
      </div>
    );
  }
}
