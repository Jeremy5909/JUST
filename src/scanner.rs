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
		self.tokens.push(Token{_type: crate::lex::TokenType::EOF, lexeme: "".to_string(), line: self.line, });
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
			_ => {error(self.line, "Unexpected character."); TokenType::NONE}
		};

		
		match c {
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
				} else {
					self.add_token(token)};
				}
		}
	}

	fn number(&mut self) {
		while self.peek().is_digit(10) {self.advance();}
		if self.peek() == '.' && self.peek_next().is_digit(10) {
			self.advance();
			while self.peek().is_digit(10) {self.advance();}
		}
		let value = TokenType::NUMBER(self.source[self.start as usize..self.current as usize].parse::<f32>().unwrap())
		self.add_token(value);
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
		self.source.chars().nth(self.current as usize).unwrap()
	}

	// Creates new token for character
	fn add_token(&mut self,_type: TokenType) {
		let text = &self.source[self.start as usize..self.current as usize];
		self.tokens.push(Token{_type, lexeme: text.to_string(), line: self.line});
	}
}

