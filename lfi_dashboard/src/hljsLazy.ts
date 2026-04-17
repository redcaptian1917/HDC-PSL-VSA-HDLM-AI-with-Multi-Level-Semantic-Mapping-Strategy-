// Lazy language loader for highlight.js.
//
// c2-228 / #79: the initial bundle previously eagerly imported + registered
// 10 language grammars (~60-90 KB uncompressed). Most chat messages contain
// no code, and when they do only one or two languages are used per session.
//
// This module replaces the up-front registration with a per-language
// dynamic import that Vite code-splits into its own chunk. Call
// ensureLanguage(lang) before hljs.highlight — it resolves when the language
// has been registered (or immediately if already loaded).

import hljs from 'highlight.js/lib/core';

// Canonical-name → { aliases, dynamic loader }. Keep in sync with the
// fenced-code language tags the users actually write.
const LANG_LOADERS: Record<string, { names: string[]; load: () => Promise<{ default: any }> }> = {
  rust:       { names: ['rust', 'rs'],          load: () => import('highlight.js/lib/languages/rust') },
  javascript: { names: ['javascript', 'js'],    load: () => import('highlight.js/lib/languages/javascript') },
  typescript: { names: ['typescript', 'ts'],    load: () => import('highlight.js/lib/languages/typescript') },
  python:     { names: ['python', 'py'],        load: () => import('highlight.js/lib/languages/python') },
  bash:       { names: ['bash', 'sh', 'shell'], load: () => import('highlight.js/lib/languages/bash') },
  json:       { names: ['json'],                load: () => import('highlight.js/lib/languages/json') },
  sql:        { names: ['sql'],                 load: () => import('highlight.js/lib/languages/sql') },
  css:        { names: ['css'],                 load: () => import('highlight.js/lib/languages/css') },
  xml:        { names: ['xml', 'html'],         load: () => import('highlight.js/lib/languages/xml') },
  go:         { names: ['go', 'golang'],        load: () => import('highlight.js/lib/languages/go') },
};

// alias → canonical key. Built once so the tag-to-loader lookup is O(1).
const ALIAS_TO_CANON: Record<string, string> = (() => {
  const out: Record<string, string> = {};
  for (const [canon, { names }] of Object.entries(LANG_LOADERS)) {
    for (const n of names) out[n] = canon;
  }
  return out;
})();

// Promise cache — one in-flight load per canonical language. Subsequent
// callers await the same promise instead of firing another import().
const IN_FLIGHT = new Map<string, Promise<boolean>>();

// Map a user-supplied fenced-code tag ("ts", "Rust", "SH") to our canonical
// key. Returns null if we don't have a grammar for it — caller falls back
// to plain-escaped rendering.
export const canonicalLang = (lang: string): string | null => {
  if (!lang) return null;
  return ALIAS_TO_CANON[lang.toLowerCase()] ?? null;
};

// Register a language (and its aliases) on hljs. Triggers a dynamic import
// on first call; returns true once the language is usable. Resolves to
// false when the language is unknown or the import fails — callers should
// fall back to plain text rather than crashing.
export const ensureLanguage = (lang: string): Promise<boolean> => {
  const canon = canonicalLang(lang);
  if (!canon) return Promise.resolve(false);
  if (hljs.getLanguage(canon)) return Promise.resolve(true);
  const cached = IN_FLIGHT.get(canon);
  if (cached) return cached;
  const p = LANG_LOADERS[canon].load()
    .then(mod => {
      // Guard against a concurrent import winning the race.
      if (!hljs.getLanguage(canon)) {
        hljs.registerLanguage(canon, mod.default);
      }
      for (const alias of LANG_LOADERS[canon].names) {
        if (alias !== canon && !hljs.getLanguage(alias)) {
          hljs.registerLanguage(alias, mod.default);
        }
      }
      return true;
    })
    .catch(err => {
      // Don't keep a rejected promise in cache — let a future call retry.
      IN_FLIGHT.delete(canon);
      console.debug('// SCC: hljs language load failed:', canon, err);
      return false;
    });
  IN_FLIGHT.set(canon, p);
  return p;
};
