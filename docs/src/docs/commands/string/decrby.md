# DECRBY

Decrements the number stored at the key by the specified integer. If the key does not exist, it is set to 0 before performing the operation. Returns an error if the key contains a value that is not an integer or out of range.

## Syntax

```
DECRBY key decrement
```

## Return

Integer reply: The value of the key after the decrement.

## Examples

```
redis> SET mykey "10"
OK
redis> DECRBY mykey 3
(integer) 7
redis> SET mykey "234293482390480948029348230948"
OK
redis> DECRBY mykey 3
ERR value is not an integer or out of range
```
