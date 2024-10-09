use std::{fs::read_to_string, io};

#[derive(Debug, Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> io::Result<Self> {
        let contents = read_to_string(file_name)?;
        let lines: Vec<String> = contents.lines().map(String::from).collect();
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
