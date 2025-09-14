use std::fmt;
use crate::menu::menu;



#[derive(Debug)]
pub enum Transfer {
    Link,
    Copy,
    None,
}

impl fmt::Display for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
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
