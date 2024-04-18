use crate::lex::{error, Token, TokenType};

struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: i32,
	current: i32,
	line: i32
}

#[allow(dead_code)]
impl Scanner {
	fn new(source: &str) -> Self {
		Self {source: source.to_string(), tokens: Vec::new(), start: 0, current: 0, line: 1}
	} 

	fn scan_tokens(&mut self) -> &Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}
		self.tokens.push(Token{_type: crate::lex::TokenType::EOF, lexeme: "".to_string(), literal: None, line: self.line, });
		&self.tokens
	}

	fn is_at_end(&mut self) -> bool {
		self.current >= self.source.len() as i32
	}

	fn scan_token(&mut self) {
		let c = self.advance();
		let token = match c {
			'(' => TokenType::LEFT_PAREN,
			')' => TokenType::RIGHT_PAREN,
			'{' => TokenType::LEFT_BRACE,
			'}' => TokenType::RIGHT_BRACE,
			',' => TokenType::COMMA,
			'.' => TokenType::DOT,
			'-' => TokenType::MINUS,
			'+' => TokenType::PLUS,
			';' => TokenType::SEMICOLON,
			'*' => TokenType::STAR,
			'!' => if self._match('=') {TokenType::BANG_EQUAL} else {TokenType::BANG},
			'=' => if self._match('=') {TokenType::EQUAL_EQUAL} else {TokenType::EQUAL},
			'<' => if self._match('=') {TokenType::LESS_EQUAL} else {TokenType::LESS},
			'>' => if self._match('=') {TokenType::GREATER_EQUAL} else {TokenType::GREATER},
			'/' => if self._match('/') {
					// Comment until end of line
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					};
					TokenType::NONE
				} else {
					TokenType::SLASH
				},
			'\n' => {self.line += 1; TokenType::NONE},
			' ' => TokenType::NONE,
			'\r' => TokenType::NONE,
			'\t' => TokenType::NONE,
			_ => {error(self.line, "Unexpected character."); TokenType::NONE}
		};

		self.add_token(token, None)
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
		self.source.chars().nth(self.current as usize).unwrap()
	}

	// Creates new token for character
	fn add_token(&mut self,_type: TokenType, literal: Option<Box<dyn std::any::Any>>) {
		let text = &self.source[self.start as usize..self.current as usize];
		self.tokens.push(Token{_type, lexeme: text.to_string(), literal, line: self.line});
	}
}

