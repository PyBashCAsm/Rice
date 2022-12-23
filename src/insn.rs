use crate::args::{Args, Regs};
use crate::engine::Engine;
use crate::parser;

pub struct Insn {
    ins: Ins,
    args: Vec<String>,
}

#[inline(always)]
fn arg_check(arg: usize, lim: usize) {
    if arg < lim {
        panic!("Too few arguments for instruction");
    }

    if arg > lim {
        panic!("Too many arguments for instruction");
    }
}

impl Insn {
    pub fn new(line: &str) -> Self {
        let args = parser::split(line);
        Insn {
            ins: Ins::new(&args[0]),
            args,
        }
    }

    pub fn exec(&self, engine: &mut Engine) -> bool {
        match self.ins {
            Ins::Mov => {
                arg_check(self.args.len(), 3);
                let arg1 = match Regs::new(&self.args[1]) {
                    Some(s) => s,
                    None => panic!("First argument must be a register"),
                };

                let arg2 = Args::new(&self.args[2]);
                engine.mov(arg1, arg2);
                true
            }

            Ins::Write => {
                arg_check(self.args.len(), 2);
                let arg1 = match Regs::new(&self.args[1]) {
                    Some(s) => s,
                    None => panic!("First argument must be a register"),
                };
                println!("{}", engine.get(arg1));
                true
            }

        }
    }
}

enum Ins {
    Mov,
    Write
}

impl Ins {
    pub fn new(ins: &str) -> Self {
        match ins {
            "mov" | "MOV" => Ins::Mov,
            "write" | "WRITE" => Ins::Write,
            _ => panic!("{} is not a valid instruction", ins),
        }
    }
}
