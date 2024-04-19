use core::fmt;
use std::{fs, fmt::Debug, io::{stdin, stdout, Write}};

use crate::scanner::Scanner;

#[derive(Debug, Clone)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    LESS_EQUAL, LESS,
    GREATER_EQUAL, GREATER,

    STRING(String), NUMBER(f32),

    IDENTIFIER,

    IF, NIL, WHILE, TRUE, FALSE,
    
    EOF,

    NONE
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub line: i32
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self._type, self.lexeme)

    }
}

pub fn error(line: i32, message: &str) {
    println!("[line {}] Error: {}", line, message);
}


fn run(line: &str) {
    println!(":{}:", line);
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