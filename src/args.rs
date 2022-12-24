use std::fmt;

#[derive(Copy, Clone)]
pub enum Regs {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
}

impl Regs {
    pub fn new(reg: &str) -> Option<Self> {
        match reg {
            "r0" => Some(Regs::R0),
            "r1" => Some(Regs::R1),
            "r2" => Some(Regs::R2),
            "r3" => Some(Regs::R3),
            "r4" => Some(Regs::R4),
            "r5" => Some(Regs::R5),
            "r6" => Some(Regs::R6),
            "r7" => Some(Regs::R7),
            "r8" => Some(Regs::R8),
            "r9" => Some(Regs::R9),
            "r10" => Some(Regs::R10),
            "r11" => Some(Regs::R11),
            _ => None,
        }
    }
}

impl fmt::Display for Regs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}", *self as usize)
    }
}

impl Into<usize> for Regs {
    fn into(self) -> usize {
        self as usize
    }
}
#[derive(Clone)]
pub enum Constant {
    Int(i32),
    Float(f32),
    Result(f64),
    Str(String),
}

impl Constant {
    pub fn new(src: &str) -> Self {
        match src.parse::<i32>() {
            Ok(s) => Constant::Int(s),
            Err(_) => match src.parse::<f32>() {
                Ok(f) => Constant::Float(f),
                Err(_) => Constant::Str(String::from(src)),
            },
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Float(fl) => write!(f, "{}", fl),
            Constant::Str(s) => write!(f, "{}", s),
            Constant::Result(r) => write!(f, "{}", r)
        }
    }
}

pub enum Args {
    Regs(Regs),
    Value(Constant),
}

impl Args {
    pub fn new(arg: &str) -> Self {
        if let Some(s) = Regs::new(arg) {
            return Args::Regs(s);
        }

        Args::Value(Constant::new(arg))
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Args::Value(s) => {
                write!(f, "{}", s)
            }

            Args::Regs(r) => {
                write!(f, "{}", r)
            }
        }
    }
}
