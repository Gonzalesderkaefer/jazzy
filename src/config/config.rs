use crate::machine::distro;
use crate::machine::window_manager as wm;
use crate::machine::dsp_server as display;
use super::packages;


/// This file defines all constants that are useful for main.rs
/// If file paths are used make then relative to $HOME

/// The source of all files that will land in ~/.config or similar
pub static CFGSRC: &'static str = "Jazzian/dotfiles/config/";
pub static CFGDEST: &'static str = ".config/";


/// Custom scripts
pub static BINSRC: &'static str = "Jazzian/dotfiles/bin/";
pub static BINDEST: &'static str = ".local/bin/";



/// Enum for the distroid
#[derive(Debug)]
pub enum DistroId {
    Debian,
    Fedora,
    Arch,
    Other,
}


#[derive(Debug)]
pub enum DspServerId {
    Xorg,
    Wayland,
    Tty,
}

/// Enum for the compositor
/*
#[derive(Debug)]
pub enum CompositorId {
    Sway,
    Niri,
    River,
    Hyprland,
}
*/

/// Enum for the window manager
#[derive(Debug)]
pub enum WindowManagerId {
    Awesome,
    Bspwm,
    I3,
    Sway,
    Niri,
    River,
    Hyprland,
}




pub const DISTRO_ASSOC: &'static [(&distro::Distro /* Distro */, &'static str /* corresponding string as it would be found in '/etc/os-release' */)] = &[
];
