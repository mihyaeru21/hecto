mod buffer;
mod terminal;
mod view;

use std::{cmp::min, env, io};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use terminal::{Position, Size, Terminal};
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
        self.handle_args();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn handle_args(&mut self) {
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
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
        match event {
            Event::Key(e) => self.evaluate_key_event(e)?,
            Event::Resize(width, height) => {
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let height = *height as usize;
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let width = *width as usize;
                self.view.resize(Size { width, height });
            }
            _ => (),
        }
        Ok(())
    }

    fn evaluate_key_event(&mut self, event: &KeyEvent) -> io::Result<()> {
        let KeyEvent {
            code, modifiers, ..
        } = event;
        match code {
            KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                self.shoud_quit = true;
            }
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Char('h' | 'j' | 'k' | 'l')
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::Home
            | KeyCode::End => self.move_caret(*code)?,
            _ => (),
        }
        Ok(())
    }

    fn move_caret(&mut self, code: KeyCode) -> io::Result<()> {
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.caret_location.y = self.caret_location.y.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.caret_location.y = min(
                    self.caret_location.y.saturating_add(1),
                    Terminal::size()?.height,
                );
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.caret_location.x = self.caret_location.x.saturating_sub(1);
            }
            KeyCode::Right | KeyCode::Char('l') => {
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

    fn refresh_screen(&mut self) -> io::Result<()> {
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
