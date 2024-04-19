use core::fmt;
use std::{fmt::Debug, io::{stdin, stdout, Write}};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
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
}

#[derive(Debug)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub line: usize
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self._type, self.lexeme)
    }
}

pub fn error(line: usize, message: &str) {
    println!("[line {}] Error: {}", line, message);
}