use super::distro::DistroId;
use crate::menu::menu;
use crate::config::config;
use std::fmt;




/// Metadata for the window manager
pub struct WindowManager {
    id: config::WindowManagerId,
    packages: [Option<&'static [&'static str]>; DistroId::Other as usize],
    setup_callback: fn(),
}


impl fmt::Display for config::WindowManagerId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl menu::MenuEntry for WindowManager {
    fn menu_entry(&self) -> String {
        // Store id as string
        let mut self_as_string = self.id.to_string();

        // Insert square brackets
        self_as_string.insert(0, '[');
        self_as_string.insert(2, ']');

        return self_as_string;
    }
}
