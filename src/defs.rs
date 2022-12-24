use crate::args::Constant;

pub struct Defs {
    pub name: String,
    pub value: Constant
}

impl Defs {
    pub fn new(name: &str, value: Constant) -> Self {
        Defs {
            name: String::from(name),
            value
        }
    }
}