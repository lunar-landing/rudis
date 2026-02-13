# SUNIONSTORE

Stores the union of all given sets in the specified destination set. If the destination set already exists, it is overwritten.

## Syntax

```
SUNIONSTORE destination key [key ...]
```

## Return

Integer reply: The number of elements in the resulting set.

## Examples

```
redis> SADD myset1 "a"
(integer) 1
redis> SADD myset1 "b"
(integer) 1
redis> SADD myset1 "c"
(integer) 1
redis> SADD myset2 "c"
(integer) 1
redis> SADD myset2 "d"
(integer) 1
redis> SADD myset2 "e"
(integer) 1
redis> SUNIONSTORE myset3 myset1 myset2
(integer) 5
redis> SMEMBERS myset3
1) "a"
2) "b"
3) "c"
4) "d"
5) "e"
```
