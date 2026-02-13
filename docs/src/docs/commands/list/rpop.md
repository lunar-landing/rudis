# RPOP

Removes and returns the last element (right) of the list.

## Syntax

```
RPOP key
```

## Return

Bulk string reply: The last element of the list, or nil if the list is empty or the key does not exist.

## Examples

```
redis> RPUSH mylist "one"
(integer) 1
redis> RPUSH mylist "two"
(integer) 2
redis> RPUSH mylist "three"
(integer) 3
redis> RPOP mylist
"three"
redis> LRANGE mylist 0 -1
1) "one"
2) "two"
```
