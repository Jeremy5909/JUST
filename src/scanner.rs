use std::collections::HashMap;

use crate::token::{error, Token, TokenType};

pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: usize,
	keywords: HashMap<String, TokenType>,
}

#[allow(dead_code)]
impl Scanner {
	pub fn new(source: &str) -> Self {
		Self {source: source.to_string(),
			tokens: Vec::new(),
			start: 0,
			current: 0,
			line: 1,
			keywords: HashMap::from([
				("if".to_string(), TokenType::IF),
				("nil".to_string(), TokenType::NIL),
				("while".to_string(), TokenType::WHILE),
				("true".to_string(), TokenType::TRUE),
				("false".to_string(), TokenType::FALSE),
			])}
	} 

	pub fn scan_tokens(&mut self) -> &Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}
		self.tokens.push(Token{_type: crate::token::TokenType::EOF, lexeme: "".to_string(), line: self.line, });
		&self.tokens
	}

	fn is_at_end(&mut self) -> bool {
		self.current >= self.source.len()
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		match c {
			'(' => self.add_token(TokenType::LEFT_PAREN),
			')' => self.add_token(TokenType::RIGHT_PAREN),
			'{' => self.add_token(TokenType::LEFT_BRACE),
			'}' => self.add_token(TokenType::RIGHT_BRACE),
			',' => self.add_token(TokenType::COMMA),
			'.' => self.add_token(TokenType::DOT),
			'-' => self.add_token(TokenType::MINUS),
			'+' => self.add_token(TokenType::PLUS),
			';' => self.add_token(TokenType::SEMICOLON),
			'*' => self.add_token(TokenType::STAR),
			'!' => if self._match('=') {self.add_token(TokenType::BANG_EQUAL)} else {self.add_token(TokenType::BANG)},
			'=' => if self._match('=') {self.add_token(TokenType::EQUAL_EQUAL)} else {self.add_token(TokenType::EQUAL)},
			'<' => if self._match('=') {self.add_token(TokenType::LESS_EQUAL)} else {self.add_token(TokenType::LESS)},
			'>' => if self._match('=') {self.add_token(TokenType::GREATER_EQUAL)} else {self.add_token(TokenType::GREATER)},
			'/' => if self._match('/') {
				while self.peek() != '\n' && !self.is_at_end(){self.advance();};
			} else {
				self.add_token(TokenType::SLASH);
			},
			' ' | '\r' | 't' => (),
			'"' => self.string(),
			'\n' => {self.line += 1},

			_ => {
				if c.is_digit(10) {
					self.number();
				} else if c.is_alphabetic() {
					self.identifier();
				} else {
					error(self.line, &format!("Unexpected character: '{}'.", c))
				}}
		};
	}

	fn identifier(&mut self) {
		while self.peek().is_alphanumeric() {self.advance();}
		let text = &self.source[self.start as usize..self.current as usize];
		let _type = self.keywords.get(text);

		// IDENTIFIER by default
		let _type = match _type {
			None => TokenType::IDENTIFIER,
			Some(x) => x.clone(),
		};
		self.add_token(_type);
	}

	fn number(&mut self) {
		while self.peek().is_digit(10) {self.advance();}
		if self.peek() == '.' && self.peek_next().is_digit(10) {
			self.advance();
			while self.peek().is_digit(10) {self.advance();}
		}
		let value = TokenType::NUMBER(self.source[self.start as usize..self.current as usize].parse::<f32>().expect("Cannot convert number to number"));
		self.add_token(value);
	}

	fn peek_next(&mut self) -> char {
		if self.current + 1 >= self.source.len() {return '\0'}
		self.source.chars().nth((self.current + 1) as usize).unwrap()
	}

	fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' {self.line+=1};
			self.advance();
		}
		if self.is_at_end() {
			error(self.line, "Unterminated string");
			return;
		}
		self.advance();
		let value = &self.source[(self.start+1) as usize..(self.current - 1) as usize];
		self.add_token(TokenType::STRING(value.to_string()));
	}

	fn peek(&mut self) -> char {
		if self.is_at_end() {return '\0'}
		self.source.chars().nth(self.current as usize).unwrap()
	}

	fn _match(&mut self, expected: char) -> bool {
		if self.is_at_end() {return false};
		if self.source.chars().nth(self.current as usize) != Some(expected) {return false};
		
		self.current += 1;
		true
	}

	// Gets next character
	fn advance(&mut self) -> char {
		self.current += 1;
		self.source.chars().nth(self.current-1 as usize).expect("Next character not found")
	}

	// Creates new token for character
	fn add_token(&mut self,_type: TokenType) {
		let text = &self.source[self.start as usize..self.current as usize];
		self.tokens.push(Token{_type, lexeme: text.to_string(), line: self.line});
	}
}

