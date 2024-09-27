mod terminal;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use terminal::Terminal;

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

    fn draw_rows() -> Result<(), std::io::Error> {
        let rows = Terminal::size()?.1;
        for row in 0..rows {
            print!("~");
            if row + 1 < rows {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.shoud_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
}
