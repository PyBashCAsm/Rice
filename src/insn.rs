pub struct Insn {
    ins: Ins,
    args: Box<Vec<String>>
}

impl Insn {
    pub fn new(line: &String) -> Self {
        let mut acc = String::new();
        let mut args: Vec<String> = Vec::new();

        for i in line.chars() {
            if i == ' ' && acc.len() != 0 {
                args.push(acc.clone());
            }

            acc.push(i);
        }

        Insn {
            ins: Ins::new(&args[0]),
            args: Box::new(args)
        }
    }

    fn verify(&self) -> bool {
        match ins {
            Ins::MOV => {
            }
        }
    }
}

enum Ins {
    MOV,
}

impl Ins {
    pub fn new(ins: &str) -> Self {
        match ins {
            "mov" | "MOV" => Ins::MOV,
            _ => panic!("{} is not a valid insn", ins)
        }
    }
}
