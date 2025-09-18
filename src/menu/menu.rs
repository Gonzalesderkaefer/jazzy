use crate::FgColor;


use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::{self, Color};
use ratatui::widgets::{List, ListState, StatefulWidget, ListItem};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};


/// NOTE: Might have to introduce another lifetime
pub fn print_menu<'a, Q: Into<ListItem<'a>>>(prompt: &str, choices: &'a [&'a Q]) -> io::Result<&'a Q> {
    todo!()
}





/// Represesnts a selection screen
pub struct SelectionScreen<'a> {
    /// If this is set to true the widget stops
    should_exit: bool,

    /// Choices that will be displayed
    choices: &'a [&'a ListItem<'a>],

    /// State of the internal List
    list_state: ListState,
}







/// Types that implement this trait will have a way to display the type
/// as a choice in a multiple choice menu
pub trait MenuEntry {
    /// Create a Menu entry for this type. This will be used to print a Menu screen.
    fn menu_entry(&self) -> String;
}





