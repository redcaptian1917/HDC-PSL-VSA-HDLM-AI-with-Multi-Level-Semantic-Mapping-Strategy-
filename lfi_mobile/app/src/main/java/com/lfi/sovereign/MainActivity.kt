package com.lfi.sovereign

import android.annotation.SuppressLint
import android.app.AlertDialog
import android.app.DownloadManager
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import android.net.Uri
import android.os.Bundle
import android.os.Environment
import android.view.KeyEvent
import android.webkit.*
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.Toast
import androidx.activity.ComponentActivity
import kotlinx.coroutines.*
import org.json.JSONObject
import java.net.URL

/**
 * PlausiDen AI Mobile Client
 *
 * WebView wrapper connecting to the PlausiDen dashboard server.
 * Features:
 * - Configurable server IP (stored in SharedPreferences)
 * - Auto-update from GitHub Releases
 * - Offline detection with retry
 * - Back button navigation within WebView
 * - Pull-to-refresh via swipe
 */
class MainActivity : ComponentActivity() {

    private lateinit var webView: WebView
    private lateinit var prefs: SharedPreferences

    companion object {
        const val PREF_NAME = "plausiden_prefs"
        const val PREF_SERVER_IP = "server_ip"
        const val PREF_SERVER_PORT = "server_port"
        const val DEFAULT_IP = "192.168.1.186"
        const val DEFAULT_PORT = "5173"
        const val UPDATE_URL = "https://api.github.com/repos/thepictishbeast/PlausiDen-Mobile/releases/latest"
        const val CURRENT_VERSION = "0.1.0"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        prefs = getSharedPreferences(PREF_NAME, Context.MODE_PRIVATE)

        // Check if server IP is configured
        if (!prefs.contains(PREF_SERVER_IP)) {
            showServerConfigDialog()
        } else {
            setupWebView()
        }

        // Check for updates in background
        checkForUpdates()
    }

    @SuppressLint("SetJavaScriptEnabled")
    private fun setupWebView() {
        webView = WebView(this)
        setContentView(webView)

        webView.settings.apply {
            javaScriptEnabled = true
            domStorageEnabled = true
            allowContentAccess = true
            mediaPlaybackRequiresUserGesture = false
            cacheMode = WebSettings.LOAD_DEFAULT
            // Allow mixed content for local network
            mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
            useWideViewPort = true
            loadWithOverviewMode = true
            setSupportZoom(true)
            builtInZoomControls = true
            displayZoomControls = false
        }

        webView.webViewClient = object : WebViewClient() {
            override fun onReceivedError(
                view: WebView?,
                request: WebResourceRequest?,
                error: WebResourceError?
            ) {
                if (request?.isForMainFrame == true) {
                    showOfflinePage()
                }
            }

            override fun shouldOverrideUrlLoading(
                view: WebView?,
                request: WebResourceRequest?
            ): Boolean {
                val url = request?.url?.toString() ?: return false
                // Keep navigation within our server
                val serverUrl = getServerUrl()
                return if (url.startsWith(serverUrl) || url.startsWith("http://localhost")) {
                    false // Let WebView handle it
                } else {
                    // External link — open in browser
                    startActivity(Intent(Intent.ACTION_VIEW, Uri.parse(url)))
                    true
                }
            }
        }

        webView.webChromeClient = WebChromeClient()

        // Add JavaScript interface for native features
        webView.addJavascriptInterface(NativeBridge(this), "PlausiDenNative")

        loadDashboard()
    }

    private fun loadDashboard() {
        val url = getServerUrl()
        if (isNetworkAvailable()) {
            webView.loadUrl(url)
        } else {
            showOfflinePage()
        }
    }

    private fun getServerUrl(): String {
        val ip = prefs.getString(PREF_SERVER_IP, DEFAULT_IP) ?: DEFAULT_IP
        val port = prefs.getString(PREF_SERVER_PORT, DEFAULT_PORT) ?: DEFAULT_PORT
        return "http://$ip:$port"
    }

