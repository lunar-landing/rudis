# INCRBYFLOAT

Increments the floating point number stored at the key by the specified float. If the key does not exist, it is set to 0 before performing the operation. Returns an error if the key contains a value that is not a valid float.

## Syntax

```
INCRBYFLOAT key increment
```

## Return

Bulk string reply: The value of the key after the increment.

## Examples

```
redis> SET mykey "10.50"
OK
redis> INCRBYFLOAT mykey 0.1
"10.6"
redis> INCRBYFLOAT mykey -5
"5.6"
redis> SET mykey "5.0e3"
OK
redis> INCRBYFLOAT mykey 2.0e2
"5200"
```
