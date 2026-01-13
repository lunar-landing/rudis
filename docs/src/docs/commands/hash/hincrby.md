# HINCRBY

Increments the number stored at field in the hash stored at key by increment. If key does not exist, a new key holding a hash is created. If field does not exist the value is set to 0 before the operation is performed.

The range of values supported by HINCRBY is limited to 64 bit signed integers.

## Syntax

```
HINCRBY key field increment
```

## Return

Integer reply: the value at field after the increment operation.

## Examples

```
redis> HSET myhash field1 5
(integer) 1
redis> HINCRBY myhash field1 1
(integer) 6
redis> HINCRBY myhash field1 -1
(integer) 5
redis> HINCRBY myhash field1 -10
(integer) -5
```

## See also

[HINCRBYFLOAT](./hincrbyfloat.md) - Increment the float value of a hash field by the given amount