package main

import "core:os"
import "core:testing"

Lex_Case :: struct {
	name: string,
	want: []Token_Kind,
}

@(test)
test_basic_files :: proc(t: ^testing.T) {
	defer lexer_destroy()

	cases := []Lex_Case {
		{"tests/hello_world.txt", {.Print, .LeftParen, .String, .RightParen, .Semicolon, .EOF}},
		{
			"tests/variables.txt",
			{
				.Identifier,
				.Assign,
				.Integer,
				.Semicolon,
				.Print,
				.LeftParen,
				.Identifier,
				.Comma,
				.String,
				.RightParen,
				.Semicolon,
				.EOF,
			},
		},
		{
			"tests/all_lex.txt",
			{
				.Print,
				.Subtract,
				.Put,
				.Less,
				.If,
				.Greater,
				.Else,
				.LessEqual,
				.While,
				.GreaterEqual,
				.LeftBrace,
				.Equal,
				.RightBrace,
				.NotEqual,
				.LeftParen,
				.And,
				.RightParen,
				.Or,
				.Subtract,
				.Semicolon,
				.Not,
				.Comma,
				.Multiply,
				.Assign,
				.Divide,
				.Integer,
				.Mod,
				.String,
				.Add,
				.Identifier,
				.Character,
				.Character,
				.Character,
				.EOF,
			},
		},
	}

	for c in cases {
		data, read_err := os.read_entire_file_from_path(c.name, context.allocator)
		if !testing.expectf(t, read_err == nil, "could not read %s: %v", c.name, read_err) {
			continue
		}
		defer delete(data)

		res := lex(string(data))
		defer delete(res)

		if !testing.expectf(
			t,
			len(res) == len(c.want),
			"%s: parsed token lengths do not match, got %d want %d",
			c.name,
			len(res),
			len(c.want),
		) {
			continue
		}

		for token, i in res {
			testing.expectf(
				t,
				token.kind == c.want[i],
				"%s: parsed tokens do not match at %d, got %v want %v",
				c.name,
				i,
				token.kind,
				c.want[i],
			)
		}
	}
}
