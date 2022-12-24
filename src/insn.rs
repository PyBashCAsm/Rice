use std::fmt;
use crate::args::{Args, Constant, Regs};
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

    pub fn exec(&self, engine: &mut Engine) -> Result<(), String> {
        let arg1 = match Regs::new(&self.args[1]) {
            Some(s) => s,
            None => return Err(String::from("First argument must be a register")),
        };
        match self.ins {
            Ins::Mov => {
                arg_check(self.args.len(), 3);
                let arg2 = Args::new(&self.args[2]);
                engine.mov(arg1, arg2);
                Ok(())
            }

            Ins::Write => {
                arg_check(self.args.len(), 2);
                println!("{}", engine.get(arg1));
                Ok(())
            }

            Ins::Add | Ins::Sub | Ins::Mul | Ins::Div => {
                arg_check(self.args.len(), 4);
                let arg2 = match Args::new(&self.args[2]) {
                    Args::Regs(s) => engine.get(s),
                    Args::Value(v) => v
                };
                let arg3 = match Args::new(&self.args[3]) {
                    Args::Regs(s) => engine.get(s),
                    Args::Value(v) => v
                };

                let s= engine.ops((self.ins as u8)  as char, arg1, arg2, arg3);
                s
            }
        }
    }
}

impl fmt::Display for Insn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut total = String::new();
        for i in &self.args {
            total += i;
            total += " ";
        }
        write!(f, "{total}")
    }
}

#[derive(Copy, Clone)]
enum Ins {
    Mov,
    Write,
    Add = '+' as isize,
    Sub = '-' as isize,
    Mul = '*' as isize,
    Div = '/' as isize,
}

impl Ins {
    pub fn new(ins: &str) -> Self {
        match ins {
            "mov" | "MOV" => Ins::Mov,
            "write" | "WRITE" => Ins::Write,
            "add" | "ADD" => Ins::Add,
            "sub" | "SUB" => Ins::Sub,
            "mul" | "MUL" => Ins::Mul,
            "div" | "DIV" => Ins::Div,
            _ => panic!("{} is not a valid instruction", ins),
        }
    }
}
