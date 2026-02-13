# HGETALL

Returns all fields and values in the hash.

## Syntax

```
HGETALL key
```

## Return

Array reply: List of all fields and values in the hash in the format [field1, value1, field2, value2, ...].

## Examples

```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HSET myhash field2 "World"
(integer) 1
redis> HGETALL myhash
1) "field1"
2) "Hello"
3) "field2"
4) "World"
```
