// use modules
use super::distro;



/// Enum for the compositor
pub enum CompositorId {
    Sway,
    Niri,
    River,
    Hyprland,
}

/// Metadata for the compositor
pub struct Compositor {
    id: CompositorId,
    packages: [Option<&'static [&'static str]>; distro::DistroId::Other as usize],
}
