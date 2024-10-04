use std::io;

use super::terminal::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> io::Result<()> {
        let height = Terminal::size()?.height;
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit up or down
        #[allow(clippy::integer_division)]
        let welcom_message_row = height / 3;
        for row in 0..height {
            Terminal::clear_line()?;
            if row == 0 {
                Terminal::print("Hello, world!")?;
            } else {
                Self::draw_empty_row()?;
            }
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
        Terminal::print(&message)?;
        Ok(())
    }
}
