use std::{
    fmt::Display,
    io::{self, stdout, Write},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Command,
};

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the cursor out of these bounds, it will also be truncated.
pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> io::Result<()> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position::origin())?;
        Ok(())
    }

    pub fn terminate() -> io::Result<()> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> io::Result<()> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> io::Result<()> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn print<T: Display>(message: T) -> io::Result<()> {
        Self::queue_command(Print(message))?;
        Ok(())
    }

    /// Moves the cursor to the given Position.
    /// # Arguments
    /// * `Position` - the  `Position`to move the cursor to. Will be truncated to `u16::MAX` if bigger.
    pub fn move_cursor_to(position: Position) -> io::Result<()> {
        // clippy::as_conversions: See doc above
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }

    pub fn hide_cursor() -> io::Result<()> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> io::Result<()> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> io::Result<Size> {
        let (width, height) = terminal::size()?;
        Ok(Size {
            // clippy::as_conversions: See doc above
            #[allow(clippy::as_conversions)]
            width: width as usize,
            #[allow(clippy::as_conversions)]
            height: height as usize,
        })
    }

    pub fn execute() -> io::Result<()> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> io::Result<()> {
        queue!(stdout(), command)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}
