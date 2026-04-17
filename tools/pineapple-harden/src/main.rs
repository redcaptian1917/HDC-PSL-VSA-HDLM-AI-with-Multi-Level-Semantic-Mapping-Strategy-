//! # pineapple-harden — Adversary Identity Generator
//!
//! Generates realistic adversary identities for WiFi Pineapple tiered simulation.
//! Each tier represents a different adversary skill level:
//! - Tier 1: Script kiddie (default config, detectable by OUI/hostname/SSID)
//! - Tier 2: Intermediate (randomized MAC from real vendor, custom hostname/SSID)
//! - Tier 3: APT (full identity rotation, low power, beacon mimicry, no fingerprints)
//!
//! Outputs identity JSON for session provenance. Optionally applies via SSH.
//!
//! SECURITY: All operations target the Pineapple via SSH. NEVER touches wlan0.

use clap::Parser;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Adversary identity generator for WiFi Pineapple tiered simulation.
#[derive(Parser, Debug)]
#[command(name = "pineapple-harden", version, about)]
struct Args {
    /// Adversary tier: 1 (script kiddie), 2 (intermediate), 3 (APT)
    #[arg(short, long)]
    tier: u8,

    /// Session UUID (auto-generated if omitted)
    #[arg(short, long)]
    session_id: Option<String>,

    /// Apply configuration to Pineapple via SSH (requires key auth)
    #[arg(long, default_value_t = false)]
    apply: bool,

    /// Pineapple SSH host (from ~/.ssh/config)
    #[arg(long, default_value = "pineapple")]
    host: String,

    /// Output directory for session data
    #[arg(long, default_value = "~/lfi/sessions")]
    output_dir: String,

    /// Tier 3 variant profile (selects from beacon behavior library)
    #[arg(long)]
    variant: Option<String>,
}

/// Real vendor OUI database — these must be actual vendor prefixes.
/// BUG ASSUMPTION: If a vendor retires an OUI, our generated MACs could
/// become detectable. Update this list periodically from IEEE OUI database.
const VENDOR_OUIS: &[VendorOui] = &[
    VendorOui { oui: [0xA4, 0x83, 0xE7], vendor: "Apple", hostname_pattern: "iPhone" },
    VendorOui { oui: [0x3C, 0x06, 0x30], vendor: "Apple", hostname_pattern: "MacBook" },
    VendorOui { oui: [0xF0, 0x18, 0x98], vendor: "Apple", hostname_pattern: "iPad" },
    VendorOui { oui: [0x8C, 0x85, 0x90], vendor: "Samsung", hostname_pattern: "Galaxy-S24" },
    VendorOui { oui: [0xB0, 0x72, 0xBF], vendor: "Samsung", hostname_pattern: "Galaxy-A54" },
    VendorOui { oui: [0x94, 0xE9, 0x79], vendor: "Samsung", hostname_pattern: "SM-G998" },
    VendorOui { oui: [0x3C, 0x28, 0x6D], vendor: "Intel", hostname_pattern: "DESKTOP" },
    VendorOui { oui: [0x48, 0xA4, 0x72], vendor: "Intel", hostname_pattern: "NUC" },
    VendorOui { oui: [0xDC, 0x71, 0x96], vendor: "Google", hostname_pattern: "Pixel-8" },
    VendorOui { oui: [0xF4, 0xF5, 0xDB], vendor: "Google", hostname_pattern: "Pixel-7a" },
    VendorOui { oui: [0x64, 0xB4, 0x73], vendor: "Xiaomi", hostname_pattern: "Redmi-Note" },
    VendorOui { oui: [0x50, 0x8A, 0x06], vendor: "Xiaomi", hostname_pattern: "Mi-14" },
    VendorOui { oui: [0x9C, 0x2E, 0xA1], vendor: "Huawei", hostname_pattern: "HUAWEI-P60" },
    VendorOui { oui: [0x04, 0xBA, 0xD6], vendor: "Huawei", hostname_pattern: "MatePad" },
    VendorOui { oui: [0x60, 0xBE, 0xB5], vendor: "Motorola", hostname_pattern: "moto-g" },
    VendorOui { oui: [0xE8, 0x6F, 0x38], vendor: "OnePlus", hostname_pattern: "OnePlus-12" },
];

