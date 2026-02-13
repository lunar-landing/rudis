# SINTER

Returns the intersection of all given sets.

## Syntax

```
SINTER key [key ...]
```

## Return

Array reply: The result set of the intersection. Returns an empty array if one of the sets is empty or does not exist.

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
redis> SINTER myset1 myset2
1) "c"
```
