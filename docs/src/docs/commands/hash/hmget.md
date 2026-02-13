# HMGET

Returns the values of one or more specified fields in the hash.

## Syntax

```
HMGET key field [field ...]
```

## Return

Array reply: List of values for the given fields; nil for non-existent fields.

## Examples

```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HSET myhash field2 "World"
(integer) 1
redis> HMGET myhash field1 field2 nofield
1) "Hello"
2) "World"
3) (nil)
```
