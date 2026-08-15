package main

import (
	"fmt"
	"os"
	"regexp"
)

type TokenType string

const (
	EOF           TokenType = "EOF"
	Print         TokenType = "Print"
	Put           TokenType = "Put"
	While         TokenType = "While"
	If            TokenType = "If"
	Else          TokenType = "Else"
	LeftParen     TokenType = "LeftParen"
	RightParen    TokenType = "RightParen"
	StringLiteral TokenType = "StringLiteral"
	Semicolon     TokenType = "Semicolon"
)

type TokenMatch struct {
	tokenType TokenType // i think we add a function
	regex     *regexp.Regexp
}

type TokenMatch2 struct {
	tokenType TokenType // i think we add a function
	size int
	
	regex     *regexp.Regexp
}

type Token struct {
	Type TokenType
	Val  string
}

var TokenMatches = []TokenMatch{
	{Print, regexp.MustCompile(`\bprint\b`)},
	{Put, regexp.MustCompile(`\bput\b`)},
	{While, regexp.MustCompile(`\bwhile\b`)},
	{StringLiteral, regexp.MustCompile(`"[^"\\]*(\\.[^"\\]*)*"`)},
	{LeftParen, regexp.MustCompile(`\(`)},
	{RightParen, regexp.MustCompile(`\)`)},
	{If, regexp.MustCompile(`if`)},
	{Else, regexp.MustCompile(`else`)},
	{Semicolon, regexp.MustCompile(`;`)},
}

func clearWhitespace(reg *regexp.Regexp, idx int, s string) int {
	loc := reg.FindStringIndex(s[idx:])
	if len(loc) == 2 {
		if loc[0] != 0 {
			return idx
		}
		idx += loc[1]
	}
	return idx
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	fileContent, err := os.ReadFile("tests/hello_world.txt")
	check(err)
	rawData := string(fileContent)
	rawDataLen := len(rawData)

	var idx int
	var res []Token
	whiteSpaceReg := regexp.MustCompile(`\s+`)

	// custom parse function for each struct in list?
	// would be nice to declare (enum val, regex, size of token, parse logic?, parse order) 
	// in one place
	for {
		match := false
		idx = clearWhitespace(whiteSpaceReg, idx, rawData)
		for _, tokenMatch := range TokenMatches {
			loc := tokenMatch.regex.FindStringIndex(rawData[idx:])
			if len(loc) == 2 {
				if loc[0] != 0 {
					continue
				}
				idx += loc[1]
				res = append(res, Token{tokenMatch.tokenType, ""}) // todo parse string
				match = true
				break
			}
		}

		if !match {
			panic("Could not parse: " + rawData[idx:])
		}

		if idx >= rawDataLen {
			res = append(res, Token{EOF, ""})
			break
		}
	}
	for _, item := range res {
		fmt.Printf("%+v\n", item)
	}

}
