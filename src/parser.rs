use crate::{token::{Token, TokenType}, Lexer};

#[derive(Debug)]
pub enum NumType {
	Int(i32),
}

#[derive(Debug, Clone)]
pub enum Type {
	VOID,
	INT,
}

#[derive(Debug)]
pub enum Expr {
	Number { val: NumType },
	
	VarRef { var_type: Type },
	Call {
		name: String,
		param: Option<Vec<crate::Expr>>,
	},

	Return {
		r_expr: Box<crate::Expr>,
	},
}

#[derive(Debug)]
pub struct FunctionExpr {
	pub name: String,
	pub param: Option<Vec<Expr>>,
	pub ret_type: Type,
	pub body: Option<Vec<Expr>>,
}

pub struct Parser {
	pub lexer: Lexer,
}

impl Parser {
	pub fn init(lexer: Lexer) -> Self {
		Self { lexer }
	}

	fn expect_tok(&mut self, tok_type: TokenType) -> Option<Token> {
		let token = self.lexer.next_token();
		if token.is_none() {
			eprintln!("error: expected {:?} but got eof!", tok_type);
			return None;
		}

		if token.clone().unwrap().tok_type != tok_type {
			eprintln!("error: expected {:?} but got {:?}, info: `{}`", tok_type,
			token.clone().unwrap().tok_type, token.unwrap().value);
			return None;
		}

		token
	}

	fn parse_type(&mut self) -> Option<Type> {
		let typee = self.expect_tok(TokenType::Type);

		match typee.unwrap().value.as_str() {
			"int" => return Some(Type::INT),
			"void" => return Some(Type::VOID),

			_ => {
				return None;
			}
		}
	}

	fn parse_expr(&mut self, ntype: Option<Type>) -> Option<Expr> {
		let mut tok = self.lexer.next_token();

		match tok.clone().unwrap().tok_type {
			TokenType::Number => {
				match ntype.unwrap() {
					Type::INT => {
						return Some(Expr::Number {
							val: NumType::Int(tok.unwrap().value.parse::<i32>().unwrap())
						});
					}

					_ => {}
				}
			},

			_ => {}
		}

		return None;
	}

	fn parse_block(&mut self, rtype: Option<Type>) -> Option<Vec<Expr>> {
		if self.expect_tok(TokenType::LeftCurly).is_none() { return None; }

		let mut exprs = Vec::<Expr>::new();

		let mut tok = self.lexer.next_token();
		while tok.clone().is_some() && tok.clone().unwrap().tok_type != TokenType::RightCurly {
			match tok.clone().unwrap().tok_type {
				TokenType::Return => {
					exprs.push(Expr::Return { r_expr: Box::new(self.parse_expr(rtype.clone()).unwrap()) });
					self.expect_tok(TokenType::SemiColon);
				},

				_ => {
					println!("error: invalid token type, info: {:?}", tok.clone().unwrap().tok_type);
				}
			}

			tok = self.lexer.next_token();
		}

		return Some(exprs);
	}

	pub fn parse_func(&mut self) -> Option<FunctionExpr> {
		let return_type = self.parse_type();
		if return_type.is_none() { return None }

		let name = self.expect_tok(TokenType::Identifier);
		if name.is_none() { return None }

		
		if self.expect_tok(TokenType::LeftParen).is_none() { return None; }
		if self.expect_tok(TokenType::RightParen).is_none() { return None; }

		let body = self.parse_block(return_type.clone());

		return Some(FunctionExpr {
			name: name.unwrap().value,
			param: None,
			ret_type: return_type.unwrap(),
			body: body,
		});
	}
}