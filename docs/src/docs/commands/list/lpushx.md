# LPUSHX

Inserts one or more values at the head (left) of the list only if the list key already exists. If the key does not exist, no operation is performed.

## Syntax

```
LPUSHX key value [value ...]
```

## Return

Integer reply: The length of the list after the insert operation, or 0 if the key does not exist.

## Examples

```
redis> LPUSHX mylist "world"
(integer) 0
redis> LPUSH mylist "hello"
(integer) 1
redis> LPUSHX mylist "world"
(integer) 2
redis> LRANGE mylist 0 -1
1) "world"
2) "hello"
```
