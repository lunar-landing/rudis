# RPUSHX

Inserts one or more values at the tail (right) of the list only if the list key already exists. If the key does not exist, no operation is performed.

## Syntax

```
RPUSHX key value [value ...]
```

## Return

Integer reply: The length of the list after the insert operation, or 0 if the key does not exist.

## Examples

```
redis> RPUSHX mylist "world"
(integer) 0
redis> RPUSH mylist "hello"
(integer) 1
redis> RPUSHX mylist "world"
(integer) 2
redis> LRANGE mylist 0 -1
1) "hello"
2) "world"
```
