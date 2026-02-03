# SINTERSTORE

Computes the intersection of all given sets, and stores the result in the specified set. If the destination set already exists, it will be overwritten.

## Syntax

```
SINTERSTORE destination key [key ...]
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
redis> SADD myset2 "b"
(integer) 1
redis> SADD myset2 "c"
(integer) 1
redis> SADD myset2 "d"
(integer) 1
redis> SINTERSTORE myset3 myset1 myset2
(integer) 2
redis> SMEMBERS myset3
1) "b"
2) "c"
redis> SADD myset1 "d"
(integer) 1
redis> SADD myset2 "e"
(integer) 1
redis> SINTERSTORE myset3 myset1 myset2
(integer) 2
redis> SMEMBERS myset3
1) "b"
2) "c"
```

