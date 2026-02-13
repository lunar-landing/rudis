# HSTRLEN

Returns the string length of the value associated with the specified field in the hash.

## Syntax

```
HSTRLEN key field
```

## Return

Integer reply: The string length of the field value, or 0 if the field or key does not exist.

## Examples

```
redis> HSET myhash f1 "HelloWorld"
(integer) 1
redis> HSTRLEN myhash f1
(integer) 10
redis> HSTRLEN myhash f2
(integer) 0
```
