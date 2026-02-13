# RANDOMKEY

Returns a random key from the current database. Returns nil if the database is empty.

## Syntax

```
RANDOMKEY
```

## Return

Bulk string reply: A random key, or nil if the database is empty.

## Examples

```
redis> MSET fruit "apple" vegetable "carrot" drink "water"
OK
redis> RANDOMKEY
"fruit"
redis> RANDOMKEY
"drink"
redis> FLUSHDB
OK
redis> RANDOMKEY
(nil)
```
