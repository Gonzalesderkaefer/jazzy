// Use modules
use super::distro::DistroId;



pub enum DspServerId {
    Wayland,
    Xorg,
    Tty,
}

pub struct DspServer {
    id: DspServerId,
    packages: [Option<&'static [&'static str]>; DistroId::Other as usize],
}
