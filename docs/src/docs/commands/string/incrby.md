# INCRBY

Increments the number stored at the key by the specified integer. If the key does not exist, it is set to 0 before performing the operation. Returns an error if the key contains a value that is not an integer or out of range.

## Syntax

```
INCRBY key increment
```

## Return

Integer reply: The value of the key after the increment.

## Examples

```
redis> SET mykey "10"
OK
redis> INCRBY mykey 5
(integer) 15
redis> SET mykey "234293482390480948029348230948"
OK
redis> INCRBY mykey 5
ERR value is not an integer or out of range
```
