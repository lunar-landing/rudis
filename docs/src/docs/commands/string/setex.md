# SETEX

Sets the value of a key and specifies the expiration time of the key in seconds. SETEX is an atomic operation, equivalent to SET + EXPIRE, but faster than the combination of these two commands.

## Syntax

```
SETEX key seconds value
```

## Return

Simple string reply: Returns OK if the key was set successfully.

## Examples

```
redis> SETEX mykey 10 "hello"
OK
redis> TTL mykey
(integer) 10
redis> GET mykey
"hello"
redis> SETEX mykey 20 "world"
OK
redis> TTL mykey
(integer) 20
redis> GET mykey
"world"
```

