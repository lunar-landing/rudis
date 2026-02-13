# HSETNX

Sets the value of a field in the hash only if the field does not exist. If the field already exists, the operation is ignored.

## Syntax

```
HSETNX key field value
```

## Return

Integer reply: 1 if the field was newly created and the value was set, 0 if the field already exists.

## Examples

```
redis> HSETNX myhash field "Hello"
(integer) 1
redis> HSETNX myhash field "World"
(integer) 0
redis> HGET myhash field
"Hello"
```
