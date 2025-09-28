use std::io;
use std::{
    fmt,
    error::Error,
};

use crate::config::config::{self, DspServerId, NOWM};
use crate::machine::dsp_server;
use crate::machine::window_manager::WindowManager;
use crate::menu;
use crate::menu::menu::MenuErr;
use crate::utils::command;
use crate::JazzyErr;
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
    DistroErr (distro::DistroError, u32, &'static str),
    MenuError (MenuErr, u32, &'static str),
    CmdError (command::CommandError, u32, &'static str),
}

/// Implementation of [fmt::Display] for [DistroError]
impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::DistroErr(error, line, file) => {
                return write!(f, "Distro not found at {line} in {file}: {}", error);
            }
            Self::MenuError(error, line, file) => {
                return write!(f, "Running the menu failed at {line} in {file}: {}", error);
            }
            Self::CmdError(error, line, file) => {
                return write!(f, "Running a command failed at {line} in {file}: {}", error);
            }
        }
    }
}
impl Error for MachineError{}


impl Machine {
    /// Construct a [Machine] from user choices
    pub fn get() -> Result<Self, MachineError> {
        // Get the distro from the system
        // Check if there was an error
        let distro = match distro::Distro::get() {
            Ok(_distro) => _distro,
            Err(error) => {
                // Check type of underlying error
                match error {
                    // This can be returned as is.
                    distro::DistroError::FileReadError(..) => {
                        return Err(MachineError::DistroErr(error, line!(), file!()));
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
                                    return Err(MachineError::MenuError(error, line!(), file!()));
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

        // Get display server from user
        // This is an [Option]
        let display_serv = match menu::menu::print_menu(" Choose a display server ", distro.supported_dsp_serv.to_vec()) {
            Ok(server) => server,
            Err(error) => return Err(MachineError::MenuError(error, line!(), file!()))
        };

        // Check if list was empty
        let display_server = match display_serv {
            Some(server) => server,
            None => &config::TTY,
        };


        // Get supported window managers
        let mut supported_wms: Vec<&WindowManager> = Vec::new();
        for window_manager in distro.supported_wms {
            if display_server.supported_wms.contains(window_manager) {
                supported_wms.push(window_manager);
            }
        }

        // Get window manager from user
        let window_manager_opt = match menu::menu::print_menu(" Choose a window manager ", supported_wms) {
            Ok(wm) => wm,
            Err(error) => return Err(MachineError::MenuError(error, line!(), file!()))
        };

        // Check if list was empty
        let window_manager = match window_manager_opt {
            Some(wm) => wm,
            None => &NOWM,
        };



        // Get window manager from user
        let transfer = match menu::menu::print_menu(" Choose a method of transfer ", vec![Transfer::None, Transfer::Link, Transfer::Copy]) {
            // This cannot ever panic because the list is not empty
            Ok(transfer_option) => transfer_option.expect("The list of transfer methods was empty"),
            Err(error) => return Err(MachineError::MenuError(error, line!(), file!())),
        };


        // All packages that need to be installed
        let mut all_packages: Vec<&str> = Vec::new();

        // Add install command
        for cmd_arg in distro.install {
            all_packages.push(cmd_arg);
        }

        // Add base packages
        for base_package in distro.packages {
            all_packages.push(base_package);
        }

        // Check if there are packages for the distro
        match display_server.packages[distro.id.clone() as usize] {
            Some(dsp_server_pkgs) => {
                // append all packages
                for display_package in dsp_server_pkgs {
                    all_packages.push(display_package);
                }
            },
            None => {},
        }

        // Check if there are packages for the distro
        match window_manager.packages[distro.id.clone() as usize] {
            Some(wm_pkgs) => {
                // append all packages
                for wm_package in wm_pkgs {
                    all_packages.push(wm_package);
                }
            },
            None => {},
        }


        match distro.install_suffix {
            Some(suffixes) => for suffix in suffixes  {
                all_packages.push(suffix);
            },
            None => {}
        }



        return Ok(Self {
            distro: distro,
            display_server: display_server,
            gui: window_manager,
            transfer: transfer,
            all_packages: all_packages,
        });
    }

    /// Update all packages on a machine
    pub fn update(&self) -> Result<(), MachineError> {
        // Run the update command
        match command::cmd("sudo", self.distro.update) {
            Ok(_) => {},
            Err(error) => {
                return Err(MachineError::CmdError(error, line!(), file!()));
            },
        }

        // Run the upgrade command
        match command::cmd("sudo", self.distro.upgrade) {
            Ok(_) => {},
            Err(error) => {
                return Err(MachineError::CmdError(error, line!(), file!()));
            },
        }

        return Ok(());
    }

    pub fn install(&self) -> Result<(), MachineError> {
        // Install all packages
        match command::cmd("sudo", self.all_packages.as_slice()) {
            Ok(_) => {},
            Err(error) => {
                return Err(MachineError::CmdError(error, line!(), file!()));
            },
        }

        return Ok(());
    }

}
