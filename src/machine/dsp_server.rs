use crate::config::config;
use super::window_manager as wm;
use crate::menu::menu;


use std::fmt;
use ratatui::widgets::{ ListItem };


/// Implement display for DisplayServerId
impl fmt::Display for config::DspServerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

/// Display server on which the window managers rely
#[derive(Debug)]
pub struct DspServer {
    /// Id is stored as an enum
    pub id: config::DspServerId,

    /// List of window managers, that work with this display server
    pub supported_wms: &'static [&'static wm::WindowManager],

    /// Packages required to install on the specific distro
    pub packages: [Option<&'static [&'static str]>; config::DistroId::Other as usize],

    /// Setup callback
    pub setup_callback: fn(),

}

/// This is for ratatui
impl<'a> From<&DspServer> for ListItem<'a> {
    fn from(value: &DspServer) -> Self {
        return Self::new(value.id.to_string());
    }
}
