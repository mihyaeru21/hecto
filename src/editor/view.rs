use std::{cmp::min, io};

use super::{
    buffer::Buffer,
    terminal::{Position, Size, Terminal},
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn render(&mut self) -> io::Result<()> {
        if !self.needs_redraw {
            return Ok(());
        }

        let Size { width, height } = self.size;
        if width == 0 || height == 0 {
            return Ok(());
        }

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit up or down
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let end = min(line.len(), width);
                Self::render_line(current_row, &line[0..end])?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcom_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }

        self.needs_redraw = false;

        Ok(())
    }

    fn render_line(at: usize, line_text: &str) -> io::Result<()> {
        Terminal::move_caret_to(Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    fn build_welcom_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let mut message = format!("{NAME} editor -- version {VERSION}");
        let len = message.len();
        if width <= len {
            return "~".to_string();
        }

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = width.saturating_sub(len).saturating_sub(1) / 2;
        let spaces = " ".repeat(padding);
        message = format!("~{spaces}{message}");
        message.truncate(width);
        message
    }
}
