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
