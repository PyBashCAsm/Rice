use crate::func::Func;

pub struct File {
    name: String,
    funcs: Vec<Func>,
}

impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: String::new(),
            funcs: Vec::new(),
        }
    }

    pub fn add_func(&mut self, func: Func) {
        self.funcs.push(func);
    }

    pub fn get_func(&self, name: &str) -> Option<&Func> {
        for func in &self.funcs {
            if func.name == name {
                return Some(func);
            }
        }

        None
    }
}
