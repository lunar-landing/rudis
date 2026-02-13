# HEXISTS

Checks whether the specified field exists in the hash.

## Syntax

```
HEXISTS key field
```

## Return

Integer reply: 1 if the field exists, 0 if the field does not exist or the key does not exist.

## Examples

```
redis> HSET myhash field1 "foo"
(integer) 1
redis> HEXISTS myhash field1
(integer) 1
redis> HEXISTS myhash field2
(integer) 0
```
