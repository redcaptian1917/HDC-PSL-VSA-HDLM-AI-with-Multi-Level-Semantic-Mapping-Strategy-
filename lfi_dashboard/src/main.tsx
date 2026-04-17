import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'

// Register the service worker in production only. Dev builds skip it — Vite's
// HMR fights with any SW caching of /src/*.tsx, and we don't want half-stale
// modules while editing.
if (import.meta.env.PROD && 'serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js').catch(() => { /* non-fatal */ });
  });
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
