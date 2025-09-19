use crate::machine::distro;
use crate::machine::window_manager as wm;
use crate::machine::dsp_server as display;
use super::packages as pkg;


/// This file defines all constants that are useful for main.rs
/// If file paths are used make then relative to $HOME

/// The source of all files that will land in ~/.config or similar
pub static CFGSRC: &'static str = "Jazzian/dotfiles/config/";
pub static CFGDEST: &'static str = ".config/";


/// Custom scripts
pub static BINSRC: &'static str = "Jazzian/dotfiles/bin/";
pub static BINDEST: &'static str = ".local/bin/";



/// Enum for the distroid
#[derive(Debug, Clone)]
pub enum DistroId {
    Debian,
    Fedora,
    Arch,
    Other,
}


#[derive(Debug, Clone)]
pub enum DspServerId {
    Xorg,
    Wayland,
    Tty,
}

/// Enum for the window manager
#[derive(Debug, Clone)]
pub enum WindowManagerId {
    Awesome,
    Bspwm,
    I3,
    Sway,
    Niri,
    River,
    Hyprland,
}

/// Window managers

/// XORG
pub const AWESOME_WM: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::Awesome,  // Id
//             Debian              Fedora              Arch Linux
    packages: [Some(pkg::DEB_AWE), Some(pkg::FED_AWE), Some(pkg::ARCH_AWE)], // Packages
    setup_callback: || {} // callback
};


pub const BSPWM: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::Bspwm,  // Id
//             Debian              Fedora              Arch Linux
    packages: [Some(pkg::DEB_BSP), Some(pkg::FED_BSP), Some(pkg::ARCH_BSP)], // Packages
    setup_callback: || {} // callback
};


pub const I3: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::I3,  // Id
//             Debian             Fedora             Arch Linux
    packages: [Some(pkg::DEB_I3), Some(pkg::FED_I3), Some(pkg::ARCH_I3)], // Packages
    setup_callback: || {} // callback
};


/// WAYLAND
pub const SWAY: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::Sway,  // Id
//             Debian               Fedora               Arch Linux
    packages: [Some(pkg::DEB_SWAY), Some(pkg::FED_SWAY), Some(pkg::ARCH_SWAY)], // Packages
    setup_callback: || {} // callback
};


pub const NIRI: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::Niri,  // Id
//             Debian Fedora               Arch Linux
    packages: [None,  Some(pkg::FED_NIR), Some(pkg::ARCH_NIR)], // Packages
    setup_callback: || {} // callback
};


pub const RIVER: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::River,  // Id
//             Debian Fedora              Arch Linux
    packages: [None,  Some(pkg::FED_RIV), Some(pkg::ARCH_RIV)], // Packages
    setup_callback: || {} // callback
};


pub const HYPRLAND: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::Hyprland,  // Id
//             Debian Fedora               Arch Linux
    packages: [None,  Some(pkg::FED_HYPR), Some(pkg::ARCH_HYPR)], // Packages
    setup_callback: || {} // callback
};


/// Display servers
pub const XORG: display::DspServer = display::DspServer {
    id: DspServerId::Xorg,
    supported_wms: &[&AWESOME_WM, &BSPWM, &I3],
    setup_callback: || {},
    packages: [Some(pkg::DEB_XORG), Some(pkg::FED_XORG), Some(pkg::ARCH_XORG)],
};

pub const WAYLAND: display::DspServer = display::DspServer {
    id: DspServerId::Wayland,
    supported_wms: &[&SWAY, &NIRI, &RIVER, &HYPRLAND],
    setup_callback: || {},
    packages: [Some(pkg::DEB_XORG), Some(pkg::FED_XORG), Some(pkg::ARCH_XORG)],
};

pub const TTY: display::DspServer = display::DspServer {
    id: DspServerId::Tty,
    supported_wms: &[],
    setup_callback: || {},
    packages: [None, None, None],
};


/// Distros
pub const DEBIAN: distro::Distro = distro::Distro {
    id: DistroId::Debian,
    supported_dsp_serv: [Some(&XORG),Some(&WAYLAND)],
    supported_wms: [Some(&[&AWESOME_WM, &BSPWM, &I3]), Some(&[&SWAY])],
    install: &["apt", "install", "-y"],
    install_suffix: None,
    update: &["apt", "update", "-y"],
    upgrade: &["apt", "upgrade", "-y"],
    packages: pkg::DEB_BASE,
};

pub const FEDORA: distro::Distro = distro::Distro {
    id: DistroId::Fedora,
    supported_dsp_serv: [Some(&XORG),Some(&WAYLAND)],
    supported_wms: [Some(&[&AWESOME_WM, &BSPWM, &I3]), Some(&[&SWAY, &NIRI, &RIVER, &HYPRLAND])],
    install: &["dnf", "install", "-y"],
    install_suffix: None,
    update: &["dnf", "upgrade", "--refresh"],
    upgrade: &["dnf", "update", "-y"],
    packages: pkg::FED_BASE,
};

pub const ARCH_LINUX: distro::Distro = distro::Distro {
    id: DistroId::Arch,
    supported_dsp_serv: [Some(&XORG),Some(&WAYLAND)],
    supported_wms: [Some(&[&AWESOME_WM, &BSPWM, &I3]), Some(&[&SWAY, &NIRI, &RIVER, &HYPRLAND])],
    install: &["pacman", "-Syu"],
    install_suffix: Some(&["--noconfirm", "--needed"]),
    update: &["pacman", "-Sy", "--noconfirm", "--needed"],
    upgrade: &["pacman", "-Syu", "--noconfirm", "--needed"],
    packages: pkg::ARCH_BASE,
};


pub const DISTRO_ASSOC: &'static [(&distro::Distro /* Distro */, &'static str /* string in '/etc/os-release' */)] = &[
    (&DEBIAN, "Debian"),
    (&FEDORA, "Fedora"),
    (&ARCH_LINUX, "Arch Linux"),
];
