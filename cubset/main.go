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
	tokenType TokenType
	parseType ParseType
	regex     *regexp.Regexp
	size      int
}

func (t TokenMatch) ParseKeyword(data string, idx int) (int, bool) {
	loc := t.regex.FindStringIndex(data[idx:])
	if len(loc) != 2 || loc[0] != 0 {
		return idx, false
	}
	return idx + loc[1], true
}

type Token struct {
	Type TokenType
	Val  string
}

var TokenMatches = []TokenMatch{
	{Print, Keyword, regexp.MustCompile(`\bprint\b`), 5},
	// {Put, regexp.MustCompile(`\bput\b`)},
	// {While, regexp.MustCompile(`\bwhile\b`)},
	// {StringLiteral, String, regexp.MustCompile(`"[^"\\]*(\\.[^"\\]*)*"`), -1},
	// {LeftParen, Keyword, regexp.MustCompile(`\(`), 1},
	// {RightParen, Keyword, regexp.MustCompile(`\)`), 1},
	// {If, regexp.MustCompile(`if`)},
	// {Else, regexp.MustCompile(`else`)},
	// {Semicolon, Keyword, regexp.MustCompile(`;`), 1},
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

	for {
		match := false
		idx = clearWhitespace(whiteSpaceReg, idx, rawData)
		for _, tokenMatch := range TokenMatches {
			if tokenMatch.parseType == Keyword {
				tokenMatch.ParseKeyword(rawData, idx)
			}

			
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
