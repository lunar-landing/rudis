# HINCRBYFLOAT

Increments the specified field of a hash stored at key by the specified float increment. If the field does not exist, it is set to 0 before performing the operation.

## Syntax

```
HINCRBYFLOAT key field increment
```

## Return

Bulk string reply: the value of field after the increment.

## Examples

```
redis> HSET mykey field 10.50
(integer) 1
redis> HINCRBYFLOAT mykey field 0.1
"10.6"
redis> HINCRBYFLOAT mykey field -5
"5.6"
redis> HSET mykey field 5.0e3
(integer) 0
redis> HINCRBYFLOAT mykey field 2.0e2
"5200"
```

## See also

[HINCRBY](./hincrby.md) - Increment the integer value of a hash field by the given number