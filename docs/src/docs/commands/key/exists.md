# EXISTS

Checks whether the given key(s) exist. Returns 1 if the key exists, 0 otherwise.

## Syntax

```
EXISTS key [key ...]
```

## Return

Integer reply: The number of keys that exist.

## Examples

```
redis> SET key1 "Hello"
OK
redis> EXISTS key1
(integer) 1
redis> EXISTS nosuchkey
(integer) 0
redis> SET key2 "World"
OK
redis> EXISTS key1 key2 nosuchkey
(integer) 2
```
