# SMOVE

将指定成员从源集合移动到目标集合。如果源集合中不存在该成员，则不执行任何操作。如果目标集合中已存在该成员，则只从源集合中移除，不会在目标集合中重复添加。

## Syntax

```
SMOVE source destination member
```

## Return

Integer reply: 如果成员成功移动，返回 1；如果源集合中不存在该成员，返回 0。

## Examples

```
redis> SADD myset1 "a"
(integer) 1
redis> SADD myset1 "b"
(integer) 1
redis> SADD myset1 "c"
(integer) 1
redis> SADD myset2 "x"
(integer) 1
redis> SADD myset2 "y"
(integer) 1
redis> SMOVE myset1 myset2 "a"
(integer) 1
redis> SMEMBERS myset1
1) "b"
2) "c"
redis> SMEMBERS myset2
1) "a"
2) "x"
3) "y"
redis> SMOVE myset1 myset2 "d"
(integer) 0
redis> SMOVE myset1 myset2 "b"
(integer) 1
redis> SMEMBERS myset2
1) "a"
2) "b"
3) "x"
4) "y"
```