    private fun showOfflinePage() {
        val html = """
            <!DOCTYPE html>
            <html>
            <head>
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <style>
                    body {
                        background: #050505;
                        color: #9CA3AF;
                        font-family: monospace;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        height: 100vh;
                        margin: 0;
                    }
                    h1 { color: #3B82F6; font-size: 24px; }
                    p { text-align: center; max-width: 300px; line-height: 1.6; }
                    button {
                        background: #3B82F6;
                        color: white;
                        border: none;
                        padding: 12px 32px;
                        font-family: monospace;
                        font-size: 14px;
                        border-radius: 4px;
                        margin-top: 20px;
                        cursor: pointer;
                    }
                    .settings {
                        margin-top: 16px;
                        color: #6B7280;
                        text-decoration: underline;
                        cursor: pointer;
                    }
                </style>
            </head>
            <body>
                <h1>PLAUSIDEN AI</h1>
                <p>Cannot reach the PlausiDen server.<br><br>
                Make sure the server is running and you're on the same network.</p>
                <button onclick="location.reload()">RETRY CONNECTION</button>
                <p class="settings" onclick="PlausiDenNative.showSettings()">Change Server IP</p>
            </body>
            </html>
        """.trimIndent()
        webView.loadDataWithBaseURL(null, html, "text/html", "UTF-8", null)
    }

    private fun showServerConfigDialog() {
        val layout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(50, 40, 50, 10)
        }

        val ipInput = EditText(this).apply {
            hint = "Server IP (e.g., 192.168.1.186)"
            setText(prefs.getString(PREF_SERVER_IP, DEFAULT_IP))
        }
        val portInput = EditText(this).apply {
            hint = "Port (e.g., 5173)"
            setText(prefs.getString(PREF_SERVER_PORT, DEFAULT_PORT))
        }

        layout.addView(ipInput)
        layout.addView(portInput)

        AlertDialog.Builder(this)
            .setTitle("PlausiDen Server")
            .setMessage("Enter the IP and port of your PlausiDen AI server:")
            .setView(layout)
            .setPositiveButton("Connect") { _, _ ->
                val ip = ipInput.text.toString().trim()
                val port = portInput.text.toString().trim()
                if (ip.isNotEmpty()) {
                    prefs.edit()
                        .putString(PREF_SERVER_IP, ip)
                        .putString(PREF_SERVER_PORT, port.ifEmpty { DEFAULT_PORT })
                        .apply()
                    setupWebView()
                }
            }
            .setCancelable(false)
            .show()
    }

    private fun isNetworkAvailable(): Boolean {
        val cm = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        val network = cm.activeNetwork ?: return false
        val caps = cm.getNetworkCapabilities(network) ?: return false
        return caps.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
    }

    private fun checkForUpdates() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val json = URL(UPDATE_URL).readText()
                val release = JSONObject(json)
                val latestVersion = release.getString("tag_name").removePrefix("v")
                if (latestVersion != CURRENT_VERSION) {
                    val assets = release.getJSONArray("assets")
                    for (i in 0 until assets.length()) {
                        val asset = assets.getJSONObject(i)
                        if (asset.getString("name").endsWith(".apk")) {
                            val downloadUrl = asset.getString("browser_download_url")
                            withContext(Dispatchers.Main) {
                                showUpdateDialog(latestVersion, downloadUrl)
                            }
                            break
                        }
                    }
                }
            } catch (_: Exception) {
                // Silent fail — update check is best-effort
            }
        }
    }

    private fun showUpdateDialog(version: String, downloadUrl: String) {
        AlertDialog.Builder(this)
            .setTitle("Update Available")
            .setMessage("PlausiDen AI v$version is available. Update now?")
            .setPositiveButton("Update") { _, _ ->
                downloadAndInstallUpdate(downloadUrl)
            }
            .setNegativeButton("Later", null)
            .show()
    }

    private fun downloadAndInstallUpdate(url: String) {
        val request = DownloadManager.Request(Uri.parse(url))
            .setTitle("PlausiDen AI Update")
            .setDescription("Downloading update...")
            .setNotificationVisibility(DownloadManager.Request.VISIBILITY_VISIBLE_NOTIFY_COMPLETED)
            .setDestinationInExternalPublicDir(Environment.DIRECTORY_DOWNLOADS, "plausiden-ai.apk")

        val dm = getSystemService(Context.DOWNLOAD_SERVICE) as DownloadManager
        dm.enqueue(request)
        Toast.makeText(this, "Download started. Install from Downloads when complete.", Toast.LENGTH_LONG).show()
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK && ::webView.isInitialized && webView.canGoBack()) {
            webView.goBack()
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    /**
     * JavaScript interface for native Android features.
     * Callable from web: PlausiDenNative.showSettings()
     */
    class NativeBridge(private val activity: MainActivity) {
        @JavascriptInterface
        fun showSettings() {
            activity.runOnUiThread {
                activity.showServerConfigDialog()
            }
        }

        @JavascriptInterface
        fun getServerIp(): String {
            return activity.prefs.getString(PREF_SERVER_IP, DEFAULT_IP) ?: DEFAULT_IP
        }

        @JavascriptInterface
        fun getVersion(): String = CURRENT_VERSION
    }
}
