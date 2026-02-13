# LRANGE

Returns the elements in the list within the specified range. Both start and stop are 0-based indices; negative indices count from the end of the list.

## Syntax

```
LRANGE key start stop
```

## Return

Array reply: List of elements within the specified range.

## Examples

```
redis> RPUSH mylist "one"
(integer) 1
redis> RPUSH mylist "two"
(integer) 2
redis> RPUSH mylist "three"
(integer) 3
redis> LRANGE mylist 0 0
1) "one"
redis> LRANGE mylist -3 2
1) "one"
2) "two"
3) "three"
redis> LRANGE mylist -100 100
1) "one"
2) "two"
3) "three"
redis> LRANGE mylist 5 10
(empty list or set)
```
