# Stringed

A simple esolang with string as the only data type.

## Installation

For now, the source code isn't complete, nothing will work, but if you insist, you'll need git and cargo.

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

TODO :)

## What can it do

This section is pretty much in WIP, I'm hoping that this language is capable, and maybe it is: with slice operation, we could decode compound data encoded in string; with eval, we could have first-class function in string. Is this turing complete? I have no idea.

For now, I'll write the interpretter.
