# PEXPIRE

Sets a timeout on the given key in milliseconds. Returns 0 if the key does not exist, 1 if the timeout was set successfully.

## Syntax

```
PEXPIRE key milliseconds
```

## Return

Integer reply: 1 if the timeout was set, 0 if the key does not exist.

## Examples

```
redis> SET mykey "Hello"
OK
redis> PEXPIRE mykey 1000
(integer) 1
redis> PTTL mykey
(integer) 1000
```
