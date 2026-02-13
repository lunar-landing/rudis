# ZCARD

Returns the number of elements in the sorted set.

## Syntax

```
ZCARD key
```

## Return

Integer reply: The number of elements in the sorted set, or 0 if the key does not exist.

## Examples

```
redis> ZADD myzset 1 "one"
(integer) 1
redis> ZADD myzset 2 "two"
(integer) 1
redis> ZCARD myzset
(integer) 2
```
