use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub struct Reader {
  file: String,
  line: u32,
  src: Option<BufReader<File>>,
}

impl Reader {
  pub fn new(file: Option<String>) -> Self {
    if file.is_some() {
      let f = file.unwrap();
      match File::open(&f) {
        Ok(handle) => return Reader { file: f, line: 0, src: Some(BufReader::new(handle)) },
        Err(e) => panic!("Error while opening file {f} : {e}")
      }
    }
    
    Reader {
      file: String::from("<stdin>"),
      line: 0,
      src: None
    }
  }

  pub fn read_line(&mut self) -> String {
    let mut buf = String::new();
    if self.src.is_some() {
      match self.src.as_mut().unwrap().read_line(&mut buf) {
        Ok(s) => {
          if s == 0 {
            return String::from("<EOF>");
          }
          self.line += 1;
        },
        Err(e) => panic!("Failed to read {e}")
      }
    }

    else {
      io::stdin().read_line(&mut buf);
      self.line += 1;
    }

    // Remove the newline and add a space
    buf.pop();
    buf += " ";
    buf
  }

}