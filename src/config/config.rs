use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::machine::distro;
use crate::machine::window_manager as wm;
use crate::machine::dsp_server as display;
use super::packages as pkg;
use super::custom as cstm;
use crate::utils::fileutils as fu;
use crate::utils::command as cmd;


/// This file defines all constants that are useful for main.rs
/// If file paths are used make then relative to $HOME

/// The source of all files that will land in ~/.config or similar
pub static CFGSRC: &'static str = "Jazzian/dotfiles/config/";
pub static CFGDEST: &'static str = ".config/";


/// Custom scripts
pub static BINSRC: &'static str = "Jazzian/dotfiles/bin/";
pub static BINDEST: &'static str = ".local/bin/";


/// Shell configurations
pub static SHELLCFG: &'static str = "Jazzian/dotfiles/config/shell/";




/// Enum for the distroid
#[derive(Debug, Clone, PartialEq)]
pub enum DistroId {
    Debian,
    Fedora,
    ArchLinux,
    Other,
}


#[derive(Debug, Clone, PartialEq)]
pub enum DspServerId {
    Xorg,
    Wayland,
    Tty,
}

/// Enum for the window manager
#[derive(Debug, Clone, PartialEq)]
pub enum WindowManagerId {
    Awesome,
    Bspwm,
    I3,
    Sway,
    Niri,
    River,
    Hyprland,
    NoWM,
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

pub const NOWM: wm::WindowManager = wm::WindowManager {
    id: WindowManagerId::NoWM,  // Id
//             Debian     Fedora     Arch Linux
    packages: [Some(&[]), Some(&[]), Some(&[])], // Packages
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
    packages: [Some(pkg::DEB_WAY), Some(pkg::FED_WAY), Some(pkg::ARCH_WAY)],
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
    supported_dsp_serv: &[&XORG, &WAYLAND, &TTY],
    supported_wms: &[&AWESOME_WM, &BSPWM, &I3, &SWAY],
    install: &["apt", "install", "-y", "--no-install-recommends"],
    install_suffix: None,
    update: &["apt", "update", "-y"],
    upgrade: &["apt", "upgrade", "-y"],
    packages: pkg::DEB_BASE,
    setup_callback: || {
        // for Rofi
        create_and_write_user(".local/bin/mdrun", cstm::DEBMDRUN, 0o755);
        create_and_write_user(".local/bin/mdmenu", cstm::DEBMDMENU, 0o755);

        // create the font directory
        if let Some(mut home) = std::env::home_dir() {
            // Create the full dir to font dir
            home.push(".local/share/fonts");
            if let Ok(_) = fs::create_dir_all(&home) {
                // Download the nerdfont
                cmd("curl", &["-OL", "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz"]);

                // extract the archives
                if let Some(home_str) = home.to_str() {
                    cmd("tar", &["xJvf", "JetBrainsMono.tar.xz", "-C", home_str]);
                }
            }
        }
    },
};

pub const FEDORA: distro::Distro = distro::Distro {
    id: DistroId::Fedora,
    supported_dsp_serv: &[&XORG,&WAYLAND, &TTY],
    supported_wms: &[&AWESOME_WM, &BSPWM, &I3, &SWAY, &NIRI, &RIVER, &HYPRLAND],
    install: &["dnf", "install", "-y"],
    install_suffix: None,
    update: &["dnf", "upgrade", "--refresh"],
    upgrade: &["dnf", "update", "-y"],
    packages: pkg::FED_BASE,
    setup_callback: || {
        // for Rofi
        create_and_write_user(".local/bin/mdrun", cstm::MDRUN_CONTENT, 0o755);
        create_and_write_user(".local/bin/mdmenu", cstm::MDMENU_CONTENT, 0o755);


        // Create the font directory
        if let Some(mut home) = std::env::home_dir() {
            // Create the full dir to font dir
            home.push(".local/share/fonts");
            if let Ok(_) = fs::create_dir_all(&home) {
                // Download the nerdfont
                cmd("curl", &["-OL", "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/JetBrainsMono.tar.xz"]);

                // extract the archives
                if let Some(home_str) = home.to_str() {
                    cmd("tar", &["xJvf", "JetBrainsMono.tar.xz", "-C", home_str]);
                }
            }
        }
    },
};

pub const ARCH_LINUX: distro::Distro = distro::Distro {
    id: DistroId::ArchLinux,
    supported_dsp_serv: &[&XORG, &WAYLAND, &TTY],
    supported_wms: &[&AWESOME_WM, &BSPWM, &I3, &SWAY, &NIRI, &RIVER, &HYPRLAND],
    install: &["pacman", "-Syu"],
    install_suffix: Some(&["--noconfirm", "--needed"]),
    update: &["pacman", "-Sy", "--noconfirm", "--needed"],
    upgrade: &["pacman", "-Syu", "--noconfirm", "--needed"],
    packages: pkg::ARCH_BASE,
    setup_callback: || {
        // for Rofi
        create_and_write_user(".local/bin/mdrun", cstm::MDRUN_CONTENT, 0o755);
        create_and_write_user(".local/bin/mdmenu", cstm::MDMENU_CONTENT, 0o755);

    },
};


pub const OTHER_DISTRO: distro::Distro = distro::Distro {
    id: DistroId::Other,
    supported_dsp_serv: &[],
    supported_wms: &[],
    install: &[],
    install_suffix: None,
    update: &[],
    upgrade: &[],
    packages: &[],
    setup_callback: || {},
};


pub const DISTRO_ASSOC: &'static [(&distro::Distro /* Distro */, &'static str /* string in '/etc/os-release' */)] = &[
    (&DEBIAN, "Debian"),
    (&FEDORA, "Fedora"),
    (&ARCH_LINUX, "Arch Linux"),
];




// Helper functions.
// These will just print an error to stdout and return nothing
fn cmd<S: AsRef<OsStr> + Clone>(command: S, args: &[S]) {
    match cmd::cmd(command.clone(), args) {
        Ok(_) => {},
        Err(error) => println!("Failed to run {} in {} : {error}", command.as_ref().display(), file!())
    }
}

fn create_and_write_user<P: AsRef<Path> + Clone, C: AsRef<[u8]>>(new_file: P, contents: C, mode: u32) {
    match fu::create_and_write_user(new_file.clone(), contents, mode) {
        Ok(_) => {}
        Err(error) => println!("Failed to create file {}: {error}", new_file.as_ref().display()),
    }

}

