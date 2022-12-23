use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Reader {
    file: String,
    line: u32,
    store: Option<String>,
    src: Option<BufReader<File>>,
    eof: bool,
}

impl Reader {
    pub fn new(file: Option<String>) -> Self {
        if file.is_some() {
            let f = file.unwrap();
            match File::open(&f) {
                Ok(handle) => {
                    return Reader {
                        file: f,
                        line: 0,
                        store: None,
                        src: Some(BufReader::new(handle)),
                        eof: false,
                    }
                }
                Err(e) => panic!("Error while opening file {f} : {e}"),
            }
        }

        Reader {
            file: String::from("<stdin>"),
            line: 0,
            store: None,
            src: None,
            eof: false,
        }
    }

    pub fn read_line(&mut self) -> String {
        if self.store.is_some() {
            let s = self.store.as_ref().unwrap().clone();
            self.store = None;
            return s;
        }
        let mut buf = String::new();
        if self.src.is_some() {
            match self.src.as_mut().unwrap().read_line(&mut buf) {
                Ok(s) => {
                    if s == 0 {
                        self.eof = true;
                        return String::from("");
                    }
                    self.line += 1;
                }
                Err(e) => panic!("Failed to read {e}"),
            }
        } else {
            io::stdin().read_line(&mut buf);
            self.line += 1;
        }

        // Remove the newline and add a space
        buf.pop();
        buf += " ";
        buf
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn is_eof(&self) -> bool {
        self.eof
    }

    pub fn push_back(&mut self, push: &str) {
        self.store = Some(String::from(push));
    }
}
