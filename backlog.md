# Backlog

## 30/11/2020

My plan is to develop a proof-of-concept implementation as soon as possible. The parser will generate an heap-allocated
AST tree and the interpreter will visit this tree. This is not the most performant implementation possible but it should
do for a while and it should allow me to evaluate the design of the language.

After that I'd like to further develop the project into a bytecode VM that should hopefully be faster. I'm not 100% sure
I'll have the skills to do that but I will try. I'm not yet sure whether I want to parse into an AST tree and then emit
the bytecode or emit the bytecode right at the moment of parsing, without generating any tree.

For parsing I'm using a mixture of Pratt Parsing for expressions and Recursive Descent Parsing for statements.

So far Ampere has got a working lexer and can parse expressions that don't involve commands, property access, function
calls or array indexing. The string interpolation seems to be working. Also we have if statements and a temporary
print statement. The interpreter can evaluate the expressions described above and execute all statements.
