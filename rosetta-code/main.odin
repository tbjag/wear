package main

import "core:fmt"
import "core:os"

main :: proc() {
	defer lexer_destroy()

	filepath := "tests/hello_world.txt"

	if len(os.args) == 2 {
		filepath = os.args[1]
    }
	
    fmt.printfln("reading from %s", filepath)
	
	data, err := os.read_entire_file_from_path(filepath, context.allocator)
	fmt.assertf(err == nil, "could not read tests/hello_world.txt: %v", err)
	defer delete(data)

	content := string(data)
	fmt.println("\nContent: \n", content)

	res := lex(content)
	defer delete(res)

	fmt.printfln("\nTokens:")
	for token in res {
		fmt.printfln("\t%+v", token)
	}

	fmt.printfln("\nAST:")
	idx := 0
	x := eval(&idx, res)
	fmt.printfln("result: %d", x)
}
