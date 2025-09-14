use crate::menu::menu;
use crate::config::config;
use std::fmt;

/// Compositor that runs on wayland
///
/// `Compositor` is used to define a Compositor. These are defined in 'src/config/config.rs'
pub struct Compositor {
    /// Name of this compositor. It's an enum so that is defined in 'src/config/config.rs'
    id: config::CompositorId,

    /// Packages needed for this Compositor, the index is the DistroId if 
    /// an entry is [None] It means the Compositor is not supported for that [Distro]
    packages: [Option<&'static [&'static str]>; config::DistroId::Other as usize],

    /// Function that is called after installing this compositor
    setup_callback: fn(),
}


/// Implementation of Display for [CompositorId] to get `to_string()` for free
impl fmt::Display for config::CompositorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}


/// Implementation of [MenuEntry] for creating menu-pages
impl menu::MenuEntry for Compositor {
    fn menu_entry(&self) -> String {
        // Store id as string
        let mut self_as_string = self.id.to_string();

        // Insert square brackets
        self_as_string.insert(0, '[');
        self_as_string.insert(2, ']');

        return self_as_string;
    }
}
