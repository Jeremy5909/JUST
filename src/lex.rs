use core::fmt;
use std::{fmt::Debug, io::{stdin, stdout, Write}};

#[derive(Debug)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    LESS_EQUAL, LESS,
    GREATER_EQUAL, GREATER,

    STRING, NUMBER,

    EOF,

    NONE
}

pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn std::any::Any>>,
    line: i32
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self._type, self.lexeme, self.literal)

    }
}
fn scanToken() {
    
}
pub fn error(line: i32, message: &str) {
    println!("[line {}] Error: {}", line, message);
}

fn run(line: &str) {
    let tokens = line.chars();
    for token in tokens {
        print!("{}", token);
    }
}

pub fn run_file(path: &str) {

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