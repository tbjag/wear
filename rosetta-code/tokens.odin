package main

Token_Kind :: enum {
	// Special
	EOF,

	// Keywords
	Print,
	Put,
	While,
	If,
	Else,

	// Literals & Identifiers
	Identifier,
	Integer,
	String,
	Character,

	// Operators
	Assign,
	Add,
	Subtract,
	Multiply,
	Divide,
	Mod,
	Negate,
	Not,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	Equal,
	NotEqual,
	And,
	Or,

	// Delimiters
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Semicolon,
}


Token :: struct {
	kind:  Token_Kind,
	value: string,
	line:  int,
}
