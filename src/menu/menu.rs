use crate::FgColor;


use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal;
use ratatui::style::{self, Color, Modifier};
use ratatui::widgets::{List, ListState, StatefulWidget, ListItem};
use ratatui::Terminal;
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
pub fn print_menu<'a, Q: Into<ListItem<'a>>>(prompt: &'a str, choices: &'a [Q]) /* -> io::Result<&'a Q> */ {
    let mut terminal = ratatui::init();
    let mut new_select = SelectionScreen::new(choices, prompt);
    new_select.run(&mut terminal);
    ratatui::restore();
}





/// Represesnts a selection screen
/// NOTE: Might have to introduce other lifetimes
pub struct SelectionScreen<'a, Q: Into<ListItem<'a>>> {
    /// If this is set to true the widget stops
    should_exit: bool,

    /// Choices that will be displayed
    choices: &'a [Q],

    /// State of the internal List
    list_state: ListState,

    /// Title of this screen
    title: &'a str,
}

impl<'a, Q: Into<ListItem<'a>>> SelectionScreen<'a, Q> {
    /// Creates a new SelectionScreen
    fn new(choices: &'a [Q], title: &'a str) -> Self {
        return Self {
            should_exit: false,
            choices: choices,
            list_state: ListState::default().with_selected(Some(0)),
            title: title,
        };
    }

    /// Start the displayed widget
    fn run(&mut self, terminal: &mut DefaultTerminal) /* -> io::Result<()> */ {
        while !self.should_exit {
            let _ = terminal.draw( |frame| self.draw(frame));
            self.handle_events();
        }
    }


    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.should_exit = true,
            _ => {}
        }
    }



    /// Draw this widget
    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}



impl<'a, Q: Into<ListItem<'a>>> Widget for &mut SelectionScreen<'a, Q> {
    fn render(self, area: Rect, buf: &mut Buffer) {

        // Create a title for the window
        let title = Line::from(self.title).style(
            style::Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
        );

        // Create a rect that is centered
        let [mid_x, mid_y] = [area.width / 2 - area.width / 8, area.height / 2 - area.height / 8];
        let cool_area = Rect {
            x: mid_x,
            y: mid_y,
            width: area.width / 4,
            height: area.height / 4,
        };

        // Create block inside rect
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);



        // Finally render the widget
        Widget::render(block, cool_area, buf);
    }
}




/// Types that implement this trait will have a way to display the type
/// as a choice in a multiple choice menu
pub trait MenuEntry {
    /// Create a Menu entry for this type. This will be used to print a Menu screen.
    fn menu_entry(&self) -> String;
}





