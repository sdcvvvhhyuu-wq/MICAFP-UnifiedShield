pub mod amneziawg;
pub mod boringtun_adapter;
pub mod split_tunnel;
pub mod tun_device;
pub mod wireguard;

pub use tun_device::TunDevice;
pub use wireguard::WireGuardTunnel;
pub use amneziawg::AmneziaWGTunnel;
pub use boringtun_adapter::BoringTunAdapter;
pub use split_tunnel::SplitTunnel;

pub type AmneziaWgTunnel  = AmneziaWGTunnel;
pub type BoringtunAdapter = BoringTunAdapter;
