package main

import (
	"fmt"
	"os"
)

type AstType string
const (
	Sequence AstType = "Sequence"
	AssignA AstType = "Assign"
	IdentifierA AstType = "Identifier"
	IntegerA AstType = "Integer"
)

type Ast struct {
	Type AstType
	Left *Ast
	Right *Ast
}

func expression(idx *int, tokens []Token) *Ast {
	
}

func statement(idx *int, tokens []Token, root *Ast) {
	
	switch curr := tokens[*idx]; curr.Type {
		case Identifier:
			fmt.Println("in identifier")
			identAst := Ast{
				Type: IdentifierA,
				Left: nil,
				Right: nil,
			}
			*idx += 1
			if tokens[*idx].Type != Assign {
				panic("Expected `=` after identifier")
			}
			exprRes := expression(idx, tokens)
			assignAst := Ast{
				Type: AssignA,
				Left: nil,
				Right: nil,
			}
		default:
			fmt.Printf("in default")
	}
	
}

func parse(tokens []Token) *Ast {
	root := Ast{Sequence, nil, nil}
	idx := 0
	for tokens[idx].Type != EOF {
		fmt.Printf("%+v\n", tokens[idx])
		statement(&idx, tokens, &root)
		idx += 1
	}
	
	return &root
}

func main() {
	allArgs := os.Args
	if len(allArgs) < 2 {
		fmt.Printf("provide file to lex")
		return
	}
	fileContent, err := os.ReadFile(allArgs[1])
	if err != nil {
		panic(err)
	}
	rawData := string(fileContent)

	fmt.Println("Content: \n", rawData)

	lexRes := lex(rawData)

	astRes := parse(lexRes)
	// for _, item := range astRes {
	// 	fmt.Printf("%+v\n", item)
	// }

	fmt.Println(astRes.Type)

}
