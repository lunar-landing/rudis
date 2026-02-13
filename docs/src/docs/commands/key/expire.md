# EXPIRE

Sets a timeout on the given key in seconds. Returns 0 if the key does not exist, 1 if the timeout was set successfully.

## Syntax

```
EXPIRE key seconds
```

## Return

Integer reply: 1 if the timeout was set, 0 if the key does not exist.

## Examples

```
redis> SET mykey "Hello"
OK
redis> EXPIRE mykey 10
(integer) 1
redis> TTL mykey
(integer) 10
redis> SET mykey "Hello World"
OK
redis> TTL mykey
(integer) -1
```
