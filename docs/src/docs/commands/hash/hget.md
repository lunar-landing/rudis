# HGET

Returns the value associated with the specified field in the hash.

## Syntax

```
HGET key field
```

## Return

Bulk string reply: The value of the field, or nil if the field does not exist or the key does not exist.

## Examples

```
redis> HSET myhash field1 "foo"
(integer) 1
redis> HGET myhash field1
"foo"
redis> HGET myhash field2
(nil)
```
