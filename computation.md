# Stringed Computation

## Boolean Operation

Boolean value can be encoded in string with `true` or `false`, this is exactly what equal operation returns. But, we could use `1` or `0` instead, it is shorter and readily usable with slice.

The following expression converts `true` or `false` to `1` or `0` respectively. Assuming `_` is the operand, you may want to use closure here.

```txt
"----10"[#_:][:"1"]
```

And the following does the opposite.

```txt
$ {"falsetrue"[} + ":"[_:] + {"5"} + ":"[:_] + {]}
```

The following are few boolean operation, accepting `1` or `0` and returning `1` or `0`, for "and" and "or" operations, it accepts two digits of `1` or `0`.

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

TODO more explanation

## Control Flow

Stringed have eval, and we can manipulate the string before it gets evaluated.

The following accepts a single boolean digit and the string of code. inputing `"1" + {"thing"}` would return `thing`, and `"0" + {"thing"}` return empty string.

```txt
$ $ {({""} + } + "{" + _["1":] + "}" + {)[} + ":"[_[:"1"]:] + {"2"} + ":"[:_[:"1"]] + {]}
```

TODO more explanation

## Recusion

The following is the key for recursion.

```txt
{code}: $ "{" + _ + "}: " + _
```

Basically, the `_` in the code is also the code in string, we can eval this and therefore achieve recursion, we may preferably conditionally eval it to avoid infinite recursion.

TODO more explanation
