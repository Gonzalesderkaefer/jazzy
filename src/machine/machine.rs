use std::io;
use std::{
    fmt,
    error::Error,
};

use crate::config::config::{self, DspServerId};
use crate::machine::dsp_server;
use crate::machine::window_manager::WindowManager;
use crate::menu;
use super::distro;
use super::dsp_server::DspServer;
use super::window_manager as wm;
use super::transfer::Transfer;



/// This struct represents a Machine. An instance of this type is built when getting the
/// options from the user. This 'machine' is then 'applied' to the computer that it is running
/// on. This type is built while this program runs.
#[derive(Debug)]
pub struct Machine {
    /// The distro of this machine.
    pub distro: &'static distro::Distro,

    /// The display server that is intened to be used.
    pub display_server: &'static DspServer,

    /// The window manager that is intended to be used
    pub gui: Option<&'static wm::WindowManager>,

    /// Method of transfer for the config files and custom scripts
    pub transfer: Transfer,

    /// All packages that need to be installed for this install.
    pub all_packages: Vec<&'static str>,
}


/// This is an error type for a Distro
#[derive(Debug)]
pub enum MachineError {
    DistroErr (distro::DistroError),
    MenuError (io::Error),
    PackageMismatch,
}

/// Implementation of [fmt::Display] for [DistroError]
impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::DistroErr(error) => {
                return write!(f, "Distro not found: {}", error);
            }
            Self::MenuError(error) => {
                return write!(f, "Running the menu failed: {}", error);
            }
            Self::PackageMismatch => {
                return write!(f, "Packages that should be there arent. Check if config.rs is configured properly");
            }
        }
    }
}
impl Error for MachineError{}


impl Machine {
    pub fn get() -> Result<Self, MachineError> {
        // Get the distro from the system
        let distro = match distro::Distro::get() {
            Ok(_distro) => _distro,
            Err(error) => {
                return Err(MachineError::DistroErr(error));
            },
        };

        // Get choice of display server from user
        let display_server = match menu::menu::print_menu(" Choose a display server ", distro.supported_dsp_serv.to_vec()) {
            Ok(display_server_) => display_server_,
            Err(e) => {
                return Err(MachineError::MenuError(e));
            },
        };

        let window_manager = {
            // If length is 0 then window manager is NOWM
            if display_server.supported_wms.len() == 0 || distro.supported_wms.len() == 0 {
                &config::NOWM
            } else {
                // Get all possible window_managers
                let mut all_window_managers: Vec<&WindowManager> = Vec::new();
                for window_manager_in_distro in distro.supported_wms {
                    if display_server.supported_wms.contains(window_manager_in_distro) {
                        all_window_managers.push(window_manager_in_distro);
                    }
                }
                // Get  choice of window manager from user
                match menu::menu::print_menu(" Choose a window manager ", all_window_managers) {
                    Ok(wm) => wm,
                    Err(error) => {
                        return Err(MachineError::MenuError(error));
                    }
                }
            }
        };

        let transfer = match menu::menu::print_menu(" Choose a window manager ", [Transfer::Link, Transfer::Copy, Transfer::None].to_vec()) {
            Ok(wm) => wm,
            Err(error) => {
                return Err(MachineError::MenuError(error));
            }
        };





        return Err(MachineError::PackageMismatch);

    }
}
