use std::env;

mod scanner;
mod lex;

fn main() {
	let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => lex::run_prompt(),
        2 => lex::run_file(&args[1]),
        _ => {
            eprintln!("Usage: rjust [script]");
            std::process::exit(64);
        }
    }
}