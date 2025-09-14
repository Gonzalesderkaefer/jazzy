use super::distro;
use super::dsp_server;
use super::window_manager as wm;
use super::transfer;



/// This struct represents a Machine. An instance of this type is built when getting the
/// options from the user. This 'machine' is then 'applied' to the computer that it is running
/// on. This type is built while this program runs.
struct Machine {
    /// The distro of this machine.
    distro: distro::Distro,

    /// The display server that is intened to be used.
    display_server: dsp_server::DspServer,

    /// The window manager that is intended to be used
    gui: Option<wm::WindowManager>,

    /// Method of transfer for the config files and custom scripts
    transfer: transfer::Transfer,

    /// All packages that need to be installed for this install.
    all_packages: Vec<&'static str>,
}
