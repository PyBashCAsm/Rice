extern crate rustop;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

mod args;
mod insn;
mod stdin;
mod engine;
mod parser;
mod reader;

use insn::Insn;
use rustop::opts;
use stdin::StdinReader;
use engine::Engine;
use reader::Reader;

fn main() {
    let (args, _) = opts! {
        synopsis "A simple programming language";
        version "0.0.1";
        param file:Option<String>, desc:"Input file (will use stdin if not specified).";
    }
    .parse_or_exit();

    let mut engine = Engine::new();
    match args.file {
        Some(s) => {
            let mut reader = Reader::new(Some(s));
            loop {
                Insn::new(&reader.read_line()).exec(&mut engine);
            }
        }
        None => {
            let mut reader = Reader::new(None);
            loop {
                Insn::new(&reader.read_line()).exec(&mut engine);
            }
        }
    }
}
