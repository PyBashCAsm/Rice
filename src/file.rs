use crate::args::Constant;
use crate::defs::Defs;
use crate::func::Func;

pub struct File {
    name: String,
    defs: Vec<Defs>,
    funcs: Vec<Func>,
}

impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            defs: Vec::new(),
            funcs: Vec::new(),
        }
    }

    pub fn add_func(&mut self, func: Func) {
        self.funcs.push(func);
    }
    pub fn add_def(&mut self, def: Defs) { self.defs.push(def) }
    pub fn get_func(&self, name: &str) -> Option<&Func> {
        self.funcs.iter().find(|&func| func.name == name)
    }
    pub fn get_def(&self, name: &str) ->Option<&Defs> {
        self.defs.iter().find(|&func| func.name == name)
    }
}
