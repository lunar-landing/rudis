# EXPIREAT

Sets the expiration timestamp for the given key (Unix timestamp in seconds). Returns 0 if the key does not exist, 1 if the expiration was set successfully.

## Syntax

```
EXPIREAT key timestamp
```

## Return

Integer reply: 1 if the expiration was set, 0 if the key does not exist.

## Examples

```
redis> SET mykey "Hello"
OK
redis> EXPIREAT mykey 1293840000
(integer) 1
redis> TTL mykey
(integer) 1234567890
```
