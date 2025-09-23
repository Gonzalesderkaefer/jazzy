use std::io;
use std::{
    fmt,
    error::Error,
};

use crate::config::config::{self, DspServerId};
use crate::machine::dsp_server;
use crate::machine::window_manager::WindowManager;
use crate::menu;
use crate::menu::menu::MenuErr;
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
    pub gui: &'static wm::WindowManager,

    /// Method of transfer for the config files and custom scripts
    pub transfer: Transfer,

    /// All packages that need to be installed for this install.
    pub all_packages: Vec<&'static str>,
}


/// This is an error type for a Distro
#[derive(Debug)]
pub enum MachineError {
    DistroErr (distro::DistroError),
    MenuError (MenuErr),
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
        // Check if there was an error
        let distro = match distro::Distro::get() {
            Ok(_distro) => _distro,
            Err(error) => {
                // Check type of underlying error
                match error {
                    // This can be returned as is.
                    distro::DistroError::FileReadError(_) => {
                        return Err(MachineError::DistroErr(error));
                    }
                    // This needs to be handeled sepereately
                    distro::DistroError::NotSupported => {
                        // Only ask for Transfer type
                        let transfer_method = {
                            let final_result: Transfer;
                            // Check for error
                            match menu::menu::print_menu(" Choose a method of transfer ", vec![Transfer::None, Transfer::Link, Transfer::Copy]) {
                                Ok(transfer_option) => {
                                    // This cannot ever panic because the list is not empty
                                    final_result = transfer_option.expect("The list of transfer methods was empty")
                                }
                                Err(error) => {
                                    return Err(MachineError::MenuError(error));
                                },
                            }
                            final_result
                        };

                        // Return 'empty' machine
                        return Ok(Self {
                            distro: &config::OTHER_DISTRO,
                            display_server: &config::TTY,
                            gui: &config::NOWM,
                            transfer: transfer_method,
                            all_packages: Vec::new(),
                        });
                    }
                }
            },
        };








    }
}
