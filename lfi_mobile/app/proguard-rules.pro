# PlausiDen mobile — Proguard rules. The WebView JavascriptInterface (NativeBridge)
# is invoked reflectively from JS, so we must keep its annotated methods.
-keepclassmembers class com.lfi.sovereign.MainActivity$NativeBridge {
    @android.webkit.JavascriptInterface <methods>;
}
