mod terminal;

use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use terminal::{Position, Terminal};

#[derive(Debug, Default)]
pub struct Editor {
    shoud_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> io::Result<()> {
        let height = Terminal::size()?.height;
        for row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn repl(&mut self) -> io::Result<()> {
        loop {
            self.refresh_screen()?;
            if self.shoud_quit {
                break;
            }

            let event = read()?;
            self.evaluate_event(&event);
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.shoud_quit = true;
                }
                _ => (),
            }
        };
    }

    fn refresh_screen(&self) -> io::Result<()> {
        Terminal::hide_cursor()?;

        if self.shoud_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position::origin())?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
