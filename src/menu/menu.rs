use crate::config::config;
use crate::FgColor;


use std::fmt::Display;
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

// This enum represents an Error for this module
#[derive(Debug)]
pub enum MenuErr {
    IO (io::Error, u32, &'static str),
}

impl Display for MenuErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self {
            MenuErr::IO(error, line, file) => write!(f, "Internal IO Error at {line} in {file}: {}", error)
        };
    }
}
impl std::error::Error for MenuErr {}



/// Print a selection screen to the terminal that contains `choices` and returns
/// one of the `choices`.
///
/// If choices is empty and no error occured [None], wrapped in a [Result] is returned.
/// If choices is not empty and no error occured [Some], wrapped in a [Result] is returned.
/// Else a [MenuErr] wrapped in a [Result] is returned.
pub fn print_menu<'a, Q: Into<ListItem<'a>> + Clone>(prompt: &'a str, choices: Vec<Q>) -> Result<Option<Q>, MenuErr> {
    // Check if there are no choices
    if choices.len() == 0 {
        return Ok(None);
    }

    // Clear the screen. This is needed for the tty stuff
    print!("{}[2J", 27 as char);

    // Initialize a Terminal to print stuff to
    let mut terminal = ratatui::init();

    // Create a Selection screen ..
    let mut new_select = SelectionScreen::new(choices, prompt);
    // .. and 'start' it.
    let menu_result =  new_select.run(&mut terminal);

    // This is so that the terminal doesn't look weird 
    ratatui::restore();

    // Clear the screen. This is needed for the tty stuff
    print!("{}[2J", 27 as char);

    // Check if running the screen was successful
    match menu_result {
        Ok(_) => {
            return Ok(Some(new_select.choices[new_select.final_choice_index].clone()));
        },
        Err(error) => {
            return Err(MenuErr::IO(error, line!(), file!()));
        }
    }
}





/// Represesnts a selection screen
pub struct SelectionScreen<'a, Q: Into<ListItem<'a>> + Clone> {
    /// If this is set to true the widget stops
    should_exit: bool,

    /// Choices that will be displayed
    choices: Vec<Q>,

    /// The end choice of the user
    final_choice_index: usize,

    /// State of the internal List
    list_state: ListState,


    /// Title of this screen
    title: &'a str,
}

impl<'a, Q: Into<ListItem<'a>> + Clone> SelectionScreen<'a, Q> {
    /// Creates a new SelectionScreen
    fn new(choices: Vec<Q>, title: &'a str) -> Self {
        return Self {
            should_exit: false,
            list_state: ListState::default().with_selected(Some(0)),
            final_choice_index: 0,
            choices: choices,
            title: title,
        };
    }

    /// Start the displayed widget
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>  {
        while !self.should_exit {
            terminal.draw( |frame| self.draw(frame))?;
            self.handle_events()?;
        }
        return Ok(());
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
            KeyCode::Enter => { 
                // Get current selected entry
                match self.list_state.selected() {
                    Some(size) => {
                        // Check if index is within boundes
                        if size < self.choices.len() {
                            self.final_choice_index = size
                        } else {
                            self.final_choice_index = 0
                        }
                    }
                    None => todo!(),
                }
                self.should_exit = true;
            },
            KeyCode::Char('q') => { 
                self.should_exit = true;
            },
            KeyCode::Char('k') => match self.list_state.selected_mut() {
                Some(index) => *self.list_state.selected_mut() = {
                    if *index == 0 {
                        Some(self.choices.len() - 1)
                    } else {
                        Some(*index - 1)
                    }
                },
                None => *self.list_state.selected_mut() = Some(0),
            },
            KeyCode::Char('j') => match self.list_state.selected_mut() {
                Some(index) => *self.list_state.selected_mut() = {
                    if *index == self.choices.len() - 1 {
                        Some(0)
                    } else {
                        Some(*index + 1)
                    }
                },
                None => *self.list_state.selected_mut() = Some(0),
            },
            _ => {}
        }
    }



    /// Draw this widget
    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}



impl<'a, Q: Into<ListItem<'a>> + Clone> Widget for &mut SelectionScreen<'a, Q> {
    fn render(self, area: Rect, buf: &mut Buffer) {

        // Create a title for the window
        let title = Line::from(self.title).style(
            style::Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
        );

        // dimensions for the centered block
        let centered_width = area.width / 4;
        let centered_height = area.height / 4;

        // Top left corner
        let centered_x = area.width / 2 - centered_width / 2;
        let centered_y = area.height / 2 - centered_height / 2;




        // Create a rect that is centered
        let cool_area = Rect {
            x: centered_x,
            y: centered_y,
            width: area.width / 4,
            height: area.height / 4,
        };

        // Create block inside rect
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        let list = List::new(self.choices.clone()).block(block).highlight_symbol(">");

        // Finally render the widget
        StatefulWidget::render(list, cool_area, buf, &mut self.list_state);
    }
}
