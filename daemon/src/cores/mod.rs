pub mod amneziavpn;
pub mod core_manager;
pub mod defyx;
pub mod hiddify;
pub mod lantern;
pub mod mahsang;
pub mod moav;
pub mod psiphon;
pub mod singbox;
pub mod xray;

pub use core_manager::CoreManager;
pub use singbox::SingboxCoreAdapter;
pub use xray::XrayCore;
pub use hiddify::HiddifyCore;
pub use psiphon::PsiphonAdapter;
pub use lantern::LanternAdapter;
pub use amneziavpn::AmneziaVpnAdapter;
pub use defyx::DefyxVpnAdapter;
pub use mahsang::MahsangAdapter;
pub use moav::MoavAdapter;

pub type SingBoxCore    = SingboxCoreAdapter;
pub type PsiphonCore    = PsiphonAdapter;
pub type LanternCore    = LanternAdapter;
pub type AmneziaVpnCore = AmneziaVpnAdapter;
pub type DefyxCore      = DefyxVpnAdapter;
pub type MahsangCore    = MahsangAdapter;
pub type MoavCore       = MoavAdapter;

// Additional aliases for xray and hiddify to ensure name consistency
pub type XrayCore    = xray::XrayCoreAdapter;
pub type HiddifyCore = hiddify::HiddifyCoreAdapter;
