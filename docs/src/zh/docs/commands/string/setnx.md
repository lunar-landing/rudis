# SETNX

只有在键不存在时，才设置键的值。如果键已经存在，则不执行任何操作。

## Syntax

```
SETNX key value
```

## Return

Integer reply: 如果键被设置，返回 1；如果键已存在，返回 0。

## Examples

```
redis> SETNX mykey "hello"
(integer) 1
redis> GET mykey
"hello"
redis> SETNX mykey "world"
(integer) 0
redis> GET mykey
"hello"
redis> DEL mykey
(integer) 1
redis> SETNX mykey "newvalue"
(integer) 1
redis> GET mykey
"newvalue"
```

