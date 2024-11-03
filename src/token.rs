#[derive(Debug)]
pub enum TokenType {
	Identifier,

	// Values
	Number,
	String,

	// Keywords
	Return,

	// Special Characters
	LeftParen,
	RightParen,
	LeftCurly,
	RightCurly,
	SemiColon,
}

#[derive(Debug)]
pub struct Token {
	pub tok_type: TokenType,
	pub value: String,
}