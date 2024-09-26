use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Debug, Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            if let Event::Key(event) = read()? {
                println!("{event:?} \r");

                if let KeyCode::Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            };
        }

        disable_raw_mode()?;

        Ok(())
    }
}
