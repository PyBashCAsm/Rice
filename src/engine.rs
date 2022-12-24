use crate::args::{Args, Regs};

pub struct Engine {
    regs: Vec<Args>,
}

impl Engine {
    pub fn new() -> Self {
        let mut regs: Vec<Args> = Vec::with_capacity(12);
        let mut iter = 0;
        while iter < 12 {
            regs.push(Args::new("0"));
            iter += 1;
        }

        Engine { regs }
    }

    pub fn mov(&mut self, dest: Regs, args: Args) {
        self.regs[dest as usize] = args;
    }

    pub fn get(&self, dest: Regs) -> &Args {
        &self.regs[dest as usize]
    }
}
