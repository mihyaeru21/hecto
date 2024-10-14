#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]

use std::io;

use editor::Editor;

mod editor;

fn main() -> io::Result<()> {
    Editor::new()?.run();
    Ok(())
}
