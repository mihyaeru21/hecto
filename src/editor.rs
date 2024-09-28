mod terminal;

use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use terminal::{Position, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

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

    fn draw_rows() -> io::Result<()> {
        let height = Terminal::size()?.height;
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit up or down
        #[allow(clippy::integer_division)]
        let welcom_message_row = height / 3;
        for row in 0..height {
            Terminal::clear_line()?;
            Self::draw_empty_row()?;
            if row == welcom_message_row {
                Self::draw_welcom_message()?;
            }
            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_empty_row() -> io::Result<()> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcom_message() -> io::Result<()> {
        let mut message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = message.len();
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("{spaces}{message}");
        message.truncate(width);
        Terminal::print(message)?;
        Ok(())
    }
}
