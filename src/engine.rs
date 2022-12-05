use crate::args::Args;
use crate::lazy_static::lazy_static;

struct Engine {
    regs: Vec<Args>
}

impl Engine {
    fn new() -> Self {
        Engine {
            regs: Vec::with_capacity(12)
        }
    }
}
