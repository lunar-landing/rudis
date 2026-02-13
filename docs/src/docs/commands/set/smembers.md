# SMEMBERS

Returns all elements in the set.

## Syntax

```
SMEMBERS key
```

## Return

Array reply: All elements in the set, or an empty array if the key does not exist.

## Examples

```
redis> SADD myset "Hello"
(integer) 1
redis> SADD myset "World"
(integer) 1
redis> SMEMBERS myset
1) "Hello"
2) "World"
```
