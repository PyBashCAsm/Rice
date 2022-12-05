use crate::args::Args;

pub struct Insn {
    ins: Ins,
    args: Vec<String>,
}

#[inline]
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
            args,
        }
    }

    fn exec(&self) -> bool {
        match self.ins {
            Ins::Mov => {
                arg_check(self.args.len(), 3);
                let arg1 = match Args::new(&self.args[1]) {
                    Args::Regs(s) => s,
                    Args::Value(_) => panic!("First argument must be a register"),
                };

                let arg2 = Args::new(&self.args[2]);
                true
            }
        }
    }
}

enum Ins {
    Mov,
}

impl Ins {
    pub fn new(ins: &str) -> Self {
        match ins {
            "mov" | "MOV" => Ins::Mov,
            _ => panic!("{} is not a valid instruction", ins),
        }
    }
}
