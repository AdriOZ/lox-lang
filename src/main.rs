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
        println!("{}", line);
    }
}

fn run_file(filename: &String) {
    let path = Path::new(filename.as_str());
    if path.is_file() {
        let contents = fs::read_to_string(path).expect("Cannot read file");
        println!("File contents:\n{}", contents);
    } else {
        println!("File {} does not exist", filename);
    }
}
