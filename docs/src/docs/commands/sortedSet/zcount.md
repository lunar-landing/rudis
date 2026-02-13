# ZCOUNT

Returns the number of members in the sorted set with scores within the specified range.

## Syntax

```
ZCOUNT key min max
```

## Return

Integer reply: The number of members within the score range.

## Examples

```
redis> ZADD myzset 1 "one"
(integer) 1
redis> ZADD myzset 2 "two"
(integer) 1
redis> ZADD myzset 3 "three"
(integer) 1
redis> ZCOUNT myzset 1 2
(integer) 2
```
