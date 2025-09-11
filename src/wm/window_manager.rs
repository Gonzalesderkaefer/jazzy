// Modules
use crate::distro::distro::DistroEnum;



/// This struct represents a window manager. Instances of this are usually instanciated
/// config.rs
pub struct WindowManager {
    /// These are the packages to install this window manager The indicies respond to `DistroEnum
    /// as u8` If one of the packages is [None] this means the window manager cannot be installed
    /// on that distro
    packages: [Option<&'static[&'static str]>; DistroEnum::Other as usize],

    /// This function is a setup function that is defined in config.rs and passed when initializing
    /// this function is called at the end of this program
    setup_fn: fn(&Self, &DistroEnum),
}

impl WindowManager {
    /// Get packages for a distro.
    ///
    /// This function returns a reference to an array of packages that is wrapped in an [Option].
    /// If [None] is returned the window manager is probably not supported on the [distro]
    pub fn packages(&self, distro: DistroEnum) -> Option<&'static[&'static str]> {
        // Turn distro to an index
        let index = distro as usize;

        // Check if index is in bounds
        if index < self.packages.len() {
            return self.packages[index];
        }
        return None;
    }

    /// Wrapper function for the setup
    ///
    /// This function is merely a wrapper function for the setup
    pub fn setup(&self, distro: &DistroEnum) {
        (self.setup_fn)(self, distro);

    }
}

