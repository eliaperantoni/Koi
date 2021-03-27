# Koi

## Primitive types

The `nil` type (which is also the literal) is used to represent the absence of a value.

Numbers in Koi have the `num` type. There's no distinction between integers and floating points. In places where an integer is expected by the language, a check is performed to assert that the decimal part is 0, otherwise an error is thrown.

```
5
3.14
.77
```

Koi offers 6 different operators for numbers: sum, difference, multiplication, division, power and modulo:

```
6 + 4 # 10
7 - 2 # 9
5 * 4 # 20
9 / 2 # 4.5
2 ^ 4 # 16
8 % 5 # 3
```

Comparison is as usual:

```
4 < 6 # true
6 <= 6 # false
44 > 5 # true
3 >= 1 # true
```

Booleans have the `bool` type. Values are created using the `true` and `false` literals.

Available operators are: conjunction, disjunction and negation:

```
true && false # false
false || true # true
!true         # false
```

All values in Koi can be coerced to a boolean by calling `.bool()` on them. All values are truthy expect `nil` and `false`.

Values can be compared for equality using `==`. Values of different types are never equal:

```
10 == 5 * 2        # true
true == 15         # false
true == 15.bool()  # true
```

## Strings

Strings are delimited with either ' or " and can span multiple lines:

```
'Hello World'
"This is Koi"
'I
like
strawberries'
```

The length of a string can be retrieved using `.len()`:

```
print('Koi'.len())
# 3
```

Strings can be interpolated by surrounding expressions with `{}`. To escape the interpolation, prefix the left brace with a `\`.

```
print('My fav number is {3.14}')
# My fav number is 3.14

print("It's friday then, it's saturday, sunday, {'whaaat'.upper()}!?!")
# It's friday then, it's saturday, sunday, WHAAAT!?!

