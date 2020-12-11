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

## 10/12/2020

The syntax for the commands is really slowing me down here. No wonder! It turns out it's quite complicated to get it right.

Anyhow, I'm been looking around to try and get inspiration (see copy) from other attempts at creating a better shell
programming language. I did this before starting this project as well but didn't found anything. This time I tried a bit
harder and here's what I've found:

+ [Xonsh](https://xon.sh)

    This one is actually incredibly well made, as far as I can tell. I think it might become my default shell from now on!.

    It's an extension of Python and has two modes of parsing: python mode and subprocess mode. When Xonsh encounters an 
    expression statement with undefined identifiers, it will try to parse it in subprocess mode, if that fails it will 
    revert back to python mode.
  
    This clever little trick allows one to mix python and command calls with no added syntax:

    ```python
    variable = f"this is normal python and 5*2 = {5 * 2}"
    if variable is not None:
        print("all good down here!")
  
    ls -l
    echo @(variable)
  
    ls = 5
    l = 2
    
    # This is no longer a command because all identifiers
    # can be resolved
    ls -l
    ```
  
    The `@()` is used to interpolate python expressions in command calls. When the expression evaluates to a list, it
    expands to the outer product:
  
    ```python
    echo @(['a', 'b']):@('x', 'y')
    # prints: a:x a:y b:x b:y
    ```

    There are also 4 synctactic elements: `$()`, `!()`, `$[]`, `![]` that force Xonsh into subprocess mode and they are all
    slightly different.

+ [NGS](https://ngs-lang.org)

    Has code syntax and command syntax. You can specify which one is the top level when starting the interpreter.
  
    When in command syntax you can switch to code syntax using `{}` for statements, `${}` to interpolate expressions
    and `$*{}` to interpolate expressions that evaluate to lists.
  
    ```
    ls
    { code syntax here }
    
    ls ${ code that computes the file name and eturns a string,
    spaces don't matter, expanded into single argument of ls }
    
    # Expands to zero or more positional arguments to ls
    ls $*{ code that computes the files names and returns array of
    strings. Spaces don't matter, each element is expanded into
    single argument of ls }
    ```

    When in code syntax you can switch to command syntax using <code>\`\`</code>. It's an expression that executes
    the command and evaluates to its output.
  
    ```
    # Capture output
    out = `commands syntax`
    my_var = "mystring\n" + `my other command`
    
    # Get reference to a process
    my_process = $( commands syntax )
    ```

+ [Shok](http://shok.io)
  
    Also has two modes but command mode is always top level. You can enter a block of code using `{}` which is also used
    to interpolate expressions in command calls. How cool!
  
    ```
    all there is to shok

    { write code in curly braces }
    
    : colon runs commands
    ```
  
    While in code mode you can start a line with `:` to create a command statement. You can also create a block in command
    mode with `{:: ::}`. 
  
    I couldn't find out how to capture a command's output.


+ [Ammonite](http://ammonite.io)

    Very focused on "code mode". You can start a line with `%` to create a command statement. Commands that are not valid
    Scala identifiers (Ammonite is based on Scala) must also be wrapped in backticks <code>%\`\`</code>.
  
    Also you can wrap a command in `%%()` to get an object representing the command.

    ```
    %ls
    %%(ls)
    ```
  
    Doesn't support piping.


## 11/12/2020

Yesterday I got a lot of useful info comparing my design to what others have come up with. I was also relieved to find
that my syntax for commands was not as bad as I thought: some are very similar to mine (Shok) while others (like Ammonite)
are even worse in my opinion.

I was thinking that understanding how to deal with implicit semicolons could help me on settling on a syntax for commands.
It's also a very important aspect of my language anyways and I should look into it ASAP.

I went to read a section from my trusty [Crafting Interpreters](http://craftinginterpreters.com/scanning.html#design-note)
(thank you [Robert](https://journal.stuffwithstuff.com/)) <3) which explains how Go and Python handle that.

Go uses a simple rule: only newlines that come after certain token types are significant. Take a look at this code:

```go
fmt.Println("Hello, World!")
x := 12 * 2
y := fmt.Sprintf("x is %d", x)
go func(){
    fmt.Println(y)
}()
z := 225 - y
for i := 0; i < 10; i++ {
    if i % 2 == 0 {
        continue
    } else if i == 15 {
        break
    }
    z -= i
}
```

Do you notice a pattern on tokens that end a statement? It's always a `)`, an identifier, any literal or some specific
keywords like `break` and `continue`. Go's lexer takes advantage of this and inserts implicit semicolons whenever it
encounters a newline preceded by one of these tokens.

But it works wonders when you want to spread out on multiple lines as well! Take a look at this example in which the code
uses as many lines as possible:

```go
fmt.
    Println(
        "Hello, World!",
    )

ctx :=
    context.
        WithValue(
            context.
                Background(
                ),
            "value",
            12,
        ).
        Err(
        ).
        Error(
        )
```

There are only two statements and they are both function calls (i.e. they both end in `)` and a newline so the lexer will
put an implicit semicolon there).

Can you see how ignored newlines are always preceded by very specific token types? `(` opens the parameters list for a
function call, `:=`/`=` precede the expression in a declaration/assignment, `.` precedes the identifier in an access 
operator, `,` comes after a parameter in a function call.

Not that the last parameter in a function call still has to be followed by a `,` unless the `)` appears on the same line.
This is not valid:

```go
fmt.Printf(
    "2*5 is %d",
    10
)
```

And the reason is pretty simple: the lexer sees `10`, a literal, and a newline that follows. An int literal is one of those
tokens that force newlines that follow to be treated as semicolons.

Similarly, chained function calls must end each line with `.` so that the lexer knows it has to hold on! This is not valid:

```go
context.Background()
       .Err()
