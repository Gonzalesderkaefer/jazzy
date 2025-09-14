use std::io;
use crate::FgColor;




pub fn print_menu<'a, Q: MenuEntry>(prompt: &str, choices: &'a [Option<&'a Q>]) -> io::Result<&'a Q> {
    todo!()
}




/// Types that implement this trait will have a way to display the type
/// as a choice in a multiple choice menu
pub trait MenuEntry {
    /// Create a Menu entry for this type. This will be used to print a Menu screen.
    fn menu_entry(&self) -> String;
}





