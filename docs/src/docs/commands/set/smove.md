# SMOVE

Moves a specified member from the source set to the destination set. If the member does not exist in the source set, no operation is performed. If the member already exists in the destination set, it is only removed from the source set and will not be duplicated in the destination set.

## Syntax

```
SMOVE source destination member
```

## Return

Integer reply: Returns 1 if the member was successfully moved, or 0 if the member does not exist in the source set.

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

