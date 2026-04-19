import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import { AppErrorBoundary } from './AppErrorBoundary'

// Register the service worker in production only. Dev builds skip it — Vite's
// HMR fights with any SW caching of /src/*.tsx, and we don't want half-stale
// modules while editing.
//
// 2026-04-19: users were getting stuck on old bundles after redeploy because
// the SW's cache-first strategy kept serving stale chunks even after the
// activate handler evicted the old cache keys. Added an aggressive update
// path: register → on new SW found, force skipWaiting + reload so the next
// navigation serves fresh bytes.
if (import.meta.env.PROD && 'serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js').then((reg) => {
      // Force the SW to check for updates on every hard-reload.
      reg.update().catch(() => { /* silent */ });
      // When a new SW has installed, tell it to skipWaiting → then reload
      // the page so the new bundle takes over. One-time per update cycle
      // via sessionStorage guard so we don't reload-loop.
      reg.addEventListener('updatefound', () => {
        const nw = reg.installing;
        if (!nw) return;
        nw.addEventListener('statechange', () => {
          if (nw.state === 'installed' && navigator.serviceWorker.controller) {
            try {
              if (sessionStorage.getItem('lfi_sw_reloaded_once') === '1') return;
              sessionStorage.setItem('lfi_sw_reloaded_once', '1');
            } catch { /* private-mode */ }
            nw.postMessage?.({ type: 'SKIP_WAITING' });
            window.location.reload();
          }
        });
      });
    }).catch(() => { /* non-fatal */ });
    // Clear the one-shot reload flag shortly after load settles so a
    // later redeploy can reload once more.
    window.setTimeout(() => {
      try { sessionStorage.removeItem('lfi_sw_reloaded_once'); } catch { /* silent */ }
    }, 8000);
  });
}

// c2-317: wrap the root so a throw during App's initial render (before App's
// own scoped boundaries mount) doesn't fall off the world with a blank page.
// AppErrorBoundary uses hardcoded dark-palette fallbacks because it may be
// shown before any theme loads.
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AppErrorBoundary>
      <App />
    </AppErrorBoundary>
  </React.StrictMode>,
)
