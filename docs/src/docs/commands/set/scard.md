# SCARD

Returns the number of elements in the set.

## Syntax

```
SCARD key
```

## Return

Integer reply: The number of elements in the set, or 0 if the key does not exist.

## Examples

```
redis> SADD myset "Hello"
(integer) 1
redis> SADD myset "World"
(integer) 1
redis> SCARD myset
(integer) 2
```
