mod buffer;
mod terminal;
mod view;

use std::{cmp::min, io};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use terminal::{Position, Terminal};
use view::View;

#[derive(Debug, Default)]
pub struct Editor {
    shoud_quit: bool,
    caret_location: Location,
    view: View,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> io::Result<()> {
        loop {
            self.refresh_screen()?;
            if self.shoud_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> io::Result<()> {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.shoud_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => self.move_caret(*code)?,
                _ => (),
            }
        };
        Ok(())
    }

    fn move_caret(&mut self, code: KeyCode) -> io::Result<()> {
        match code {
            KeyCode::Up => self.caret_location.y = self.caret_location.y.saturating_sub(1),
            KeyCode::Down => {
                self.caret_location.y = min(
                    self.caret_location.y.saturating_add(1),
                    Terminal::size()?.height,
                );
            }
            KeyCode::Left => self.caret_location.x = self.caret_location.x.saturating_sub(1),
            KeyCode::Right => {
                self.caret_location.x = min(
                    self.caret_location.x.saturating_add(1),
                    Terminal::size()?.width,
                );
            }
            KeyCode::PageUp => self.caret_location.y = 0,
            KeyCode::PageDown => self.caret_location.y = Terminal::size()?.height.saturating_sub(1),
            KeyCode::Home => self.caret_location.x = 0,
            KeyCode::End => self.caret_location.x = Terminal::size()?.width.saturating_add(1),
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> io::Result<()> {
        Terminal::hide_caret()?;

        if self.shoud_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Terminal::move_caret_to(Position::default())?;
            self.view.render()?;
            Terminal::move_caret_to(self.caret_location.into())?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

impl From<Location> for Position {
    fn from(l: Location) -> Self {
        Position { col: l.x, row: l.y }
    }
}