struct VendorOui {
    oui: [u8; 3],
    vendor: &'static str,
    hostname_pattern: &'static str,
}

/// Realistic SSID templates per cover identity.
const COVER_SSIDS: &[&str] = &[
    "NETGEAR-5G", "ATT-WIFI-{rand4}", "xfinitywifi",
    "Verizon_{rand4}", "DIRECT-{rand2}-HP", "CenturyLink{rand4}",
    "HOME-{rand4}", "MySpectrumWiFi{rand2}", "linksys_{rand4}",
    "TP-Link_{rand4}", "ASUS_5G_{rand4}", "Google_Wifi_{rand2}",
];

/// Tier 3 beacon behavior variants.
const TIER3_VARIANTS: &[Tier3Variant] = &[
    Tier3Variant {
        name: "quiet_apple",
        beacon_interval_ms: 100,
        active_window_secs: 300,
        silent_window_secs: 600,
        description: "Apple device profile, intermittent beacons",
    },
    Tier3Variant {
        name: "samsung_persistent",
        beacon_interval_ms: 100,
        active_window_secs: 900,
        silent_window_secs: 300,
        description: "Samsung hotspot profile, mostly active",
    },
    Tier3Variant {
        name: "burst_and_hide",
        beacon_interval_ms: 50,
        active_window_secs: 60,
        silent_window_secs: 1800,
        description: "Brief burst capture then long silence",
    },
    Tier3Variant {
        name: "slow_exfil",
        beacon_interval_ms: 200,
        active_window_secs: 3600,
        silent_window_secs: 0,
        description: "Low-rate persistent presence for data exfiltration",
    },
];

struct Tier3Variant {
    name: &'static str,
    beacon_interval_ms: u32,
    active_window_secs: u32,
    silent_window_secs: u32,
    description: &'static str,
}

/// The generated adversary identity — full provenance for every session.
#[derive(Serialize, Deserialize, Debug)]
struct AdversaryIdentity {
    session_id: String,
    tier: u8,
    timestamp: String,
    generator_version: String,
    authorization_basis: String,

    // Radio configuration
    radios: Vec<RadioConfig>,

    // Cover identity
    hostname: String,
    vendor: String,

    // PineAP filter config
    pineap_ssid_filter: FilterConfig,
    pineap_client_filter: FilterConfig,

