/// This file defines all constants that are useful for main.rs
/// If file paths are used make then relative to $HOME

/// The source of all files that will land in ~/.config or similar
pub static CFGSRC: &'static str = "Jazzian/dotfiles/config/";
pub static CFGDEST: &'static str = ".config/";


/// Custom scripts
pub static BINSRC: &'static str = "Jazzian/dotfiles/bin/";
pub static BINDEST: &'static str = ".local/bin/";



/// Enum for the distroid
pub enum DistroId {
    Debian,
    Fedora,
    Arch,
    Other,
}

/// Metadata for the distro
pub struct Distro {
    id: DistroId,
    supported_wms: &'static [&'static WindowManager],
    supported_comps: &'static [&'static Compositor],
    install: &'static [&'static str],
    update: &'static [&'static str],
    upgrade: &'static [&'static str],
    packages: &'static [&'static str],
}


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
    packages: [Option<&'static [&'static str]>; DistroId::Other as usize],
}

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


