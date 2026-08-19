1. No bounds/EOF guard, no error path. tokens[idx^] can run past end on malformed input. ok from parse_int discarded — bad token silently becomes 0.

2. Precedence ladder doesn't scale. One func per level. Subtract, Divide, Mod, Negate, parens, and all 8 comparison/logic ops in Token_Kind are unhandled. Adding them = 5 more near-identical funcs.

3. import "core:fmt" unused.

4. Token_Kind.Multiply → .Multiply. Odin infers enum from context.

5. File named ast.odin but builds no AST. Direct eval. Either rename eval.odin or return nodes.