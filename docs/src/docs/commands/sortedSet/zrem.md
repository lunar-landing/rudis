# ZREM

Removes one or more members from the sorted set.

## Syntax

```
ZREM key member [member ...]
```

## Return

Integer reply: The number of members that were successfully removed, not including non-existent members.

## Examples

```
redis> ZADD myzset 1 "one"
(integer) 1
redis> ZADD myzset 2 "two"
(integer) 1
redis> ZADD myzset 3 "three"
(integer) 1
redis> ZREM myzset "two"
(integer) 1
redis> ZRANGE myzset 0 -1
1) "one"
2) "three"
```