    // Tier 3 specific
    variant: Option<String>,
    beacon_interval_ms: Option<u32>,
    active_window_secs: Option<u32>,
    silent_window_secs: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RadioConfig {
    radio: String,
    mac: String,
    ssid: String,
    channel: u8,
    tx_power_dbm: u8,
    mode: String,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FilterConfig {
    mode: String, // "allow" or "deny"
    entries: Vec<String>,
}

fn generate_mac(rng: &mut impl Rng, oui: &[u8; 3]) -> String {
    let nic: [u8; 3] = rng.gen();
    format!(
        "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        oui[0], oui[1], oui[2], nic[0], nic[1], nic[2]
    )
}

fn generate_ssid(rng: &mut impl Rng, template: &str) -> String {
    let mut ssid = template.to_string();
    while ssid.contains("{rand2}") {
        let r: u16 = rng.gen_range(10..100);
        ssid = ssid.replacen("{rand2}", &r.to_string(), 1);
    }
    while ssid.contains("{rand4}") {
        let r: u16 = rng.gen_range(1000..10000);
        ssid = ssid.replacen("{rand4}", &r.to_string(), 1);
    }
    ssid
}

fn generate_identity(tier: u8, session_id: &str, variant: Option<&str>) -> AdversaryIdentity {
    let mut rng = rand::thread_rng();

    match tier {
        1 => {
            // Tier 1: Script kiddie — default everything, maximum detectability
            let hak5_oui = [0x00, 0x13, 0x37];
            AdversaryIdentity {
                session_id: session_id.to_string(),
                tier: 1,
                timestamp: chrono::Utc::now().to_rfc3339(),
                generator_version: env!("CARGO_PKG_VERSION").to_string(),
                authorization_basis: "controlled_home_lab".to_string(),
                radios: vec![
                    RadioConfig {
                        radio: "radio0".into(), mac: generate_mac(&mut rng, &hak5_oui),
                        ssid: "Pineapple_2G".into(), channel: 11, tx_power_dbm: 15,
                        mode: "ap".into(), enabled: true,
                    },
                    RadioConfig {
                        radio: "radio1".into(), mac: generate_mac(&mut rng, &hak5_oui),
                        ssid: String::new(), channel: 6, tx_power_dbm: 15,
                        mode: "monitor".into(), enabled: true,
                    },
                ],
                hostname: "mk7".into(),
                vendor: "Hak5".into(),
                pineap_ssid_filter: FilterConfig { mode: "deny".into(), entries: vec![] },
                pineap_client_filter: FilterConfig { mode: "deny".into(), entries: vec![] },
                variant: None,
                beacon_interval_ms: Some(100),
                active_window_secs: None,
                silent_window_secs: None,
            }
        }
        2 => {
            // Tier 2: Intermediate — randomized vendor MAC, custom identity
            let vendor = &VENDOR_OUIS[rng.gen_range(0..VENDOR_OUIS.len())];
            let ssid_template = COVER_SSIDS[rng.gen_range(0..COVER_SSIDS.len())];
            AdversaryIdentity {
                session_id: session_id.to_string(),
                tier: 2,
                timestamp: chrono::Utc::now().to_rfc3339(),
                generator_version: env!("CARGO_PKG_VERSION").to_string(),
                authorization_basis: "controlled_home_lab".to_string(),
                radios: vec![
                    RadioConfig {
                        radio: "radio0".into(), mac: generate_mac(&mut rng, &vendor.oui),
                        ssid: generate_ssid(&mut rng, ssid_template),
                        channel: *[1, 6, 11].choose(&mut rng).unwrap_or(&6),
                        tx_power_dbm: rng.gen_range(10..=12),
                        mode: "ap".into(), enabled: true,
                    },
                    RadioConfig {
                        radio: "radio1".into(), mac: generate_mac(&mut rng, &vendor.oui),
                        ssid: String::new(),
                        channel: *[1, 6, 11].choose(&mut rng).unwrap_or(&6),
                        tx_power_dbm: 10,
                        mode: "monitor".into(), enabled: true,
                    },
                ],
                hostname: vendor.hostname_pattern.to_string(),
                vendor: vendor.vendor.to_string(),
                pineap_ssid_filter: FilterConfig {
                    mode: "allow".into(),
                    entries: vec![generate_ssid(&mut rng, ssid_template)],
                },
                pineap_client_filter: FilterConfig {
                    mode: "allow".into(),
                    entries: vec![format!("{}:*", &generate_mac(&mut rng, &vendor.oui)[..8])],
                },
                variant: None,
                beacon_interval_ms: Some(100),
                active_window_secs: None,
                silent_window_secs: None,
            }
        }
        3 => {
            // Tier 3: APT — full identity rotation, low power, beacon mimicry
            let vendor = &VENDOR_OUIS[rng.gen_range(0..VENDOR_OUIS.len())];
            let ssid_template = COVER_SSIDS[rng.gen_range(0..COVER_SSIDS.len())];
            let v = variant
                .and_then(|name| TIER3_VARIANTS.iter().find(|v| v.name == name))
                .unwrap_or(&TIER3_VARIANTS[rng.gen_range(0..TIER3_VARIANTS.len())]);

            // Security tool OUIs to deny (other Pineapples, Pwnagotchi, etc.)
            let deny_ouis = vec![
                "00:13:37:*".to_string(), // Hak5
                "00:13:EF:*".to_string(), // Pwnagotchi
                "DE:AD:BE:*".to_string(), // Common pen-test placeholder
            ];

            AdversaryIdentity {
                session_id: session_id.to_string(),
                tier: 3,
                timestamp: chrono::Utc::now().to_rfc3339(),
                generator_version: env!("CARGO_PKG_VERSION").to_string(),
                authorization_basis: "controlled_home_lab".to_string(),
                radios: vec![
                    RadioConfig {
                        radio: "radio0".into(), mac: generate_mac(&mut rng, &vendor.oui),
                        ssid: generate_ssid(&mut rng, ssid_template),
                        channel: *[1, 6, 11].choose(&mut rng).unwrap_or(&6),
                        tx_power_dbm: rng.gen_range(5..=8),
                        mode: "ap".into(), enabled: true,
                    },
                    RadioConfig {
                        radio: "radio1".into(), mac: generate_mac(&mut rng, &vendor.oui),
                        ssid: String::new(),
                        channel: *[1, 6, 11].choose(&mut rng).unwrap_or(&6),
                        tx_power_dbm: 5,
                        mode: "monitor".into(), enabled: true,
                    },
                ],
                hostname: format!("{}-{}", vendor.hostname_pattern, rng.gen_range(100..999)),
                vendor: vendor.vendor.to_string(),
                pineap_ssid_filter: FilterConfig {
                    mode: "allow".into(),
                    entries: vec![generate_ssid(&mut rng, ssid_template)],
                },
                pineap_client_filter: FilterConfig {
                    mode: "deny".into(),
                    entries: deny_ouis,
                },
                variant: Some(v.name.to_string()),
                beacon_interval_ms: Some(v.beacon_interval_ms),
                active_window_secs: Some(v.active_window_secs),
                silent_window_secs: Some(v.silent_window_secs),
            }
        }
        _ => panic!("Invalid tier: {}. Must be 1, 2, or 3.", tier),
    }
}

fn main() {
    let args = Args::parse();

    if args.tier < 1 || args.tier > 3 {
        eprintln!("Error: tier must be 1, 2, or 3");
        std::process::exit(1);
    }

    let session_id = args.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let identity = generate_identity(args.tier, &session_id, args.variant.as_deref());

    // Create session directory
    let output_dir = args.output_dir.replace("~", &std::env::var("HOME").unwrap_or("/root".into()));
    let session_dir = PathBuf::from(&output_dir).join(&session_id);
    std::fs::create_dir_all(&session_dir).expect("Failed to create session directory");

    // Write identity JSON
    let identity_path = session_dir.join("adversary_identity.json");
    let json = serde_json::to_string_pretty(&identity).expect("Failed to serialize identity");
    std::fs::write(&identity_path, &json).expect("Failed to write identity file");

    println!("=== Adversary Identity Generated ===");
    println!("Session:  {}", session_id);
    println!("Tier:     {} ({})", args.tier, match args.tier {
        1 => "script kiddie",
        2 => "intermediate",
        3 => "APT",
        _ => "unknown",
    });
    println!("Vendor:   {}", identity.vendor);
    println!("Hostname: {}", identity.hostname);
    for r in &identity.radios {
        println!("  {}: MAC={} SSID={} CH={} TX={}dBm [{}]",
            r.radio, r.mac, if r.ssid.is_empty() { "(monitor)" } else { &r.ssid },
            r.channel, r.tx_power_dbm, r.mode);
    }
    if let Some(ref v) = identity.variant {
        println!("Variant:  {}", v);
    }
    println!("Saved:    {}", identity_path.display());

    if args.apply {
        println!("\n--- Applying to Pineapple via SSH ---");
        apply_to_pineapple(&args.host, &identity);
    }
}

fn apply_to_pineapple(host: &str, identity: &AdversaryIdentity) {
    // BUG ASSUMPTION: SSH connection could fail, uci commands could fail,
    // wifi reload could leave radios in inconsistent state. Verify after.
    use std::process::Command;

    let mut commands = Vec::new();

    for radio in &identity.radios {
        // Set MAC address
        commands.push(format!("iw dev {} set addr {} 2>/dev/null || ip link set {} address {}",
            radio.radio.replace("radio", "wlan"), radio.mac,
            radio.radio.replace("radio", "wlan"), radio.mac));

        // Set TX power
        commands.push(format!("iw dev {} set txpower fixed {}00",
            radio.radio.replace("radio", "wlan"), radio.tx_power_dbm));

        if radio.mode == "ap" && !radio.ssid.is_empty() {
            // Set SSID via uci
            let iface_idx = radio.radio.replace("radio", "");
            commands.push(format!("uci set wireless.default_radio{}.ssid='{}'", iface_idx, radio.ssid));
        }
    }

    // Set hostname
    commands.push(format!("uci set system.@system[0].hostname='{}'", identity.hostname));

    // Commit and reload
    commands.push("uci commit".to_string());
    commands.push("wifi reload".to_string());

    // Join all commands
    let full_cmd = commands.join(" && ");

    println!("Executing on {}: {} commands", host, commands.len());
    let output = Command::new("ssh")
        .args(&["-o", "BatchMode=yes", "-o", "ConnectTimeout=10", host, &full_cmd])
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                println!("Applied successfully.");
                // Verify
                let verify = Command::new("ssh")
                    .args(&["-o", "BatchMode=yes", host,
                        "iw dev wlan0 info 2>/dev/null; iw dev wlan1 info 2>/dev/null; uname -n"])
                    .output();
                if let Ok(v) = verify {
                    println!("--- Verification ---");
                    println!("{}", String::from_utf8_lossy(&v.stdout));
                }
            } else {
                eprintln!("SSH command failed: {}", String::from_utf8_lossy(&o.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("SSH execution failed: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier1_identity_has_hak5_oui() {
        let id = generate_identity(1, "test-session", None);
        assert_eq!(id.tier, 1);
        assert_eq!(id.hostname, "mk7");
        assert_eq!(id.vendor, "Hak5");
        for r in &id.radios {
            assert!(r.mac.starts_with("00:13:37:"), "Tier 1 should use Hak5 OUI, got {}", r.mac);
        }
    }

    #[test]
    fn test_tier2_identity_no_hak5() {
        let id = generate_identity(2, "test-session", None);
        assert_eq!(id.tier, 2);
        assert_ne!(id.hostname, "mk7");
        assert_ne!(id.vendor, "Hak5");
        for r in &id.radios {
            assert!(!r.mac.starts_with("00:13:37:"), "Tier 2 must NOT use Hak5 OUI, got {}", r.mac);
            assert!(r.tx_power_dbm <= 12, "Tier 2 TX should be <=12dBm, got {}", r.tx_power_dbm);
        }
    }

    #[test]
    fn test_tier3_identity_covert() {
        let id = generate_identity(3, "test-session", None);
        assert_eq!(id.tier, 3);
        assert_ne!(id.vendor, "Hak5");
        assert!(id.variant.is_some(), "Tier 3 must have a variant");
        for r in &id.radios {
            assert!(!r.mac.starts_with("00:13:37:"), "Tier 3 must NOT use Hak5 OUI");
            assert!(r.tx_power_dbm <= 8, "Tier 3 TX should be <=8dBm, got {}", r.tx_power_dbm);
        }
        // Must deny security tool OUIs
        let deny = &id.pineap_client_filter.entries;
        assert!(deny.iter().any(|e| e.contains("00:13:37")), "Tier 3 must deny Hak5 OUI");
    }

    #[test]
    fn test_tier3_variant_selection() {
        let id = generate_identity(3, "test-session", Some("burst_and_hide"));
        assert_eq!(id.variant.as_deref(), Some("burst_and_hide"));
        assert_eq!(id.active_window_secs, Some(60));
        assert_eq!(id.silent_window_secs, Some(1800));
    }

    #[test]
    fn test_mac_format_valid() {
        let mut rng = rand::thread_rng();
        let oui = [0xA4, 0x83, 0xE7];
        let mac = generate_mac(&mut rng, &oui);
        assert_eq!(mac.len(), 17, "MAC should be 17 chars (XX:XX:XX:XX:XX:XX)");
        assert!(mac.starts_with("A4:83:E7:"), "MAC should start with OUI");
    }

    #[test]
    fn test_ssid_generation() {
        let mut rng = rand::thread_rng();
        let ssid = generate_ssid(&mut rng, "ATT-WIFI-{rand4}");
        assert!(ssid.starts_with("ATT-WIFI-"), "SSID should match template prefix");
        assert!(ssid.len() > "ATT-WIFI-".len(), "SSID should have random suffix");
    }

    #[test]
    fn test_authorization_basis_always_set() {
        for tier in 1..=3 {
            let id = generate_identity(tier, "test", None);
            assert_eq!(id.authorization_basis, "controlled_home_lab");
        }
    }

    #[test]
    fn test_all_tiers_produce_unique_sessions() {
        let id1 = generate_identity(2, "session-a", None);
        let id2 = generate_identity(2, "session-b", None);
        // Different sessions should have different MACs (with overwhelming probability)
        assert_ne!(id1.radios[0].mac, id2.radios[0].mac);
    }
}
