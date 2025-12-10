# SDIFF

返回给定所有集合的差集。差集是指在第一个集合中但不在后续任何集合中的元素。

## Syntax

```
SDIFF key [key ...]
```

## Return

Array reply: 差集的结果集。如果第一个集合为空或不存在，则返回空数组。

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