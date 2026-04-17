# PlausiDen AI — Android Mobile Client

WebView wrapper that connects to a PlausiDen AI dashboard server on the local
network. Shows the same React UI as the desktop dashboard, with native
back-button handling and an in-app server-IP configuration dialog.

## Build (debug APK)

Requires Android SDK 34, JDK 17, and Gradle 8.x (or use the wrapper after
running `gradle wrapper` once on a host with Gradle installed).

```bash
cd /root/LFI/lfi_mobile
gradle wrapper                  # one-time, generates ./gradlew
./gradlew assembleDebug         # builds app/build/outputs/apk/debug/app-debug.apk
```

Sideload the APK to a device with USB debugging enabled:

```bash
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

## Configure server IP

First launch shows a dialog to enter the IP and port of your PlausiDen server
(default `192.168.1.186:5173`). The choice is persisted in SharedPreferences.

To re-open the config dialog later, tap **Change Server IP** on the offline
page (shown when the WebView fails to reach the server).

## Architecture

- `MainActivity.kt` — owns the WebView, network checks, in-app updater.
- `NativeBridge` (inner class) — exposes `PlausiDenNative.*` to JS so the web
  UI can request the native settings dialog.
- `network_security_config.xml` — restricts cleartext HTTP to local network
  ranges only (loopback, 10.\*, 172.16/12, 192.168/16). Public traffic still
  requires TLS.

## Hardening notes (AVP-2)

- `mixedContentMode = MIXED_CONTENT_ALWAYS_ALLOW` is intentional — the dashboard
  is served over HTTP on the LAN. Combined with the network-security config,
  cleartext is *only* permitted to private-IP destinations.
- The in-app updater fetches `releases/latest` over HTTPS from GitHub. The
  user must manually approve the install (Android default for sideloaded APKs).
- WebView `JavaScriptEnabled = true` is required by the dashboard. The
  `NativeBridge` exposes only two read-only getters and one settings dialog
  trigger — no FS, exec, or arbitrary intent dispatch.
