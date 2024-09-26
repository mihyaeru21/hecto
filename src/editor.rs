use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Debug, Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        enable_raw_mode().unwrap();

        loop {
            let event = match read() {
                Ok(Event::Key(e)) => e,
                Err(err) => {
                    println!("Error: {err}");
                    continue;
                }
                _ => continue,
            };

            println!("{event:?} \r");

            if let KeyCode::Char(c) = event.code {
                if c == 'q' {
                    break;
                }
            }
        }

        disable_raw_mode().unwrap();
    }
}
