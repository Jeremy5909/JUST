use std::{env, fs, io::{stdin, stdout, Write}};

use crate::scanner::Scanner;

mod scanner;
mod token;

fn main() {
	let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rjust [script]");
            std::process::exit(64);
        }
    }

    fn run(line: &str) {
        let mut scanner = Scanner::new(line);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{}", token);
        }
    }
    
    pub fn run_file(path: &str) {
        let contents = fs::read_to_string(path).expect(&format!("Could not find file: '{}'",path));
        run(&contents);
    }
    
    pub fn run_prompt() {
        loop {
            print!("> ");
    
            // Flush output
            stdout().flush().unwrap();
    
            // Get input
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
    
            run(&line);
        }
    }
}