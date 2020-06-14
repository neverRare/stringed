# Stringed Computation

## Boolean Operation

Boolean value can be encoded in string with `true` or `false`, this is exactly what equal operation returns. But, we could use `1` or `0` instead, it is shorter and readily usable with slice.

The following expression converts `true` or `false` to `1` or `0` respectively.

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
