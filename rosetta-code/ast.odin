package main

import "core:fmt"
import "core:strconv"
//basic evaluation for fun just to test my brain w/ odin
// Qs
// is pointer the best way to do this?
// can i have tokens as readonly constant?



parse_term :: proc(idx: ^int, tokens: []Token) -> (int) {
	result, ok := strconv.parse_int(tokens[idx^].value)
	idx^ += 1

	for tokens[idx^].kind == Token_Kind.Multiply {
		idx^ += 1
		right, ok := strconv.parse_int(tokens[idx^].value)
		idx^ += 1
		result *= right
	}
	
	return result
}

eval :: proc (idx: ^int, tokens: []Token) -> int {
	result := parse_term(idx, tokens)
	for tokens[idx^].kind == Token_Kind.Add {
		idx^ += 1
		right_side := parse_term(idx, tokens)
		result += right_side
	}
	return result
}
