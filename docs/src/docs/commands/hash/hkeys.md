# HKEYS

Returns all field names in the hash.

## Syntax

```
HKEYS key
```

## Return

Array reply: List of all field names in the hash.

## Examples

```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HSET myhash field2 "World"
(integer) 1
redis> HKEYS myhash
1) "field1"
2) "field2"
```
