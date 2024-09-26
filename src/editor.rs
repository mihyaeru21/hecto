use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Debug, Default)]
pub struct Editor {
    shoud_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r",
                );

                match code {
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.shoud_quit = true;
                    }
                    _ => {}
                }

                if self.shoud_quit {
                    break;
                }
            };
        }

        disable_raw_mode()?;

        Ok(())
    }
}
