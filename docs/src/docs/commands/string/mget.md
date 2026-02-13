# MGET

Returns the values of all specified keys. Returns nil for non-existent keys.

## Syntax

```
MGET key [key ...]
```

## Return

Array reply: List of values of the specified keys.

## Examples

```
redis> SET key1 "Hello"
OK
redis> SET key2 "World"
OK
redis> MGET key1 key2 nonexisting
1) "Hello"
2) "World"
3) (nil)
```
