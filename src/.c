extern crate rustop;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

mod args;
mod insn;
mod stdin;
mod engine;

use insn::Insn;
use rustop::opts;
use stdin::StdinReader;
use engine::Engine;

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
            match File::open(&s) {
                Ok(handle) => {
                    let lines = BufReader::new(handle).lines();
                    for line in lines {
                        Insn::new(&line).exec(&mut engine);
                    }
                }
                Err(e) => panic!("Error opening file {s}: {e}"),
            }
        }
        None => {
            let mut reader = StdinReader::new();
            let mut s = String::new();
            loop {
                reader.read_line(&mut s);
                Insn::new(&s).exec(&mut engine);
                s.clear();
            }
        }
    }
}
