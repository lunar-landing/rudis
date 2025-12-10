# SDIFF

Returns the difference between the first set and all successive sets. The difference consists of elements that are in the first set but not in any of the subsequent sets.

## Syntax

```
SDIFF key [key ...]
```

## Return

Array reply: The resulting set of differences. If the first set is empty or does not exist, an empty array is returned.

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
redis> SDIFF myset1 myset2
1) "a"
2) "b"
redis> SDIFF myset2 myset1
1) "d"
2) "e"
```