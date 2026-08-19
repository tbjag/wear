package main

import "core:fmt"
import r "core:text/regex"

Pattern :: struct {
	kind:    Token_Kind,
	pattern: string,
}

Token_Match :: struct {
	kind:  Token_Kind,
	regex: r.Regular_Expression,
}

@(rodata)
JUNK_PATTERNS := []string{`^\s+`, `^/\*[\s\S]*?\*/`}

@(rodata)
KEYWORD_PATTERNS := []Pattern {
	{.Print, `^\bprint\b`},
	{.Put, `^\bputc\b`},
	{.While, `^\bwhile\b`},
	{.If, `^if`},
	{.Else, `^else`},
	{.Equal, `^==`},
	{.NotEqual, `^!=`},
	{.LessEqual, `^<=`},
	{.GreaterEqual, `^>=`},
	{.And, `^&&`},
	{.Or, `^\|\|`},
	{.Greater, `^>`},
	{.Less, `^<`},
	{.Not, `^!`},
	{.Multiply, `^\*`},
	{.Divide, `^/`},
	{.Mod, `^%`},
	{.Add, `^\+`},
	{.Subtract, `^-`},
	{.Semicolon, `^;`},
	{.Assign, `^=`},
	{.Comma, `^,`},
	{.LeftParen, `^\(`},
	{.RightParen, `^\)`},
	{.LeftBrace, `^\{`},
	{.RightBrace, `^\}`},
}

@(rodata)
VARIABLE_PATTERNS := []Pattern {
	{.String, `^"[^"\\]*(\\.[^"\\]*)*"`},
	{.Integer, `^[0-9]+`},
	{.Character, `^'([^'\n]|\\n|\\\\)'`},
	{.Identifier, `^[_a-zA-Z][_a-zA-Z0-9]*`},
}

@(private = "file")
junk_matches: []r.Regular_Expression
@(private = "file")
keyword_matches: []Token_Match
@(private = "file")
variable_matches: []Token_Match
@(private = "file")
capture: r.Capture

compile_patterns :: proc() {
	compile :: proc(pattern: string) -> r.Regular_Expression {
		regex, err := r.create(pattern)
		fmt.assertf(err == nil, "bad pattern %q: %v", pattern, err)
		return regex
	}

	junk_matches = make([]r.Regular_Expression, len(JUNK_PATTERNS))
	for pattern, i in JUNK_PATTERNS {
		junk_matches[i] = compile(pattern)
	}

	keyword_matches = make([]Token_Match, len(KEYWORD_PATTERNS))
	for p, i in KEYWORD_PATTERNS {
		keyword_matches[i] = {p.kind, compile(p.pattern)}
	}

	variable_matches = make([]Token_Match, len(VARIABLE_PATTERNS))
	for p, i in VARIABLE_PATTERNS {
		variable_matches[i] = {p.kind, compile(p.pattern)}
	}

	capture = r.preallocate_capture()
}

// match_at reports the end index of an anchored match beginning at `idx`.
match_at :: proc(regex: r.Regular_Expression, data: string, idx: int) -> (int, bool) {
	if _, matched := r.match(regex, data[idx:], &capture); !matched {
		return idx, false
	}
	return idx + capture.pos[0][1], true
}

eat_junk :: proc(data: string, idx: int) -> (int, bool) {
	for regex in junk_matches {
		if end, ok := match_at(regex, data, idx); ok {
			return end, true
		}
	}
	return idx, false
}

check_for_keyword :: proc(data: string, idx: int) -> (int, Token, bool) {
	for tm in keyword_matches {
		if end, ok := match_at(tm.regex, data, idx); ok {
			return end, Token{kind = tm.kind}, true
		}
	}
	return idx, {}, false
}

check_for_variable :: proc(data: string, idx: int) -> (int, Token, bool) {
	for tm in variable_matches {
		if end, ok := match_at(tm.regex, data, idx); ok {
			return end, Token{kind = tm.kind, value = data[idx:end]}, true
		}
	}
	return idx, {}, false
}

lex :: proc(content: string, allocator := context.allocator) -> []Token {
	if keyword_matches == nil {
		compile_patterns()
	}

	res := make([dynamic]Token, allocator)
	idx := 0

	for {
		if end, ok := eat_junk(content, idx); ok {
			idx = end
			continue
		}

		if end, token, ok := check_for_keyword(content, idx); ok {
			idx = end
			append(&res, token)
			continue
		}

		if end, token, ok := check_for_variable(content, idx); ok {
			idx = end
			append(&res, token)
			continue
		}

		if idx >= len(content) {
			append(&res, Token{kind = .EOF})
			break
		}

		fmt.panicf("Could not parse: %s", content[idx:])
	}

	return res[:]
}

// Frees the lazily-compiled pattern tables. Only needed to keep leak
// trackers quiet; the process would reclaim them anyway.
lexer_destroy :: proc() {
	for regex in junk_matches {
		r.destroy(regex)
	}
	for tm in keyword_matches {
		r.destroy(tm.regex)
	}
	for tm in variable_matches {
		r.destroy(tm.regex)
	}
	delete(junk_matches)
	delete(keyword_matches)
	delete(variable_matches)
	r.destroy(capture)

	junk_matches = nil
	keyword_matches = nil
	variable_matches = nil
	capture = {}
}
