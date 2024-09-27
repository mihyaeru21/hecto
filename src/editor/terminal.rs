use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

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
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> io::Result<()> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn print(message: &str) -> io::Result<()> {
        queue!(stdout(), Print(message))?;
        Ok(())
    }

    pub fn execute() -> io::Result<()> {
        stdout().flush()?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> io::Result<()> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn hide_cursor() -> io::Result<()> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> io::Result<()> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn size() -> io::Result<Size> {
        let (width, height) = terminal::size()?;
        Ok(Size { width, height })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}
