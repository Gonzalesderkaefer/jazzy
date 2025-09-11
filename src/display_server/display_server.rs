
// Modules
use crate::distro::distro::DistroEnum;



/// This struct represents a display server. Instances of this are usually instanciated
/// config.rs
pub struct DisplayServer {
    /// These are the packages to install this window manager The indicies respond to `DistroEnum
    /// as u8` If one of the packages is [None] this means the window manager cannot be installed
    /// on that distro
    packages: [Option<&'static[&'static str]>; DistroEnum::Other as usize],

    /// This function is a setup function that is defined in config.rs and passed when initializing
    /// this function is called at the end of this program
    setup_fn: fn(&Self, &DistroEnum),
}
