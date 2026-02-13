# SISMEMBER

Determines whether the specified element exists in the set.

## Syntax

```
SISMEMBER key member
```

## Return

Integer reply: 1 if the element exists in the set, 0 if the element does not exist or the key does not exist.

## Examples

```
redis> SADD myset "Hello"
(integer) 1
redis> SISMEMBER myset "Hello"
(integer) 1
redis> SISMEMBER myset "World"
(integer) 0
```