```

You can take a look at the Go's lexer's unit tests [here](https://github.com/golang/go/blob/master/src/go/scanner/scanner_test.go#L369)
for more examples.


Indeed if we take a look at Go's lexer's [source code](https://github.com/golang/go/blob/master/src/go/scanner/scanner.go#L782)
we can see it using those patterns we talked about. `Scan()` scans the next token and returns it.
The function immediately calls `skipWhitespace()` which discards any whitespace expect newlines if `insertSemi` is true.
Going back to `Scan()` we see that tokens such as `)` (that allow a following newline to be lexed as an implicit semicolon)
set `insertSemi` to true. The next time `Scan()` is called `skipWhitespace()` will not discard the newline which is going
to be scanned as `SEMICOLON`. Neat!

What about Python? Python treats all newlines as significant (i.e. treats them as semicolons) expect when inside a pair
of `()`, `[]` or `{}`. That's why function calls, arrays and objects can still be spread over multiple lines.

```python
range(
    5,
    100,
    2
)

[
    1,
    2,
    3
]

{
    "a": 1,
    "b": 2,
    "c": 3
}
```

That's also why lambdas cannot contain multiple statements on multiple lines! The newlines, that normally mark the end of
a statement in Python, are ignored in lambdas because they oftern appear inside `()`... :

```python
map(lambda e: e * 2, [1, 2, 3])
```

...and even when they don't, you still cannot use multiple lines because they terminate the outer statement:

```python
fn = lambda e: e = e * 2 # implicit ; here
               return e  # implicit ; here
```

So where do we go from here? No clue yet!

![](https://i.imgflip.com/ry4bq.jpg)

But I think Python and Go were really insightful. TBH I think Python's approach only works because it doesn't have `{}`
blocks which Ampere does. Otherwise how would the lexer know it has to treat newlines in the if statement (with an
hypothetical `{}` block) as semicolons but not in the dict?

```python
if True {
    print("foo")
    print("bar")
}

d = {
    "foo": 1,
    "bar": 2
}
```

In both cases newlines appear inside `{}` but in one case they are meaningful in the other they are not. This cannot work
in Ampere. Go's solution is quite simple and neat. I'll ponder whether or not it's applicable to Ampere.
