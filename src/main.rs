
#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
#[allow(unused_imports)]
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};





pub struct SelectionScreen {
    selected: u8, // Index that is selected
    choices: Vec<String>,
}



impl Widget for &SelectionScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let title = Line::from(" Window ".bold());

        let instructions = Line::from(vec![
            " Navigate with  ".into(),
            "<H, J, K, L>".blue().bold(),
            " or ".into(),
            "<Up, Right, Left, Down>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);


        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

    }
}


fn main() -> io::Result<()> {
    return Ok(());
}
