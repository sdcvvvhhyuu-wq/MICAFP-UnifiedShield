// ─────────────────────────────────────────────────────────────────────────────
// MICAFP-UnifiedShield-vip-ultra-Quantum-ultra v8.0 — Error Types
// Complete unified error system for all 13 source projects.
// ─────────────────────────────────────────────────────────────────────────────

use thiserror::Error;

/// Structured error codes for every failure mode in UnifiedShield.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    // ── IPC Errors (1xxx) ────────────────────────────────────────────────────
    IpcConnectionFailed   = 1001,
    IpcChannelClosed      = 1002,
    IpcMessageParseError  = 1003,
    IpcTimeout            = 1004,

    // ── Transport Errors (2xxx) ──────────────────────────────────────────────
    TransportConnectionFailed = 2001,
    TransportTimeout          = 2002,
    AllTransportsExhausted    = 2003,
    DpiBlockDetected          = 2004,

    // ── Config Errors (3xxx) ─────────────────────────────────────────────────
    ConfigParseFailed     = 3001,
    ConfigNotFound        = 3002,
    ConfigUpdateFailed    = 3003,

    // ── Crypto Errors (4xxx) ─────────────────────────────────────────────────
    CryptoKeyExchangeFailed = 4001,
    CryptoSignatureInvalid  = 4002,
    CryptoDecryptionFailed  = 4003,

    // ── Anti-Forensics Errors (5xxx) ─────────────────────────────────────────
    AntiForensicsWipeFailed = 5001,

    // ── AI / Inference Errors (6xxx) ─────────────────────────────────────────
    AiInferenceFailed     = 6001,
    AiModelNotFound       = 6002,

    // ── Unknown / Generic ────────────────────────────────────────────────────
    Unknown               = 9999,
}

impl ErrorCode {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

/// Structured IPC error payload sent to the UI.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpcErrorResponse {
    pub code: i32,
    pub message: String,
    pub category: String,
    pub source: Option<String>,
    pub timestamp_ms: u64,
}

/// The unified error type for the ShieldDaemon.
#[derive(Debug, Error)]
pub enum ShieldError {
    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IPC error [{code:?}]: {message}")]
    Ipc { code: ErrorCode, message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Timeout")]
    Timeout,

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Authentication failed")]
    AuthFailed,

    #[error("All transports exhausted")]
    AllTransportsExhausted,

    #[error("NAIN detected — switching to covert channel")]
    NainDetected,

    #[error("DPI block detected — triggering failover")]
    DpiBlock,

    #[error("Quantum key exchange failed: {0}")]
    QuantumKex(String),

    #[error("AI inference error: {0}")]
    AiInference(String),

    #[error("Peer exchange failed: {0}")]
    PeerExchange(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl ShieldError {
    /// Create an IPC-category error.
    pub fn ipc(code: ErrorCode, message: impl Into<String>) -> Self {
        ShieldError::Ipc { code, message: message.into() }
    }

    /// Create a config-category error.
    pub fn config(message: impl Into<String>) -> Self {
        ShieldError::Config(message.into())
    }

    /// Create a transport-category error.
    pub fn transport(message: impl Into<String>) -> Self {
        ShieldError::Transport(message.into())
    }

    /// Create a crypto-category error.
    pub fn crypto(message: impl Into<String>) -> Self {
        ShieldError::Crypto(message.into())
    }
}

impl From<anyhow::Error> for ShieldError {
    fn from(e: anyhow::Error) -> Self {
        ShieldError::Unknown(e.to_string())
    }
}

impl From<serde_json::Error> for ShieldError {
    fn from(e: serde_json::Error) -> Self {
        ShieldError::Serialization(e.to_string())
    }
}
