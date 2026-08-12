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
	data, err := os.ReadFile("tests/hello_world.txt")
	check(err)

	fmt.Printf("file size %d\n", len(data))
	fmt.Printf("file content : %s\n", data)
}