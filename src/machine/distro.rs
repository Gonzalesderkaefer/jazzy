use super::window_manager::WindowManager;
use super::dsp_server::DspServer;
use crate::config::config;
use crate::utils::fileutils as fu;

use std::{
    fmt,
    fs,
    io,
    error::Error,
};

/// The distro this program is running on.
///
/// This struct stores metadata for a certain distro
pub struct Distro {
    /// Name of the distro. Is an enum.
    pub id: config::DistroId,

    /// List of display servers supported by this distro. If a display server is not supported,
    /// its entry in this list will be [None].
    pub supported_dsp_serv: [Option<&'static DspServer>; config::DspServerId::Tty as usize],

    /// List of List of supported window managers. There are as many lists as there are
    /// display servers. If there's a display server that has no window manager (unlikely)
    /// its entry in this list is [None].
    pub supported_wms: [Option<&'static [&'static WindowManager]>; config::DspServerId::Tty as usize],

    /// Install command to install packages. These are the words with which the install command
    /// starts. It is generally assumed that install commands have the following structure:
    /// sudo [install_cmd] [args...] [packages...] (suffix..)
    pub install: &'static [&'static str],

    /// Suffix for the install command this will be appened at the end of the big install command.
    pub install_suffix: Option<&'static [&'static str]>,

    /// Update command to update all the the repos. would be 'sudo apt update' on debian based
    /// systems.
    pub update: &'static [&'static str],

    /// Upgrade command to properly update all packages that are installed on this system.
    pub upgrade: &'static [&'static str],

    /// Required packages to use for a minimal install so that the tty works.
    pub packages: &'static [&'static str],
}

/// This is an error type for a Distro
#[derive(Debug)]
pub enum DistroError {
    FileReadError (io::Error),
    NotSupported,
}

/// Implementation of [fmt::Display] for [DistroError]
impl fmt::Display for DistroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DistroError::FileReadError(file_util_err) => {
                return write!(f, "File read error: {}", file_util_err);
            },
            DistroError::NotSupported => {
                return write!(f, "No supported distro found");
            },
        }
    }
}
impl Error for DistroError{}




impl Distro {
    pub fn get() -> Result<&'static Distro, DistroError> {
        // Read the release file and check for an error
        let release_file = match fs::read_to_string("/etc/os-release") {
            Ok(file_contents) => file_contents,
            Err(error) => {
                return Err(DistroError::FileReadError(error));
            },
        };


        // Search for the distros
        for distro in config::DISTRO_ASSOC {
            if release_file.find(distro.1).is_some() {
                return Ok(distro.0);
            }
        };

        return Err(DistroError::NotSupported);
    }
}



