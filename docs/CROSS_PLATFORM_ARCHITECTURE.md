# Cross-Platform Frontend Architecture

## The Problem
Three frontends (web, desktop, Android) must look and behave identically.
Changes to one must propagate to all. No drift.

## Solution: One Codebase, Three Delivery Mechanisms

```
lfi_dashboard/          ← Single React codebase (source of truth)
  src/
    design-system.ts    ← Colors, typography, spacing, components
    App.tsx             ← Main application
    ...

Delivery:
  1. Web:     vite dev server → localhost:5173
  2. Desktop: Tauri wraps the same build → native window
  3. Android: WebView loads the same build → APK
```

All three load the EXACT same React app. The only differences:
- Desktop: Tauri adds native window chrome, system tray, auto-update
- Android: WebView adds back button, server IP config, offline page
- Web: Direct browser access, no wrapper

## Design System (design-system.ts)
Single source of truth for ALL visual properties:
- Color palette (slate neutrals + blue accent)
- Dark/Light theme tokens
- Typography (system fonts, mono for code)
- Spacing (8px grid)
- Border radius, shadows, z-index
- Component presets (card, button, input, chat bubble, modal)
- Animation timings

NO hardcoded colors or sizes anywhere else. Everything references design-system.ts.

## Keeping in Sync
1. Web dashboard IS the source — desktop and Android wrap it
2. `npm run build` produces the dist that ALL platforms use
3. Desktop: Tauri loads from dist/ folder
4. Android: WebView loads from server IP (which serves the same dist/)
5. Theme changes in design-system.ts automatically apply everywhere

## Auto-Update Strategy
- Web: Always latest (served from server)
- Desktop: Tauri updater checks GitHub Releases, downloads new binary
- Android: In-app updater checks GitHub Releases, downloads new APK

## Platform-Specific Adaptations
- Touch targets: 44px minimum on mobile (already in design-system)
- Sidebar: Collapsible on mobile, permanent on desktop
- Keyboard shortcuts: Desktop only
- System tray: Desktop only
- Server IP config: Android only
