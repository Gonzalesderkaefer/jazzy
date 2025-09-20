use crate::menu::menu;

use std::fmt;
use ratatui::widgets::{ ListItem };


/// Represents method of transfer
#[derive(Debug, Clone)]
pub enum Transfer {
    Link,
    Copy,
    None,
}

/// Implement Display for Transfer to get to_string()
impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

/// This is for ratatui
impl<'a> From<&Transfer> for ListItem<'a> {
    fn from(value: &Transfer) -> Self {
        return Self::new(value.to_string());
    }
}

impl menu::MenuEntry for Transfer {
    fn menu_entry(&self) -> String {
        // Store id as string
        let mut self_as_string = self.to_string();

        // Insert square brackets
        self_as_string.insert(0, '[');
        self_as_string.insert(2, ']');

        return self_as_string;
    }
}
