import React from 'react';
import { T } from './tokens';

// Class component because React 18 still requires class for error boundaries.
// Shows a helpful recovery surface instead of a blank page when any child throws
// during render. Resets on button click; offers reload as escape hatch.
// Theme fallback defaults are dark-palette safe since the boundary renders
// above theme context.
//
// c2-239 / #20: spacing/radii/typography now go through tokens.ts. Colour
// fallbacks remain hardcoded — the boundary deliberately runs above theme
// context so it can render even when the palette hasn't loaded.
export class AppErrorBoundary extends React.Component<
  { children: React.ReactNode; themeBg?: string; themeText?: string; themeAccent?: string },
  { error: Error | null; componentStack: string | null }
> {
  state = { error: null as Error | null, componentStack: null as string | null };
  static getDerivedStateFromError(error: Error) { return { error, componentStack: null }; }
  componentDidCatch(error: Error, info: React.ErrorInfo) {
    console.error('[AppErrorBoundary]', error, info?.componentStack);
    this.setState({ componentStack: info?.componentStack ?? null });
  }
  reset = () => { this.setState({ error: null, componentStack: null }); };
  render() {
    if (!this.state.error) return this.props.children;
    const bg = this.props.themeBg || '#0b0d14';
    const fg = this.props.themeText || '#e8e6f0';
    const accent = this.props.themeAccent || '#8b7bf7';
    const err = this.state.error;
    // Distinguish a lazy-chunk load failure from a generic render error — the
    // former usually means the user is offline or their cache is stale, not
    // that the UI is broken.
    const rawMsg = String(err?.message || err);
    const isChunkLoadError = /Failed to fetch dynamically imported module|Loading chunk|Loading CSS chunk|ChunkLoadError/i.test(rawMsg);
    const btnBase: React.CSSProperties = {
      padding: `${T.spacing.sm} 18px`,
      fontSize: T.typography.sizeMd,
      fontWeight: T.typography.weightBold,
      borderRadius: T.radii.lg,
      cursor: 'pointer', fontFamily: 'inherit',
    };
    return (
      <div role="alert" style={{
        minHeight: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center',
        background: bg, color: fg,
        padding: '40px', fontFamily: "'DM Sans', -apple-system, sans-serif",
      }}>
        <div style={{ maxWidth: '560px', width: '100%' }}>
          <div style={{
            fontSize: T.typography.sizeMd, color: accent,
            fontWeight: T.typography.weightBold,
            letterSpacing: '0.14em', textTransform: 'uppercase',
            marginBottom: T.spacing.sm,
          }}>
            {isChunkLoadError ? 'Module load failed' : 'UI Error'}
          </div>
          <h2 style={{
            fontSize: T.typography.size3xl, fontWeight: T.typography.weightBold,
            margin: `0 0 ${T.spacing.sm}`, letterSpacing: T.typography.trackingTight,
          }}>
            {isChunkLoadError ? 'A code chunk could not be fetched' : 'Something broke — but your work is safe'}
          </h2>
          <p style={{
            fontSize: T.typography.sizeBody, lineHeight: 1.6,
            opacity: 0.8, margin: `0 0 ${T.spacing.lg}`,
          }}>
            {isChunkLoadError
              ? 'This usually means the app is offline, the dev server was restarted, or the cache is stale. Reload the page to fetch the latest build.'
              : 'The dashboard hit a rendering error. Conversations and settings live in localStorage and are untouched. Try again to re-mount the UI; if that fails, reload.'}
          </p>
          <pre style={{
            background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)',
            borderRadius: T.radii.lg, padding: `${T.spacing.md} ${T.spacing.lg}`,
            fontSize: T.typography.sizeSm, lineHeight: 1.5,
            color: fg, overflow: 'auto', maxHeight: '200px', margin: `0 0 ${T.spacing.lg}`,
            fontFamily: "'JetBrains Mono', monospace",
          }}>{String(err?.message || err)}</pre>
          <div style={{ display: 'flex', gap: T.spacing.sm }}>
            <button onClick={this.reset} style={{
              ...btnBase, color: '#fff', background: accent, border: 'none',
            }}>Try again</button>
            <button onClick={() => window.location.reload()} style={{
              ...btnBase, color: fg, background: 'transparent',
              border: '1px solid rgba(255,255,255,0.12)',
            }}>Reload page</button>
          </div>
          {this.state.componentStack && (
            <details style={{ marginTop: T.spacing.lg, fontSize: T.typography.sizeXs, opacity: 0.6 }}>
              <summary style={{ cursor: 'pointer' }}>Component stack</summary>
              <pre style={{
                background: 'rgba(255,255,255,0.03)', padding: T.spacing.sm,
                borderRadius: T.radii.md, overflow: 'auto', maxHeight: '160px',
                fontFamily: "'JetBrains Mono', monospace", fontSize: '10px',
              }}>{this.state.componentStack}</pre>
            </details>
          )}
        </div>
      </div>
    );
  }
}
