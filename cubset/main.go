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
	fileContent, err := os.ReadFile("tests/hello_world.txt")
	check(err)
	rawData := string(fileContent)

	fmt.Println("Content: \n", rawData)

	res := lex(rawData)

	for _, item := range res {
		fmt.Printf("%+v\n", item)
	}

}
