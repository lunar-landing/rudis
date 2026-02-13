# DECR

Decrements the number stored at the key by one. If the key does not exist, it is set to 0 before performing the operation. Returns an error if the key contains a value that is not an integer or out of range.

## Syntax

```
DECR key
```

## Return

Integer reply: The value of the key after the decrement.

## Examples

```
redis> SET mykey "10"
OK
redis> DECR mykey
(integer) 9
redis> SET mykey "234293482390480948029348230948"
OK
redis> DECR mykey
ERR value is not an integer or out of range
```
