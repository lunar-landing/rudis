# PTTL

Similar to TTL, but returns the remaining time to live of the key in milliseconds. Returns -2 if the key does not exist, -1 if the key exists but has no expiration set.

## Syntax

```
PTTL key
```

## Return

Integer reply: The remaining time to live of the key in milliseconds, or a negative value indicating an error.

- -2 indicates the key does not exist
- -1 indicates the key exists but has no associated expiration

## Examples

```
redis> SET mykey "Hello"
OK
redis> PEXPIRE mykey 1000
(integer) 1
redis> PTTL mykey
(integer) 1000
```
