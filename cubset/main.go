package main

import (
	"fmt"
	"os"
	"regexp"
)

type TokenType int

const (
	EOF TokenType = iota
	Print
	Put
	While
	If
	Else
	LeftParen
	RightParen
	StringLiteral
)

type TokenMatch struct {
	tokenType TokenType
	regex     *regexp.Regexp
}

type Token struct {
	tokenType TokenType
	val     string
}

var TokenMatches = []TokenMatch{
	{Print, regexp.MustCompile(`\bprint\b`)},
	{Put,   regexp.MustCompile(`\bput\b`)},
	{While,  regexp.MustCompile(`\bwhile\b`)},
	{StringLiteral, regexp.MustCompile(`"[^"\\]*(\\.[^"\\]*)*"`)},
	{LeftParen, regexp.MustCompile(`\(`)},
	{RightParen, regexp.MustCompile(`\)`)},
	{If, regexp.MustCompile(`if`)},
}


func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	content, err := os.ReadFile("tests/hello_world.txt")
	check(err)

	data := string(content)

	re := regexp.MustCompile(`\bprint\b`)
	result := re.FindStringIndex(string(content))

	fmt.Println("found print: ", result)

	var res[]Token

	for {
		// match on the first index 0
		for _, tokenMatch := range TokenMatches {
			m := tokenMatch.regex.FindStringIndex(data)
			if m != nil {
				if m[0] != 0 {
					continue
				}
				res = append(res, Token{tokenMatch.tokenType, ""})
			}
			break
		}
		// chop by size of string matched
		// create token here
		// if we dont find a match then we error
		// keep chopping until EOF, append eof
		break
	}
	fmt.Printf("%v", res)
}
