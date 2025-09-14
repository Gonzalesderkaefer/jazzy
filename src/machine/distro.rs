use super::window_manager::WindowManager;
use super::compositor::Compositor;
use super::dsp_server::DspServer;
use crate::config::config;


/// The distro this program is running on.
///
/// This struct stores metadata for a certain distro
pub struct Distro {
    id: config::DistroId,
    supported_wms: &'static [&'static WindowManager],
    supported_comps: &'static [&'static Compositor],
    supported_dsp_serv: &'static [&'static DspServer],
    install: &'static [&'static str],
    update: &'static [&'static str],
    upgrade: &'static [&'static str],
    packages: &'static [&'static str],
}
