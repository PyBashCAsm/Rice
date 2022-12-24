use crate::insn::Insn;
use crate::parser::Parser;
use crate::{Engine, Reader};

pub struct Func {
  pub name: String,
  insn: Vec<Insn>
}

impl Func {
  pub fn new(name: &str, parser: &mut Parser) -> Self {
    let mut insn: Vec<Insn> = Vec::new();
    let mut src = parser.inner_reader();
    loop {
      let line = src.read_line();
      if src.is_eof() {
        if name == "__main__" {
          break;
        }
        parser.error(&format!("Unexpected EOF (Method {} ended abruptly without having 'end')", name))
      }

      if line.contains("end") {
        break;
      }
      else if line.len() == 1 {
          continue;
      }
      insn.push(Insn::new(&line));
    }

    Func {
      name: String::from(name),
      insn
    }
  }

  pub fn exec(&self, engine: &mut Engine) -> bool {
    for ins in &self.insn {
      ins.exec(engine);
    }
    true
  }

}
