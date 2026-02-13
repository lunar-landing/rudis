# INCR

Increments the number stored at the key by one. If the key does not exist, it is set to 0 before performing the operation. Returns an error if the key contains a value that is not an integer or out of range.

## Syntax

```
INCR key
```

## Return

Integer reply: The value of the key after the increment.

## Examples

```
redis> SET mykey "10"
OK
redis> INCR mykey
(integer) 11
redis> SET mykey "234293482390480948029348230948"
OK
redis> INCR mykey
ERR value is not an integer or out of range
```
