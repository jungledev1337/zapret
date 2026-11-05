use std::path::Path;
use zapret_lib::config::AppConfig;
use zapret_lib::engine::{build_engine_args, compute_ports};

#[test]
fn test_compute_ports_discord_only() {
    let (tcp, udp) = compute_ports(true, false);
    assert!(tcp.contains(&"80"));
    assert!(tcp.contains(&"443"));
    assert!(tcp.contains(&"2053"));
    assert!(tcp.contains(&"8443"));
    assert!(udp.contains(&"19294-19344"));
    assert!(udp.contains(&"50000-50100"));
}

#[test]
fn test_compute_ports_telegram_only() {
    let (tcp, udp) = compute_ports(false, true);
    assert!(tcp.contains(&"80"));
    assert!(tcp.contains(&"443"));
    assert!(tcp.contains(&"8443"));
    assert_eq!(udp, vec!["443"]);
}

#[test]
fn test_build_engine_args_structure() {
    let bin_dir = Path::new("bin");
    let lists_dir = Path::new("lists");

    let args = build_engine_args(bin_dir, lists_dir, true, true, false);

    assert!(args.iter().any(|a| a.starts_with("--wf-tcp=")));
    assert!(args.iter().any(|a| a.starts_with("--wf-udp=")));
    assert!(args.iter().any(|a| a.contains("ACTIVE_DISCORD_UDP.bin")));
    assert!(args.iter().any(|a| a.contains("ipset-all.txt")));
}

#[test]
fn test_build_engine_args_youtube_adblock() {
    let bin_dir = Path::new("bin");
    let lists_dir = Path::new("lists");

    let args = build_engine_args(bin_dir, lists_dir, false, false, true);

    assert!(args.iter().any(|a| a.contains("list-adblock.txt")));
    assert!(args.iter().any(|a| a == "--dpi-desync=rst"));
    assert!(args.iter().any(|a| a.contains("list-google.txt")));
}

#[test]
fn test_config_serialization() {
    let cfg = AppConfig {
        discord: false,
        telegram: true,
        youtube: false,
    };

    let json = serde_json::to_string(&cfg).expect("Serialization failed");
    let decoded: AppConfig = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(cfg, decoded);
}
