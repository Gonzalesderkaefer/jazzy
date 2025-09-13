// Use modules
use super::window_manager::WindowManager;
use super::compositor::Compositor;
use super::dsp_server::DspServer;

/// Enum for the distroid
pub enum DistroId {
    Debian,
    Fedora,
    Arch,
    Other,
}

/// Metadata for the distro
pub struct Distro {
    id: DistroId,
    supported_wms: &'static [&'static WindowManager],
    supported_comps: &'static [&'static Compositor],
    supported_dsp_serv: &'static [&'static DspServer],
    install: &'static [&'static str],
    update: &'static [&'static str],
    upgrade: &'static [&'static str],
    packages: &'static [&'static str],
}
