# SETNX

Sets the value of a key only if the key does not exist. If the key already exists, no operation is performed.

## Syntax

```
SETNX key value
```

## Return

Integer reply: Returns 1 if the key was set, or 0 if the key already exists.

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

