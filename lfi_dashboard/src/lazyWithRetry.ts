/**
 * lazyWithRetry — drop-in replacement for React.lazy that survives stale
 * chunk hashes after a redeploy.
 *
 * Vite emits content-hashed chunk filenames (e.g. AdminModal-9e51fc31.js).
 * After a rebuild, the index.html that's currently in the user's tab still
 * references the OLD hash, but the chunk on disk has a NEW hash. First
 * lazy-load throws "Failed to fetch dynamically imported module" and
 * white-screens the app.
 *
 * Recovery strategy:
 *   1. Retry up to 2 times with 350ms / 700ms backoff (transient network).
 *   2. If all retries fail AND we haven't already attempted a one-time
 *      cache-bust reload this session, set a sessionStorage flag and call
 *      window.location.reload() — that fetches the fresh index.html with
 *      the correct chunk hashes.
 *   3. If the flag is already set (we already reloaded this session),
 *      let the error propagate to the AppErrorBoundary so the user sees
 *      "Module load failed" with manual reload buttons.
 *
 * Every step is logged to diag for the Diag tab + telemetry.
 */
import React from 'react';
import { diag } from './diag';

const RELOAD_FLAG = 'lfi_chunk_reload_attempted';
const MAX_RETRIES = 2;
const BACKOFF_MS = [350, 700];

function isChunkLoadError(err: unknown): boolean {
  const msg = err instanceof Error ? err.message : String(err);
  return /Failed to fetch dynamically imported module|Loading chunk|Loading CSS chunk|ChunkLoadError|Importing a module script failed/i.test(msg);
}

function alreadyReloadedThisSession(): boolean {
  try { return sessionStorage.getItem(RELOAD_FLAG) === '1'; } catch { return false; }
}

function markReloaded() {
  try { sessionStorage.setItem(RELOAD_FLAG, '1'); } catch { /* private mode */ }
}

function clearReloadFlag() {
  try { sessionStorage.removeItem(RELOAD_FLAG); } catch { /* silent */ }
}

export function lazyWithRetry<T extends React.ComponentType<any>>(
  factory: () => Promise<{ default: T }>,
  chunkName: string,
): React.LazyExoticComponent<T> {
  return React.lazy(async () => {
    let lastErr: unknown = null;
    for (let attempt = 0; attempt <= MAX_RETRIES; attempt++) {
      try {
        const m = await factory();
        if (attempt > 0) {
          diag.info('lazy-retry', `${chunkName} loaded after retry`, { attempt });
        }
        return m;
      } catch (err) {
        lastErr = err;
        if (!isChunkLoadError(err)) {
          diag.error('lazy-load', `${chunkName} non-chunk error`, err);
          throw err;
        }
        if (attempt < MAX_RETRIES) {
          diag.warn('lazy-retry', `${chunkName} chunk load failed, retrying`, {
            attempt: attempt + 1,
            of: MAX_RETRIES,
            err: String((err as Error).message || err),
          });
          await new Promise(r => setTimeout(r, BACKOFF_MS[attempt]));
        }
      }
    }
    if (!alreadyReloadedThisSession() && typeof window !== 'undefined') {
      diag.error('lazy-load', `${chunkName} chunk unrecoverable — auto-reloading once`, lastErr);
      markReloaded();
      window.location.reload();
      return new Promise(() => { /* intentional hang while reload happens */ }) as Promise<{ default: T }>;
    }
    diag.error('lazy-load', `${chunkName} chunk failed after retries + reload — surfacing to error boundary`, lastErr);
    throw lastErr;
  });
}

export function clearChunkReloadFlag() {
  clearReloadFlag();
}

if (typeof window !== 'undefined') {
  window.addEventListener('load', () => {
    setTimeout(() => clearReloadFlag(), 5000);
  });
}
