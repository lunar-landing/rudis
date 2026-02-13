# LPUSH

Inserts one or more values at the head (left) of the list. If the key does not exist, an empty list is created before the operation.

## Syntax

```
LPUSH key value [value ...]
```

## Return

Integer reply: The length of the list after the insert operation.

## Examples

```
redis> LPUSH mylist "world"
(integer) 1
redis> LPUSH mylist "hello"
(integer) 2
redis> LPUSH mylist "foo" "bar"
(integer) 4
redis> LRANGE mylist 0 -1
1) "bar"
2) "foo"
3) "hello"
4) "world"
```
