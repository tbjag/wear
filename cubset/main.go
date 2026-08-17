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

type ParseType int

const (
	Keyword ParseType = iota
	String
	Integer
)

type TokenMatch struct {
	TokenType TokenType
	Regex     *regexp.Regexp
	Size      int
}

type Token struct {
	Type TokenType
	Val  string
}

func (t TokenMatch) ParseKeyword(data string, idx int) (int, Token, bool) {
	loc := t.Regex.FindStringIndex(data[idx:])
	token := Token{
		Type: t.TokenType,
		Val:  "",
	}
	if len(loc) != 2 || loc[0] != 0 {
		return idx, token, false
	}
	return idx + loc[1], token, true
}

func (t TokenMatch) ParseVariable(data string, idx int) (int, Token, bool) {
	loc := t.Regex.FindStringIndex(data[idx:])
	token := Token{
		Type: t.TokenType,
		Val:  "",
	}
	if len(loc) != 2 || loc[0] != 0 {
		return idx, token, false
	}
	token.Val = data[idx : idx+loc[1]]
	return idx + loc[1], token, true
}

var JunkMatches = []*regexp.Regexp {
	regexp.MustCompile(`\s+`),
	//todo add comments
}

var KeywordTokenMatches = []TokenMatch{
	{Print, regexp.MustCompile(`\bprint\b`), 5},
	{Put, regexp.MustCompile(`\bput\b`), 3},
	{While, regexp.MustCompile(`\bwhile\b`), 5},
	{LeftParen, regexp.MustCompile(`\(`), 1},
	{RightParen, regexp.MustCompile(`\)`), 1},
	{If, regexp.MustCompile(`if`), 2},
	{Else, regexp.MustCompile(`else`), 4},
	{Semicolon, regexp.MustCompile(`;`), 1},
}

var VariableTokenMatches = []TokenMatch{
	{StringLiteral, regexp.MustCompile(`"[^"\\]*(\\.[^"\\]*)*"`), -1},
}

func checkForKeyword(data string, idx int) (int, Token, bool) {
	match := false
	token := Token{}
	for _, tm := range KeywordTokenMatches {
		if _idx, _token, ok := tm.ParseKeyword(data, idx); ok {
			match = true
			idx = _idx
			token = _token
			break
		}
	}
	return idx, token, match
}

func checkForVariable(data string, idx int) (int, Token, bool) {
	match := false
	token := Token{}
	for _, tm := range VariableTokenMatches {
		if _idx, _token, ok := tm.ParseVariable(data, idx); ok {
			match = true
			idx = _idx
			token = _token
			break
		}
	}
	return idx, token, match
}

func eatJunk(data string, idx int) (int, bool) {
	for _, jm := range JunkMatches {
		loc := jm.FindStringIndex(data[idx:])
		if len(loc) != 2 || loc[0] != 0 {
			continue
		}
		return  idx + loc[1], true
	}
	return idx, false
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

	for {
		match := false

		if _idx, ok := eatJunk(rawData, idx); ok {
			idx = _idx
			continue
		}

		if _idx, token, ok := checkForKeyword(rawData, idx); ok {
			match = true
			idx = _idx
			res = append(res, token)
			continue
		}

		if _idx, token, ok := checkForVariable(rawData, idx); ok {
			match = true
			idx = _idx
			res = append(res, token)
			continue
		}

		if idx >= rawDataLen {
			res = append(res, Token{EOF, ""})
			break
		}

		if !match {
			panic("Could not parse: " + rawData[idx:])
		}

	}
	for _, item := range res {
		fmt.Printf("%+v\n", item)
	}

}
