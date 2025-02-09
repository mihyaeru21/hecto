mod buffer;
mod terminal;
mod view;

use std::{
    cmp::min,
    env, io,
    panic::{set_hook, take_hook},
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use terminal::{Position, Size, Terminal};
use view::View;

#[derive(Debug)]
pub struct Editor {
    shoud_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panick_info| {
            let _ = Terminal::terminate();
            current_hook(panick_info);
        }));
        Terminal::initialize()?;

        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            shoud_quit: false,
            location: Location::default(),
            view,
        })
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.shoud_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Event::Key(e) => self.evaluate_key_event(e),
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
    }

    fn evaluate_key_event(&mut self, event: &KeyEvent) {
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
            | KeyCode::End => self.move_point(*code),
            _ => (),
        }
    }

    fn move_point(&mut self, code: KeyCode) {
        let size = Terminal::size().unwrap_or_default();
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.location.y = self.location.y.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.location.y = min(self.location.y.saturating_add(1), size.height);
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.location.x = self.location.x.saturating_sub(1);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.location.x = min(self.location.x.saturating_add(1), size.width);
            }
            KeyCode::PageUp => self.location.y = 0,
            KeyCode::PageDown => self.location.y = size.height.saturating_sub(1),
            KeyCode::Home => self.location.x = 0,
            KeyCode::End => self.location.x = size.width.saturating_add(1),
            _ => (),
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.shoud_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
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
