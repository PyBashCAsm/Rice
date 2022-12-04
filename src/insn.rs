use crate::args::Regs;

pub struct Insn {
    ins: Ins,
    args: Box<Vec<String>>
}

#[inline]
fn arg_check (arg: usize, lim: usize) {
  if arg < lim {
    panic!("Too few arguments for instruction");
  }
  
  if arg > lim {
    panic!("Too many arguments for instruction");
  }
}

impl Insn {
    pub fn new(line: &String) -> Self {
        let mut acc = String::new();
        let mut args: Vec<String> = Vec::new();
        let mut string = false;
        
        for i in line.chars() {
            if i == '"' {
              if !string {
                string = true;
                continue;
              }
              
              string = false;
              args.push(acc.clone());
              acc.clear();
              continue;
            }
            
            if (i == ' '  || i == ',') && !string  {
               if acc.len() != 0 {
                     args.push(acc.clone());
                     acc.clear();
                }
                
                if i == ' ' {
                  continue;
                }
                
                if i == ',' {
                  args.push(String::from(","));
                  continue;
                }
            }

            acc.push(i);
        }
        
        if string {
          panic!("Unclosed string literal");
        }
        
        for i in args.iter() {
          println!("{i}");
        }
        Insn {
            ins: Ins::new(&args[0]),
            args: Box::new(args)
        }
    }

    fn verify(&self) -> bool {
        match self.ins {
            Ins::MOV => {
              arg_check(self.args.len(), 3);
              
              true
            }
            
            _ => unreachable!()
        }
    }
}

enum Ins {
    MOV,
}

impl Ins {
    pub fn new(ins: &str) -> Self {
        match ins {
            "mov" | "MOV" => Ins::MOV,
            _ => panic!("{} is not a valid insn", ins)
        }
    }
}
