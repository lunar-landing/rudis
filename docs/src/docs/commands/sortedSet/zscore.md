# ZSCORE

Returns the score of the specified member in the sorted set.

## Syntax

```
ZSCORE key member
```

## Return

Bulk string reply: The score of the member, or nil if the member does not exist or the key does not exist.

## Examples

```
redis> ZADD myzset 1 "one"
(integer) 1
redis> ZADD myzset 2 "two"
(integer) 1
redis> ZSCORE myzset "one"
"1"
redis> ZSCORE myzset "three"
(nil)
```
