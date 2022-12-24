extern crate rustop;

mod args;
mod insn;
mod engine;
mod parser;
mod reader;
mod func;
mod file;

use insn::Insn;
use rustop::opts;
use engine::Engine;
use reader::Reader;
use crate::parser::Parser;

fn main() {
    let (args, _) = opts! {
        synopsis "A simple programming language";
        version "0.0.1";
        param file:Option<String>, desc:"Input file (will use stdin if not specified).";
    }
    .parse_or_exit();

    let mut engine = Engine::new();
    let file = Parser::new(Reader::new(args.file)).parse();
    match file.get_func("__main__") {
        Some(s) => s.exec(&mut engine),
        None => {
            match file.get_func("main") {
                Some(u) => u.exec(&mut engine),
                None => panic!("No main method")
            }
        }
    };
}