print('\{"they see me rollin": "they hatin"}')
# {"they see me rollin": "they hatin"}
```

Apart from `\{`, other escape sequences are `\'`, `\"`, `\n`, `\r`, `\t` and `\\`. Everything else is left untouched.

Two strings can be concatenated with `+`:

```
print('x' + 'y' + 'z')
# xyz
```

Strings have many different methods to their disposal. Here's a few:

```
print('koi'.upper())
# KOI

print('KOI'.lower())
# koi

print("What you're referring to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux.".contains('Linux'))
# true

print('koi \n  \t  '.strip() == 'koi')
# true

print('21.2'.parseNum() + 2 == 23.2)
# true

print('AAA---BBB---CCC'.split('---'))
# ['AAA', 'BBB', 'CCC']

print('_'.join(['I', 'love', 'Koi']))
# I_love_Koi

print('All dogs are bad bois'.replace('bad', 'good'))
# All dogs are good bois

print('xxx000yyy'.matches('\w\{3}[0-9]+.*'))
# true

print('x0x xKOIx'.find('x([^\s]+)x'))
# [['x0x', '0'], ['xKOIx', 'KOI']]
```

You can take in input from the user using `input('message')` just like you would in Python:

```
let name = input("What's your name?\n").strip()
print('Nice to meet you {name}!')
```

## Variables

New variables are declared with the `let` keyword. If you don't provide an initializer, the variable will be `nil`.

```
let x
# x is nil

let y = 10
# y is 10
```

Koi has blocks and names are lexically scoped.

```
let x = 10

{
    let x = 20
    print(x)
}

print(x)

# Prints
# 20
# 10
```

Assignment to variables is done using `=`:

```
let year = 2020
year = 2021
print(year)
# 2021
```

It can also be combined with many operators:

```
let year = 2020
year += 1
print(year)
# 2021

let name = 'Koi'
name += '!'
print(name)
# Koi!

let n = 2
n ^= 4
print(n)
# 16
```

## Vectors and dictionaries

Vectors are list of values of (potentially) different types.

```
print([1, 2, 3])
print([1  2  3])
print([
    1
    2
    3
])

# They all print
# [1, 2, 3]
```

You can access elements using the traditional `[]` syntax (vectors are 0-indexed) and retrieve the vector's length using `.len()`:

```
let x = [0 0 99 0]
print(x[2])
# 99

print(x.len())
# 4
```

Commas are not required to separate elements most of the time. This will also be true for dictionaries and function calls.

There are some edge cases were the parser might be too greedy and consume an expression that you intended to be two separate expressions instead. In cases like these, commas can be useful.

```
let x = [1 2 3]

print([
    x
    [0]
])
# [1]

print([
    x,
    [0]
])
# [[1, 2, 3], [0]]
```

Vectors can be sliced using ranges. Ranges represent an interval between two numbers:

```
let fruits = ['pear' 'apple' 'orange']
let good_fruits = fruits[1..fruits.len()]
print(good_fruits)
# ['apple' 'orange']
```

Also note that an `=` can appear in ranges after the dots to make it right-inclusive.

Dictionaries are sets of key-value pairs (also called hash-maps, maps or objects in other languages).

Keys are internally treated as strings but it's allowed to use numbers and they will be rendered to strings automatically.

In dictionary literals string keys can appear without quotes.

Fields are accessed either using the dot-syntax or `[]`.

```
let some_obj = {
    name: 'elia'
    "fav_movie": 'interstellar'
    fav_video: 'https://youtu.be/iik25wqIuFo'
    .5: 3.9
    nested: {x:1 y:2 z:3}
}

print(x.name, x.fav_movie, x[0.5], x.nested.x)
# elia interstellar 3.9 2
```

The number of pairs is retrieved with `.len()`:

```
let songs = [
    {author: 'Hippo Campus' name: 'Vacation'}
    {author: 'Daft Punk'    name: 'Touch'}
]
print(songs.len())
# 2
```

Both vectors and dictionaries act like references to memory but equality is tested by looking at the actual values. This is exactly what happens in JavaScript, Python and alike. You can create a fresh copy of a vector or a dictionary using `.clone()`.

```
let x = ['samir', 'u', 'are', 'breaking', 'the', 'car']
let y = x
y[x.len()-1] = 'nuclear reactor'
print(x)
# ['samir', 'u', 'are', 'breaking', 'the', 'nuclear reactor']

let x = ['who', 'let', 'the', 'dogs', 'out']
let y = x.clone()
y[3] = 'cats'
print(x)
# ['who', 'let', 'the', 'dogs', 'out']

let x = [1 2 4 8 16 32]
let y = x.clone()
print(x == y)
# true
```

Let's take a quick look at the methods that can make working with vectors and dictionaries a little easier:

```
let primes = [1 2 3 4 5 7]
primes.remove(3)
print(primes)
# [1, 2, 3, 5, 7]

print(primes.contains(4))
# false

print(primes.map(fn(n) {return n*2}))
# [2, 4, 6, 10, 14]

print(primes.filter(fn(n){return n % 2 == 0}))
# [2]

primes.forEach(fn(n){
    print(n.type())
})
# Prints 'num' 5 times

let db = {
    host: 'localhost'
    port: 5432
    user: 'lisa'
    password: 'dGhlcmUgYXJlIGVhc3RlciBlZ2dz'
}
db.remove('password')

print(db.contains('host'))
# true

# This is similar to JS .toEntries()
let db_vec = db.toVec()
print(db_vec)
# [['host', 'localhost'], ['port', 5432], ['user', 'lisa']]

# This is similar to JS .fromEntries()
print(db_vec.toDict() == db)
# true
```

## JSON

Values in Koi can easily be converted to and from JSON strings:

```
let cars = [
    {make: 'Audi'    model: 'TT'}
    {make: 'McLaren' model: 'F1'}
    {make: 'Ferrari' model: 'F40'}
]
let str = cars.toJson()
print(str)
# [{"make":"Audi","model":"TT"},{"make" etc...

print(str.parseJson() == cars)
# true
```

Note that it won't work if the value is composed of non-serializable values (functions and ranges).

# Ifs and loops

Koi has an if statement and two loops (for and while). They don't require parenthesis to wrap their header but it's mandatory for the body to be a block.

`if` and `while` are pretty straightforward:

```
let name = 'rust'
if name.upper() == 'RUST' {
    print(name)
}

let vec = ['python', 'lua', 'go']
while vec.len() > 0 {
    print(vec.remove(0))
}
```

The `for` loop is can be used to iterate over: a range, a vector or a dictionary. In the first case it expects one and only one iteration variable, the other two require exactly two: the first is the index or the key, the second is the value. If you don't need either one, name the variable `_`. 

```
for i in 0..3 {
    print(i)
}
# 0
# 1
# 2

for i in 0..=3 {
    print(i)
}
# 0
# 1
# 2
# 3

let a_great_song = ['one', 'more', 'time']
for _, str in a_great_song {
    print(str)
}
# one
# more
# time

let movies_stars = {
    cars: 5
    interstellar: 5
    wolf_of_wall_street: 5
}
for movie, stars in movies_stars {
    print(movie stars)
}
# cars 5
# interstellar 5
# wolf_of_wall_street 5
```

`break` and `continue` work as you would expect:

```
while true {
    break
    print('Unreachable code')
}

for i in 0..5 {
    if i % 2 == 0 {continue}
    print(i)
}
# 1
# 3
```

# Functions

Functions are declared using the `fn` keyword. In function calls, commas are optional (just like in vectors and dictionaries literals):

```
fn double(n) {
    return n * 2
}
print(double(8))
# 16

fn happy_birthday(name, age) {
    print('Happy {age}th birthday {name}!')
}
happy_birthday(
    'Micheal'
    25
)
# Happy 25th birthday Micheal!
```

We've already looked at a good number of methods: they are essentially functions provided by the language runtime and accessed from values.

```
let str = 'I could really use a coffee right now'
print(str.replace('coffee', 'fruit juice'))
# I could really use a fruit juice right now
```

Calling a method actually happens in two steps: first the native function is copied and bound to the value the method was accessed from, then it is called like a regular function.

This means that methods can be accessed and saved for later. They will remember the object they belong to:

```
let str = 'Radiator Springs'
let str_f = str.upper

print(str_f())
# RADIATOR SPRINGS
```

Lambdas are expressions that evaluate to a new function. They use the same exact syntax as regular functions except they don't have a name.

```
let square = fn(n) {
    return n^2
}

print(square(8))
# 64
```

Closures are functions that capture the environment they're declared in. They can continue to access this environment even after it falls out of scope.

```
fn make_counter() {
    let i = 0
    return fn() {
        i += 1
        print(i)
    }
}

let bump = make_counter()
bump()
# 1
bump()
# 2
bump()
# 3
```

## Commands

Commands like those you would write in a shell prompt are valid statements in Koi. No need to mark or prefix them in any way.

Code meant to be interpreted and code meant to execute child processes on the operating system can be interleaved freely:

```
let cond = 2 + 2 == 4
if cond {
    ls -l
}

docker ps -aq
```

Command statements, as we shall call them from now on, inherit the interpreter's standard output and standard error. Therefore their output will appear in real time as they're running.

It might be useful to know how the parser distinguishes what is a command and what is interpretable code: whenever it begins to parse a new line, it looks for keywords that start a statement (such as `if`, `for`, `let`). If it can't find one, then it probes for an expression prefix on the same line. An expression prefix is a series of `identifier.` until one of `[`,`(` or `=` (or one of its variants) is found. If the parser encounters an error or no prefix is found that matches this pattern, the whole line is parsed as a command.

Let's look at some examples:

```
# OK Treated as epxressions

x = 10

obj.func(
    'Koi'
)

# BAD Prefix must appear on the same line

x
= 10

obj.func
(
    'Koi'
)
```

If a command is also valid interpretable code and is being parsed as such, you can force Koi to parse it as a command by prefixing it with `$`.

```
echo = 10
# ERROR Undefined variable 'echo'

$ echo = 10
# Prints '= 10'
```

Now that we're talking about parsing, it might be a good time to explain that Koi has no semicolons at all, not even optional ones. Statements require no syntax to be separated from one another:

```
let a = 1 let b = 2 print(a, b) let c = 3 print(c)
```

Note that once a line begins with interpretable code, the parser will automatically assume that everything else that follows on the same line is also interpretable code. To force it to parse a command you have to use `$`.

```
let x = 10 $ echo {x}
```

Sometimes it might be useful to span a command on multiple lines for clarity. To do that, wrap the command in `$()` and you won't need to escape every single newline like you would in Bash.

```
print('Creating container')

$(
    docker
    run
    --name db
    -p 5432:5432
    --rm
    postgres
)

print('Done')
```

The same syntax can be used to create command expressions. As the name suggests, they allow you to insert commands where an expression is expected. Evaluating the expression will launch the command, wait for it to finish and resolve to its output that was being captured.

```
let me = $(whoami).strip()

for _, id in $(docker ps -aq).strip().split('\n') {
    print(id)
}
```

Commands are parsed like lists of strings with optional quotes. Therefore, command literals inherit all that is available to strings, such as interpolation and escape sequences:

```
let os = $(uname).strip()
echo -e You are running:\n{os}
```

Arguments are split on spaces that appear outside of quotes or double quotes in the source code.

We'll demonstrate this using `argtest`, a little C program that shows us what `argv` looks like:

```
argtest "Welcome to the jungle"
# [0] -> argtest
# [1] -> Welcome to the jungle
```

The spaces appear in the source code but inside double quotes. Therefore no splitting is done.

If we remove them we get many arguments:

```
argtest Welcome to the jungle
# [0] -> argtest
# [1] -> Welcome
# [2] -> to
# [3] -> the
# [4] -> jungle
```

Interpolating a string that includes spaces will not split arguments like it would in Bash. No need to wrap everything in quotes:

```
let str = "We're up all night to get lucky"
argtest {str}
# [0] -> argtest
# [1] -> We're up all night to get lucky
```

Multiple strings (quoted or not) can appear adjacent to one another and they will  be part of the same argument:

```
let what = 'awesome'
argtest "Koi_"is"_{what}!"
# [0] -> argtest
# [1] -> Koi_is_awesome!
```

Remember: splitting is only performed when a space appears unquoted in the source code.

String interpolation works a little bit differently in commands when compared to strings. If the expression evaluates to an array, a cross product is performed and multiple arguments are produced:

```
let files = ['passwd', 'group']

print("The files are: {files}")
# The files are: ['passwd', 'group']

argtest /etc/{files}
# [0] -> argtest
# [1] -> /etc/passwd
# [2] -> /etc/group
```

When Koi starts, all environment variables are declared in the global scope:

```
let path = PATH.split(':')

print(USER)
```

To declare a new exported variable use `exp`. All variables declared with `exp` that are in scope of a command statement or expression will be part of the child process' environment:

```
exp let MYVAR = 'Koi'
python -c 'import os; print(os.environ["MYVAR"])'
# Koi
```

Commands can be composed by piping, conditionally chaining or redirecting them.

Pipes forward the output of a command to the input of the next one. The whole pipeline acts as a single command. You can choose to pipe only standard output `|`, standard error `*|` or both `&|`.

```
echo -n 'What a nice day' | sed 's/nice/beautiful/g'
# What a beautiful day

touch /.. *| tee errors.log
```

Chaining is done using `&&`, `||` and `;` and conditionally executes the second command after the first has terminated. The whole chain is treated as a command and the two subcommands' standard streams are joined together.

Parenthesis can be used to override the default precedence and associativity rules.

```
echo This && echo Is && echo Koi
# This
# Is
# Koi

(echo -n S29 && echo -n pCg==) | base64 -d
# Koi
```

`&&` only executes the second command if the first terminated with a zero return code. `||` only executes the second command if the first failed. `;` always executes the second command no matter what.

```
python -c 'exit(1)' && echo "I don't get printed"
python -c 'exit(1)' || echo "I do"
python -c 'import random; exit(random.randint(0,1))' ; echo "I do"
```

Redirection forwards the standard output, standard error or both of a process to a file by overwriting or by appending.

`>` is for overwriting while `>>` is for appending. Both default to forwarding only standard output but their behavior can be changed by prefixing them with `*` and `&` just like with pipes.

```
docker ps > containers.txt
toch /.. *>> errors.log
```

Finally, redirection can also be used to forward the contents of a file to the standard input of a process:

```
head -n 4 < src/main.rs
```

## Command line

`koi` takes in the path to a Koi source file to run. Alternatively the `-s` flag can be set to read the source from standard input.

```
$ echo 'print("Koi")' | koi -s
# Koi
```

The `-f` argument allows you to specify the name of a function declared in the global scope. Koi will execute the source file and then call the function with no arguments.

This is useful for writing automation scripts that provide a series of tasks that can be invoked from the terminal. Similar to makefiles or gulpfiles.

```
fn build() {
    print("BUILDING")
    gcc main.c -o main
}

fn clean() {
    print("CLEANING")
    rm main
}
```

```
$ koi script.koi -f clean
# CLEANING
```

If no path is provided to `koi`, a default value of `Koifile` will be used.

This means that you can write your automation tasks in a `Koifile` and run them very easily like:

```
$ koi -f build
# BUILDING
```

