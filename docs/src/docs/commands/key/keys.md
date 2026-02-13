# KEYS

Finds all keys matching the given pattern. Supports glob-style pattern matching.

## Syntax

```
KEYS pattern
```

## Return

Array reply: List of keys matching the pattern.

## Examples

```
redis> MSET firstname Jack lastname Stuntman age 35
OK
redis> KEYS *name*
1) "firstname"
2) "lastname"
redis> KEYS a??
1) "age"
redis> KEYS *
1) "firstname"
2) "lastname"
3) "age"
```
