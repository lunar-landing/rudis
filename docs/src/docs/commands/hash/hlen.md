# HLEN

Returns the number of fields in the hash.

## Syntax

```
HLEN key
```

## Return

Integer reply: The number of fields in the hash, or 0 if the key does not exist.

## Examples

```
redis> HSET myhash field1 "Hello"
(integer) 1
redis> HSET myhash field2 "World"
(integer) 1
redis> HLEN myhash
(integer) 2
```
