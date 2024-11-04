#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
	Identifier,

	// Values
	Number,
	String,

	Type,

	// Keywords
	Return,

	// Special Characters
	LeftParen,
	RightParen,
	LeftCurly,
	RightCurly,
	SemiColon,
}

#[derive(Debug, Clone)]
pub struct Token {
	pub tok_type: TokenType,
	pub value: String,
}