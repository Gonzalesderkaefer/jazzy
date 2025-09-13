// Use modules
use super::distro::DistroId;


/// Enum for the window manager
pub enum WindowManagerId {
    Awesome,
    Bspwm,
    I3,
}

/// Metadata for the window manager
pub struct WindowManager {
    id: WindowManagerId,
    packages: [Option<&'static [&'static str]>; DistroId::Other as usize],
    setup_callback: fn(),
}
