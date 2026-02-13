# RPUSH

Inserts one or more values at the tail (right) of the list. If the key does not exist, an empty list is created before the operation.

## Syntax

```
RPUSH key value [value ...]
```

## Return

Integer reply: The length of the list after the insert operation.

## Examples

```
redis> RPUSH mylist "hello"
(integer) 1
redis> RPUSH mylist "world"
(integer) 2
redis> RPUSH mylist "foo" "bar"
(integer) 4
redis> LRANGE mylist 0 -1
1) "hello"
2) "world"
3) "foo"
4) "bar"
```
