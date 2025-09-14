use super::distro;
use super::dsp_server;
use super::window_manager as wm;
use super::compositor;
use super::transfer;



/// This enum represents a variant of some Graphical User interface. It can be one of the following
/// * Wayland
/// * Xorg
/// * Nothing (A desktop is probably pre installed)
pub enum GuiVariant {
    Wayland (compositor::Compositor),
    Xorg (wm::WindowManager),
    Nothing,
}


/// This struct represents a Machine. An instance of this type is built when getting the
/// options from the user. This 'machine' is then 'applied' to the computer that it is running
/// on.
struct Machine {
    distro: distro::Distro,
    display_server: dsp_server::DspServer,
    gui: GuiVariant,
    transfer: transfer::Transfer,
    all_packages: Vec<&'static str>,
}
