// ─────────────────────────────────────────────────────────────────────────────
// Battery / power management subsystem
// MICAFP-UnifiedShield-vip-ultra-Quantum-ultra v8.0
// ─────────────────────────────────────────────────────────────────────────────

pub mod adaptive_duty;
pub mod coalesced_timer;
pub mod optimizer;
pub mod power_state;

pub use adaptive_duty::{AdaptiveDutyCycler, PowerMode, TaskId, TaskDutyTable};
pub use coalesced_timer::CoalescedTimer;
pub use power_state::PowerState;
