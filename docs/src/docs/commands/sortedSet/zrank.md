# ZRANK

Returns the rank of the specified member in the sorted set, with scores ordered from low to high.

## Syntax

```
ZRANK key member
```

## Return

Integer reply: The rank of the member, or nil if the member does not exist or the key does not exist.

## Examples

```
redis> ZADD myzset 1 "one"
(integer) 1
redis> ZADD myzset 2 "two"
(integer) 1
redis> ZADD myzset 3 "three"
(integer) 1
redis> ZRANK myzset "two"
(integer) 1
```
