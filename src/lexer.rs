use std::process::exit;

use crate::token::*;

pub struct Lexer {
	pub source: String,
	pub pos: usize,
	pub c: char,
}

impl Lexer {
	pub fn init(source: String) -> Self {
		Self {
			source: source.clone(),
			pos: 0,
			c: source.chars().nth(0).unwrap()
		}
	}

	fn advance(&mut self) {
		self.pos += 1;

		if self.pos < self.source.len() {
			self.c = self.source.chars().nth(self.pos).unwrap();
		}
	}

	fn advance_with(&mut self, tok: Token) -> Token {
		self.advance();
		tok
	}

	fn lex_word(&mut self) -> Token {
		let start = self.pos;
		while self.pos < self.source.len() && (self.c.is_alphanumeric() || self.c == '_') {
			self.advance();
		}

		let value = self.source[start..self.pos].to_string();
		self.pos -= 1;

		match value.as_str() {
			"return" => return Token { tok_type: TokenType::Return, value },

			"int" => return Token { tok_type: TokenType::Type, value },

			_ => return Token { tok_type: TokenType::Identifier, value }
		}
	}

	fn lex_string(&mut self) -> Token {
		self.advance();

		let start = self.pos;
		while self.pos < self.source.len() && self.c != '"' {
			self.advance();
		}
		let value = self.source[start..self.pos].to_string();

		return Token { tok_type: TokenType::String, value }
	}

	fn lex_num(&mut self) -> Token {
		let start = self.pos;
		while self.pos < self.source.len() && self.c.is_numeric() {
			self.advance();
		}

		let value = self.source[start..self.pos].to_string();
		self.pos -= 1;

		return Token { tok_type: TokenType::Number, value }
	}

	pub fn next_token(&mut self) -> Option<Token> {
		let _ = self.source.trim_start();
		
		if self.pos < self.source.len() {
			while self.c.is_whitespace() {
				self.advance();
			}

			if self.c.is_alphabetic() || self.c == '_' {
				let lexed_word = self.lex_word();
				return Some(self.advance_with(lexed_word));
			}

			if self.c.is_numeric() {
				let lexed_num: Token = self.lex_num();
				return Some(self.advance_with(lexed_num));
			}

			match self.c {
				'(' => return Some(self.advance_with(Token {
					tok_type: TokenType::LeftParen,
					value: self.c.to_string(),
				})),
				')' => return Some(self.advance_with(Token {
					tok_type: TokenType::RightParen,
					value: self.c.to_string(),
				})),

				'{' => return Some(self.advance_with(Token {
					tok_type: TokenType::LeftCurly,
					value: self.c.to_string(),
				})),
				'}' => return Some(self.advance_with(Token {
					tok_type: TokenType::RightCurly,
					value: self.c.to_string(),
				})),

				'"' => {
					let lexed_string = self.lex_string();
					return Some(self.advance_with(lexed_string));
				}

				';' => return Some(self.advance_with(Token {
					tok_type: TokenType::SemiColon,
					value: self.c.to_string(),
				})),

				_ => {
					println!("error: unexpected character: `{}`", self.c);
					exit(1);
				}
			}
		} else { return None }
	}
}