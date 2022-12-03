extern crate rustop;
mod stdin;
mod insn;

use rustop::opts;
use stdin::StdinReader;
use insn::Insn;

fn main() {
    let (args, _) = opts!{
        synopsis "A simple programming language";
        version "0.0.1";
        param file:Option<String>, desc:"Input file (will use stdin if not specified).";
    }.parse_or_exit();

   match args.file {
       Some(s) => unimplemented!(),
       None => {
           let mut reader = StdinReader::new();
           let mut s = String::new();
           loop {
               reader.read_line(&mut s);
               let s = Insn::new(&s);
           }
       }
   }
}
