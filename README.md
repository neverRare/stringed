# Stringed

[![Rust](https://github.com/neverRare/stringed/workflows/Rust/badge.svg)](https://github.com/neverRare/stringed/actions?query=workflow%3ARust)

An esolang with first-class strings.

## Installation

TODO

## Examples

```txt
"Hello world!"
```

```txt
"Please enter your name:
Hello " + ? + "!"
```

```txt
{"loop
" + $ _}: $ "{" + _ + "}: " + _
```

## What it is

- UTF-8 string as the only data type
- have simple IO
- have few operation

## Syntax and Semantics

The following are syntax of Stringed code representing its few operation. The capital letter denotes another Stringed expression and the ellipsis `...` denotes another special syntax.

| Syntax             | Name    |
| ------------------ | ------- |
| `"..."` or `{...}` | Literal |
| `(A)`              | Group   |
| `?`                | Prompt  |
| `_`                | Var     |
| `A:B`              | Closure |
| `A+B`              | Concat  |
| `A[B:C]`           | Slice   |
| `A=B`              | Equal   |
| `#A`               | Length  |
| `$A`               | Eval    |

With the following precedence from highest to lowest.

- Grouping
- Length
- Slice
- Concat
- Equal
- Closure
- Eval

## Literal

String literal are enclosed with either double quotation marks `""` or curly braces `{}`.

It can contain any characters and it doesn't have escaping functionality.

Literals enclosed with `{}` can contain quotation mark or another braces. It can be nested: `{{}}` and `{{{}}{}}` are both valid literal and `{}}` may cause syntax error. It doesn't recognize quotation mark for nesting: `{"{"}` may cause syntax error.

## Basic Operations

Pretty basic, you'll understand it in few examples.

```txt
"concat" + "enation"
"concatenation"

"slice"["2":"4"]
"ic"

"slice"["2":]
"ice"

"slice"[:"4"]
"slic"

"slice"[:]
"slice"

"slice error"["a number":"-10"]
Error: Bound is not convertible to unsigned integer

"slice error"["":"100"]
Error: Upper bound is larger than the length

"slice error"["10":"0"]
Error: Lower bound is larger than upper bound

"equal" = "equal"
"true"

"not equal" = "not really equal"
"false"

#"length"
"6"

#"size"
"4"

("group" + "ings")[:"8"]
"grouping"
```

## Closure and Var

Closure creates a scope in which gives variable `_` a value: It evaluates to the second operand as if the variable is the first operand.

```txt
"apple": "my favorite fruit is " + _
```

Closure is right to left associative and the first operand is evaluated first.

```txt
A:B:C
is evaluated as
A:(B:C)

A is evaluated first, then B:C
```

Closure creates a scope in a way variable can be shadowed.

```txt
"a": "b" + _ + ("nan": _) + _
```

The first operand can even use the variable of outer closure.

```txt
"a": "b" + _ + ("n" + _ + "n": _) + _
```

## Eval

Stringed can immediately evaluate strings as Stringed expression and return a... string. We may need to include a literal inside a literal, this is where `{}` can be useful.

```txt
$ {"evaluation"} + {[:"4"]}
```

Evals can also capture variable.

```txt
"world": $ {"hello " + _}
```

Literals in `{}` doesn't look like literals, nice!

## IO

Stringed have `?`, when evaluated, it ask the user to input then it is evaluated to that value.

```txt
"Hello " + ?
```

Stringed also have output, which is pretty weird. It is explained below.

```txt
"some" + "thing"
```

When not dealing with input, we could think of stringed code as an expression and it output whatever it evaluates to. The example above could be imagined as the following pseudo-code:

```txt
print("some" + "thing")
```

But this is what stringed actually does:

```txt
print("some")
print("thing")
```

Every stringed operation have 2 modes: evaluation and execution. Evaluation means it will be evaluated normally as an operation, it does what explained earlier, and it evaluates to a string. Execution means it is executed, it may output something, and it doesn't evaluates to any value.

| Operation | Execution                                               |
| --------- | ------------------------------------------------------- |
| Group     | It executes any expression inside                       |
| Concat    | It executes each operand one by one                     |
| Closure   | It evaluates its first operand then executes the second |
| Eval      | It evaluates its operand then it is executed            |
| any other | It is evaluated then outputted                          |

Stringed expression on top level are executed.

## Output queue

The stringed interpretter internally have queue for output. Whenever the stringed code outputs a string, it is first goes to queue, then it is only printed on standard output on every newline (LF or CRLF). When the stringed code is done executing, the remaining string on queue is printed as well.
