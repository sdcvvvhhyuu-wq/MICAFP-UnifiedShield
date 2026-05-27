// ─────────────────────────────────────────────────────────────────────────────
// Transport subsystem — All 22 protocols from all 13 source projects
// MICAFP-UnifiedShield-vip-ultra-Quantum-ultra v8.0
// ─────────────────────────────────────────────────────────────────────────────

pub mod cdn_tunnel;
pub mod cdn_worker;
pub mod chinese_cdn;
pub mod cloudflare_worker;
pub mod doh_tunnel;
pub mod domain_fronting;
pub mod doq_tunnel;
pub mod hysteria2;
pub mod icmp_tunnel;
pub mod manager;
pub mod meek;
pub mod mqtt_tunnel;
pub mod mqtt_ws;
pub mod multihop_chain;
pub mod naive_proxy;
pub mod pluggable_transport;
pub mod reality;
pub mod shadow_tls;
pub mod tuic_v5;
pub mod vless;
pub mod webrtc_relay;
pub mod webtransport;

use crate::error::ShieldError;
use async_trait::async_trait;
use std::sync::Arc;

pub use manager::TransportManager;
pub use multihop_chain::MultiHopChainTransport;

/// Common trait implemented by every transport protocol variant.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Human-readable protocol name (e.g. "hysteria2", "cdn-worker").
    fn name(&self) -> &'static str;

    /// Connect and return an opaque byte-stream handle.
    async fn connect(&self) -> Result<Box<dyn TransportStream>, ShieldError>;

    /// Returns `true` when this transport should be considered for the
    /// current network environment (DPI level, ISP fingerprint, etc.).
    fn is_suitable(&self, dpi_level: u8) -> bool;

    /// Estimated latency in milliseconds (used by the load-balancer).
    fn latency_ms(&self) -> u32 { 200 }
}

/// Byte-stream abstraction returned by every `Transport::connect` call.
#[async_trait]
pub trait TransportStream: Send + Sync {
    async fn send(&mut self, data: &[u8]) -> Result<(), ShieldError>;
    async fn recv(&mut self) -> Result<Vec<u8>, ShieldError>;
    async fn close(&mut self) -> Result<(), ShieldError>;
}

/// All protocol variants — used by the orchestrator for ordered fallback.
pub fn all_protocols() -> Vec<&'static str> {
    vec![
        "vless",
        "reality",
        "hysteria2",
        "tuic-v5",
        "shadow-tls",
        "naive-proxy",
        "cloudflare-worker",
        "cdn-worker",
        "cdn-tunnel",
        "chinese-cdn",
        "domain-fronting",
        "meek",
        "doh-tunnel",
        "doq-tunnel",
        "webrtc-relay",
        "webtransport",
        "mqtt-tunnel",
        "mqtt-ws",
        "icmp-tunnel",
        "pluggable-transport",
        "multihop-chain",
    ]
}

/// Returns the ordered fallback list for a given DPI level (0=none … 5=AI-DPI).
pub fn fallback_order(dpi_level: u8) -> Vec<&'static str> {
    match dpi_level {
        0 | 1 => vec!["vless", "hysteria2", "tuic-v5"],
        2 | 3 => vec![
            "reality", "shadow-tls", "cdn-worker", "cloudflare-worker",
            "domain-fronting", "meek",
        ],
        4 | 5 => vec![
            "webrtc-relay", "webtransport", "mqtt-ws", "doh-tunnel",
            "doq-tunnel", "icmp-tunnel", "multihop-chain",
        ],
        _ => all_protocols(),
    }
}

/// Arc-wrapped handle to a connected `TransportManager`.
pub type ArcTransportManager = Arc<TransportManager>;

// ── Re-exports for submodule convenience ────────────────────────────────────
// submodules use `use super::{ShieldError, Transport, TransportConnection};`
pub use crate::error::ShieldError;

/// Alias — TransportStream is the byte-stream abstraction.
/// Some submodules refer to it as TransportConnection.

/// A connected transport byte-stream handle.
/// Alias of TransportStream for backward compatibility.
pub trait TransportConnection: Send + Sync {
    fn send_bytes(&mut self, data: &[u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;
    fn recv_bytes(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send + '_>>;
    fn close_conn(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;
}

// ── Re-exports for submodule convenience (continued) ────────────────────────

/// ISP profile re-exported for transport submodules.
pub use crate::config::isp_profile::IspProfile;

/// Battery state re-exported for transport submodules.
pub use crate::ipc::BatteryState;

/// Statistics for a single transport endpoint.
#[derive(Debug, Clone, Default)]
pub struct EndpointStats {
    pub attempts: u32,
    pub successes: u32,
    pub failures: u32,
    pub avg_latency_ms: f64,
    pub last_attempt_ts: u64,
    pub last_success_ts: u64,
}

impl EndpointStats {
    pub fn success_rate(&self) -> f64 {
        if self.attempts == 0 { 0.5 } else { self.successes as f64 / self.attempts as f64 }
    }
}

/// Transport weight for the load balancer.
#[derive(Debug, Clone)]
pub struct TransportWeight {
    pub name: String,
    pub weight: u32,
    pub current_weight: i64,
}

/// Exponential backoff with full jitter.
pub fn exponential_backoff_with_jitter(attempt: u32, base_ms: u64, max_ms: u64) -> std::time::Duration {
    use rand::Rng;
    let cap = (base_ms * 2u64.pow(attempt)).min(max_ms);
    let jitter = rand::thread_rng().gen_range(0..=cap);
    std::time::Duration::from_millis(jitter)
}
