# PERSIST

Removes the expiration from the given key, making it a persistent key. Returns 0 if the key does not exist or has no expiration set.

## Syntax

```
PERSIST key
```

## Return

Integer reply: 1 if the expiration was removed successfully, 0 if the key does not exist or has no expiration set.

## Examples

```
redis> SET mykey "Hello"
OK
redis> EXPIRE mykey 10
(integer) 1
redis> TTL mykey
(integer) 10
redis> PERSIST mykey
(integer) 1
redis> TTL mykey
(integer) -1
```
