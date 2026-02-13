# PEXPIREAT

Sets the expiration timestamp for the given key (Unix timestamp in milliseconds). Returns 0 if the key does not exist, 1 if the expiration was set successfully.

## Syntax

```
PEXPIREAT key milliseconds-timestamp
```

## Return

Integer reply: 1 if the expiration was set, 0 if the key does not exist.

## Examples

```
redis> SET mykey "Hello"
OK
redis> PEXPIREAT mykey 1293840000000
(integer) 1
redis> PTTL mykey
(integer) 1234567890123
```
