# Stringed Computation

This doc explains high-level computation on Stringed. It assumes you know syntax and semantics of Stringed and how it is executed, as well as few programming concepts. I am a noob so I can't gurantee the correctness of this doc.

## Boolean Operation

Boolean value can be encoded in string with `true` or `false`, this is exactly what equal operation returns. But, we could use `1` or `0` instead, it is shorter and readily usable with slice.

Let's call `true` and `false` stringified bollean and `1` and `0` boolean digit.

The following expression converts stringified boolean to boolean digit. Assuming `_` is the operand, you may want to use closure here.

```txt
"----10"[#_:][:"1"]
```

And the following does the opposite.

```txt
$ {"falsetrue"[} + ":"[_:] + {"5"} + ":"[:_] + {]}
```

The following are few boolean operation, accepting and returning boolean digit. For "and" and "or" operations, it accepts two digits of it, you may need to concat it.

```txt
NOT
"10"[_:][:"1"]

AND
("0" + _["1":])[_[:"1"]:][:"1"]

OR
(_["1":] + "1")[_[:"1"]:][:"1"]
```

## Arithmetic Operation

On surface, stringed have highly limited arithmetic operations. It have length operator that finds length of string and return a number in string format, lets call this stringified number, that format is usable with slice.

We could simulate arithmetic operation by manipulating the length of string, addition would be concat and subtraction would be slice (underflow causes error, I might relax the restriction of slice in the future), and other than that, thats pretty much the only arithmetic operation in stringed.

Additionally, we can't yet convert stringified number to a string with that length.

## String Operation

Well, duh.

## Compounded data

Stringed only have 1 variable name `_`, there can be multiple variables but it shadows any variables on higher scope. We may need to store many data in single string.

TODO

## Control Flow

Stringed have eval, and we can manipulate the string before it gets evaluated.

The following accepts a single boolean digit and the string of code. inputing `"1" + {"thing"}` would return `thing`, and `"0" + {"thing"}` return empty string.

```txt
$ $ "{" + {""} + _["1":] + "}[" + ":"[_[:"1"]:] + {"2"} + ":"[:_[:"1"]] + "]"
```

TODO more explanation

## Recusion

The following is the key for recursion.

```txt
$ "_: " + _
```

TODO
