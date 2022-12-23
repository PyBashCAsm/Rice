use std::io::{self, BufReader};
use std::fs::File;

pub struct Reader {
  file: String,
  line: u32,
  src: Option<BufReader>,
}

impl Reader {
  pub fn new(file: Option<String>) -> Self {
    if file.is_some() {
      let f = file.unwrap();
      match File::open(&f) {
        Ok(handle) => Reader { file: f, line: 0, src: Some(BufReader::new(handle)) },
        Err(e) => panic!("Error while opening file {f} : {e}")
      }
    }
    
    Reader {
      file: String::from("<stdin>"),
      line: 0,
      src: None
    }
  }
}