# HVALS

Returns all values in the hash.

## Syntax

```
HVALS key
```

## Return

Array reply: List of all values in the hash.

## Examples

```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HSET myhash field2 "World"
(integer) 1
redis> HVALS myhash
1) "Hello"
2) "World"
```
