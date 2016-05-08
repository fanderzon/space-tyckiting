use std::io::prelude::*;
use std::fs::File;

pub struct Logger {
    file: Option<File>,
}

static TAB: &'static str = "    ";

impl Logger {
    pub fn new() -> Logger {
        return Logger {
            file: File::create("log.txt").ok(),
        }
    }

    pub fn log(&mut self, msg: &str, indent: usize) {
        if let Some(ref mut f) = self.file {
            for _ in 0..indent {
                let _ = f.write_all(TAB.as_bytes());
            }
            let _ = f.write_all(msg.as_bytes());
            let _ = f.write_all(b"\n");
        }
    }
}
