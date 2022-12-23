// This file is no longer in use and will be removed in the future
use std::io;

pub struct StdinReader {
    line: u32,
}

impl StdinReader {
    pub fn new() -> Self {
        StdinReader { line: 0u32 }
    }

    pub fn read_line(&mut self, dest: &mut String) {
        let line = io::stdin().read_line(dest);
        match line {
            Ok(s) => {
                if s == 0 {
                    std::process::exit(0);
                }

                let trim = dest.trim_end_matches('\n').len();
                dest.truncate(trim);
                dest.push(' ');
                self.line += 1;
            }

            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
