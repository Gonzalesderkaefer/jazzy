#![allow(unused_imports)]
use std::io;
use color_eyre::owo_colors::Style;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::{List, ListState, StatefulWidget};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};





fn main() -> Result<(), io::Error>{
    let mut terminal = ratatui::init();
    let app_result = FrameCountWidget::default().run(&mut terminal);
    ratatui::restore();
    return app_result;
}




#[derive(Default)]
struct FrameCountWidget {
    exit: bool,
    counter: i32,
    list_state: ListState,
}


impl FrameCountWidget {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let _ = terminal.draw( |frame| self.draw(frame));
            self.handle_events()?;
        }
        Ok(())
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
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('k') => match self.list_state.selected_mut() {
                Some(index) => *self.list_state.selected_mut() = Some(*index - 1),
                None => *self.list_state.selected_mut() = Some(0),
            },
            KeyCode::Char('j') => match self.list_state.selected_mut() {
                Some(index) => *self.list_state.selected_mut() = Some(*index + 1),
                None => *self.list_state.selected_mut() = Some(0),
            },
            _ => {}
        }
    }




    pub fn draw(&mut self, frame: &mut Frame) {
        let mut i = self.counter;
        frame.render_stateful_widget(self, frame.area(), &mut i);
    }
}


impl StatefulWidget for &mut FrameCountWidget {
    type State = i32;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut i32) {
        //*state += 1;
        let text = format!("Frame count: {state}");
        let title = Line::from(text);

        let [mid_x, mid_y] = [area.width / 2 - area.width / 8, area.height / 2 - area.height / 8];

        let cool_area = Rect {
            x: mid_x,
            y: mid_y,
            width: area.width / 4,
            height: area.height / 4,
        };


        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED);

        let list = List::new(["First Item","Second Item"]).block(block).highlight_symbol(">");

        StatefulWidget::render(list, cool_area, buf, &mut self.list_state);
    }
}

