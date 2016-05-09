extern crate time;

use std::io::prelude::*;
use std::fs::File;

pub struct Logger {
    file: Option<File>,
}

static TAB: &'static str = "    ";

impl Logger {
    pub fn new() -> Logger {
        return Logger {
            file: File::create(Logger::make_filename()).ok(),
        }
    }

    fn make_filename() -> String {
        let tm = time::now().to_timespec();
        let msg = format!("logger_{}:{}.log", tm.sec, tm.nsec);
        return msg;
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

    pub fn write_q(&mut self, q: &Vec<(String, usize)>) {
        for &(ref msg, indent) in q {
            self.log(&msg, indent);
        }
    }
}
