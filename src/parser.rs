use crate::file::File;
use crate::func::Func;
use crate::Reader;

pub struct Parser {
    line: String,
    src: Reader,
}

pub fn split(line: &str) -> Vec<String> {
    let mut acc = String::new();
    let mut args: Vec<String> = Vec::new();
    let mut string = false;
    let mut comm = false;
    let mut exp = -2;

    // Before you move through this code, make sure to read this
    // Basically we loop through every character and it to acc
    // If the character is a whitespace and acc is not an empty
    // string (i.e not equal to ""), then whatever is in acc
    // is assumed to be a token and pushed to the vector
    // This is the simple part.
    //
    // Now coming to the parsing of double quotes or the string
    // literals. If a double quote is encountered anywhere, it is
    // assumed that everything is a part of the literal till the
    // next double quote. Multiline strings are *not* supported
    // so the next double quote has to be in the same line. If not
    // found, an error is thrown. The string flag is true if we
    // are in text between the quotes and false if we are not
    //
    // Lastly, we come to exp that is used for parsing
    // the commas. Exp can have four possible values: -2, -1, 0
    // and 1. These values indicate whether a comma is expected or
    // not. The values are explained as:
    // -2 = This is the beginning of the instruction, comma is
    // not expected here so add anything that is in acc to args
    // -1 = This is the first argument after the instruction has
    // been passed. No commas expected here too so do the same
    // -2.
    // 0 = Comma was expected but not found.
    // 1 = Comma was expected and found. Push acc to args.

    for i in line.chars() {
        if i == '/' {
            if comm {
                break;
            }

            comm = true;
        }

        else if comm {
            panic!("Expected '/' here");
        }

        if i == '"' {
            if !string {
                string = true;
                continue;
            }

            string = false;
            args.push(acc.clone());
            acc.clear();
            continue;
        }

        if (i == ' ' || i == ',') && !string {
            if !acc.is_empty() {
                if exp == -2 || exp == -1 || exp == 1 {
                    args.push(acc.clone());
                    acc.clear();

                    // This was the instruction, now change exp to
                    // -1 so we can prepare for the first argument
                    if exp == -2 {
                        exp = -1;
                    }
                    // This was an argument. A comma is expected
                    // after this if more arguments are to be
                    // given
                    else {
                        exp = 0;
                    }
                } else {
                    panic!("',' expected here");
                }
            }

            if i == ' ' {
                continue;
            }

            if i == ',' {
                exp = 1;
                continue;
            }
        }

        acc.push(i);
    }

    if string {
        panic!("Unclosed string literal");
    }

    args
}

impl Parser {
    pub fn new(src: Reader) -> Self {
        Parser {
            line: String::new(),
            src,
        }
    }

    pub fn parse(&mut self) -> File {
        let mut file = File::new(&self.src.file());
        loop {
            self.line = self.src.read_line();
            if self.src.is_eof() {
                break;
            }
            let top = true;
            let parts = split(&self.line);
            if parts.len() == 0 {
                continue;
            }

            if parts[0] == "function" {
                if parts.len() < 2 {
                    self.error("Expected identifier (name of function) ");
                }
                file.add_func(Func::new(&parts[1], self));
            } else {
                self.src.push_back(&self.line);
                file.add_func(Func::new("__main__", self));
            }
        }

        file
    }

    pub fn error(&self, reason: &str) -> ! {
        eprintln!(
            "Error:{}:{} - {}\nReason: {}",
            self.src.line(),
            self.src.file(),
            self.line,
            reason
        );
        std::process::exit(1)
    }

    pub fn is_eof(&self) -> bool {
        self.src.is_eof()
    }

    pub fn inner_reader(&mut self) -> &mut Reader {
        &mut self.src
    }
}
