# Clip

A toy programming language for me to learn and explore compilers/the compilation process/compiler internals/etc.

## Installing

1. Clone the repo
2. `cargo install`
3. `cargo build -r` (optional: this will build to `./target/release`)

## Using

You can run the interpreter via `cargo run -- run <file>` or start the REPL with just `cargo run -- repl`.

## Syntax

The language can be best described as lisp without the parentheses, everything is declared and read left to right. Comments are declared using `#` unlike lisp — semicolons actually have meaning (they are delimiters, as they should be).

### Variables

Variables can be assigned and reassigned using `=`:

```
= foo 24
= foo "bar"
```

### Data types

There are primitive data types such as integers, floats, strings and booleans as per usual. However, there is no _explicit_ `null`. Instead, `null` is represented via an empty expression `()` (also known as "unit" in some actual languages).

### Operators

> **Note**
> Operators will only compare arguments of the same type, except for `==` with `()`.

| Definition     | Description                                                                       |
| -------------- | --------------------------------------------------------------------------------- |
| `&& a b ...`   | Logic And: checks if all arguments are _truthy_.                                  |
| `\|\| a b ...` | Logic Or: checks if at least one argument is _truthy_.                            |
| `== a b ...`   | Equality: checks if `a` is equal to any of the other arguments.                   |
| `> a b ...`    | Comparison: checks if `a` is greater than any of the other arguments.             |
| `>= a b ...`   | Comparison: checks if `a` is greater than or equal to any of the other arguments. |
| `< a b ...`    | Comparison: checks if `a` is less than any of the other arguments.                |
| `<= a b ...`   | Comparison: checks if `a` is less than or equal to any of the other arguments.    |
| `+ a b ...`    | Addition: adds all the arguments sequentially.                                    |
| `- a b ...`    | Subtraction: subtracts all the arguments sequentially.                            |
| `* a b ...`    | Multiplication: multiplies all the arguments sequentially.                        |
| `/ a b ...`    | Division: divides all the arguments sequentially.                                 |
| `! a`          | Inverse: gets the inverse value of `a`. Only works for boolean values.            |

### Functions

Functions can be declared using braces. A function's return type is inferred from last expression in the function block. To call a function, simply specify provide the arguments after the function name. If the function doesn't take any arguments, it can be called with `()`.

```
= random { 42 }
random () # integer : 42
```

Function parameters can be defined in brackets following the opening brace:

```
= add { [a b] + a b }
add 2 3 # integer : 5
```

Note that calling a function that has a singular argument with `()` still works:

```
= is_null { [a] == a () }
is_null () # boolean : true
```

## Control Flow

Control flow in the form of `if` and `else` statements is possible (chained `else-if` statements coming soon). Here's fibonacci (it actually works):

```
= fib { [n]
    if < n 2 {
        1
    } else {
        + (fib - n 2) (fib - n 1)
    }
}

fib 12 # integer : 233
```

## Development

- Control statements (`for`)
- Module management (`import`, `export`)
- Method calls
- Data types (`object`, `init`)
- Function type signatures
- Error management (`error`, `catch`)
- Null safety
- Separation of interpretation and compilation

This repository is managed under the Mozilla Public License v2.

© 2023 devnote-dev
