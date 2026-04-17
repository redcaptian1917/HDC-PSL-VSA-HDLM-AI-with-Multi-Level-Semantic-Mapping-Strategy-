// PlausiDen dashboard service worker — static-asset cache-first for offline.
// Scope is the registering origin (usually the full dashboard).
//
// Cache strategy:
//   - Same-origin GET for fonts/images/css/js: cache-first, populate-on-miss.
//   - Everything else (API calls, POSTs, cross-origin): network only.
// API calls MUST NEVER be cached — stale facts would be worse than offline.
// Navigation requests fall through to the network and let AppErrorBoundary
// surface a real offline card when fetch fails.

const CACHE_VERSION = 'plausiden-v1';
const SAME_ORIGIN_STATIC = /\.(?:js|mjs|css|woff2?|ttf|otf|png|jpg|jpeg|svg|webp|ico)(?:\?.*)?$/i;

self.addEventListener('install', (event) => {
  self.skipWaiting();
  event.waitUntil(caches.open(CACHE_VERSION));
});

self.addEventListener('activate', (event) => {
  event.waitUntil((async () => {
    const keys = await caches.keys();
    await Promise.all(keys.filter(k => k !== CACHE_VERSION).map(k => caches.delete(k)));
    await self.clients.claim();
  })());
});

self.addEventListener('fetch', (event) => {
  const req = event.request;
  if (req.method !== 'GET') return;
  const url = new URL(req.url);
  if (url.origin !== self.location.origin) return;            // skip cross-origin (API on :3000)
  if (url.pathname.startsWith('/api/')) return;               // explicit safety: never cache API
  if (url.pathname.startsWith('/ws/')) return;                // websockets shouldn't reach here but be safe
  if (!SAME_ORIGIN_STATIC.test(url.pathname)) return;         // only cache static assets

  event.respondWith((async () => {
    const cache = await caches.open(CACHE_VERSION);
    const hit = await cache.match(req);
    if (hit) return hit;
    try {
      const res = await fetch(req);
      // Only cache successful opaque-safe responses.
      if (res.ok && res.status === 200) {
        cache.put(req, res.clone()).catch(() => { /* quota — drop */ });
      }
      return res;
    } catch (e) {
      // Offline + no cache — return a minimal synthesized response so the browser
      // surfaces a reasonable error rather than a generic network failure.
      return new Response('offline', { status: 503, statusText: 'Offline' });
    }
  })());
});
