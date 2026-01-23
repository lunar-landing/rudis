# PSETEX

Sets the value of a key and specifies the expiration time of the key in milliseconds. PSETEX is an atomic operation, equivalent to SET + PEXPIRE, but faster than the combination of these two commands.

## Syntax

```
PSETEX key milliseconds value
```

## Return

Simple string reply: Returns OK if the key was set successfully.

## Examples

```
redis> PSETEX mykey 10000 "hello"
OK
redis> PTTL mykey
(integer) 10000
redis> GET mykey
"hello"
redis> PSETEX mykey 20000 "world"
OK
redis> PTTL mykey
(integer) 20000
redis> GET mykey
"world"
```

