#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec!["Hello, World!".into(), "hoge".into()],
        }
    }
}
