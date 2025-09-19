use crate::menu::menu;
use crate::config::config;


use std::fmt;
use ratatui::widgets::{ ListItem };




/// Window manager that runs on xorg
///
/// `WindowManager` is used to define a window manager. These are defined in 'src/config/config.rs'
pub struct WindowManager {
    /// Name of this window manager. It's an enum so that is defined in 'src/config/config.rs'
    pub id: config::WindowManagerId,

    /// Packages needed for this Compositor, the index is the DistroId if 
    /// an entry is [None] It means the Compositor is not supported for that [Distro]
    pub packages: [Option<&'static [&'static str]>; config::DistroId::Other as usize],

    /// Function that is called after installing this window manager
    pub setup_callback: fn(),
}





/// Implementation of Display for [CompositorId] to get `to_string()` for free
impl fmt::Display for config::WindowManagerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

/// This is for ratatui
impl<'a> From<config::WindowManagerId> for ListItem<'a> {
    fn from(value: config::WindowManagerId) -> Self {
        return Self::new(value.to_string());
    }
}
