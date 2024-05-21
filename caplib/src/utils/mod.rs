use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
struct Screen {
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy)]
enum Encoder {
    Mp4,
    Avi,
    Hevc,
    Wmv,
}
