use crate::menu::menu;
use crate::config::config;
use std::fmt;

/// Metadata for the compositor
pub struct Compositor {
    id: config::CompositorId,
    packages: [Option<&'static [&'static str]>; config::DistroId::Other as usize],
}

impl fmt::Display for config::CompositorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

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
