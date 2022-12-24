use crate::args::{Args, Constant, Regs};

pub struct Engine {
    regs: Vec<Constant>,
}

impl Engine {
    pub fn new() -> Self {
        let mut regs: Vec<Constant> = Vec::with_capacity(12);
        let mut iter = 0;
        while iter < 12 {
            regs.push(Constant::new("0"));
            iter += 1;
        }

        Engine { regs }
    }


    pub fn mov(&mut self, dest: Regs, args: Args) {
        self.regs[dest as usize] = match args {
            Args::Value(s) => s,
            Args::Regs(a) => self.get(a)
        };
    }

    pub fn get(&self, dest: Regs) -> Constant {
        self.regs[dest as usize].clone()
    }

    pub fn ops(&mut self, op: char, dest: Regs, arg1: Constant, arg2: Constant) -> Result<(), String>{
        let val1 = match arg1 {
            Constant::Int(i) => i as f64,
            Constant::Float(f) => f as f64,
            Constant::Str(..) => return Err(String::from("cannot use math instructions on strings")),
            Constant::Result(r) => r,
        };

        let val2 = match arg2 {
            Constant::Int(i) => i as f64,
            Constant::Float(f) => f as f64,
            Constant::Str(..) => return Err(String::from("cannot use math instructions on strings")),
            Constant::Result(r) => r,
        };

        match op {
            '+' => self.regs[dest as usize] = Constant::Result(val1 + val2),
            '-' => self.regs[dest as usize] = Constant::Result(val1 - val2),
            '*' => self.regs[dest as usize] = Constant::Result(val1 * val2),
            '/' => self.regs[dest as usize] = Constant::Result(val1 / val2),
            _ => return Err(String::from("Invalid op passed to engine.ops() (internal error)"))
        }
        Ok(())
    }
}
