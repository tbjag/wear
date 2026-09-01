package main

import (
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	allArgs := os.Args
	if len(allArgs) < 2 {
		fmt.Printf("provide file to lex")
		return
	}
	fileContent, err := os.ReadFile(allArgs[1])
	check(err)
	rawData := string(fileContent)

	fmt.Println("Content: \n", rawData)

	res := lex(rawData)

	for _, item := range res {
		fmt.Printf("%+v\n", item)
	}

}
