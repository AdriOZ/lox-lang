pub mod ast;
pub mod parser;
pub mod scanner;
pub mod token;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        run_repl();
    }
}

fn run_repl() {
    loop {
        let mut line = String::new();
        println!("$> ");
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut scanner = scanner::Scanner::new(&line);
        let tokens = scanner.parse();
        println!("{:#?}", tokens);
    }
}

fn run_file(filename: &String) {
    let path = Path::new(filename.as_str());
    if path.is_file() {
        let contents = fs::read_to_string(path).expect("Cannot read file");
        let mut scanner = scanner::Scanner::new(&contents);
        let tokens = scanner.parse();
        let mut parser = parser::Parser::new(tokens);
        println!("{:#?}", parser.parse());
    } else {
        println!("File {} does not exist", filename);
    }
}
