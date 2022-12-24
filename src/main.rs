extern crate rustop;

mod args;
mod engine;
mod file;
mod func;
mod insn;
mod parser;
mod reader;
mod defs;

use crate::parser::Parser;
use engine::Engine;
use insn::Insn;
use reader::Reader;
use rustop::opts;

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
        None => match file.get_func("main") {
            Some(u) => u.exec(&mut engine),
            None => panic!("No main method"),
        },
    };
}
