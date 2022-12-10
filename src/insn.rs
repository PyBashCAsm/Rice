use crate::args::{Args, Regs};
use crate::engine::Engine;

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
        let mut acc = String::new();
        let mut args: Vec<String> = Vec::new();
        let mut string = false;
        let mut exp = -2;

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

            if (i == ' ' || i == ',') && !string {
                if !acc.is_empty() {
                    if exp == -2 || exp == -1 || exp == 1 {
                        args.push(acc.clone());
                        acc.clear();
                        if exp == -2 {
                            exp = -1;
                        }

                        else {
                            exp = 0;
                        }
                    }

                    else {
                        panic!("',' expected here");
                    }
                }

                if i == ' ' {
                    continue;
                }

                if i == ',' {
                    exp = 1;
                    continue;
                }
            }

            acc.push(i);
        }

        if string {
            panic!("Unclosed string literal");
        }

        
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
    Write,
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
