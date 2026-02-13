# MOVE

Moves the specified key from the current database to the specified database number. If the key already exists in the target database, no operation is performed.

## Syntax

```
MOVE key db
```

## Return

Integer reply: 1 if the key was moved successfully, 0 if the key does not exist or a key with the same name already exists in the target database.

## Examples

```
redis> SELECT 0
OK
redis> SET mykey "Hello"
OK
redis> MOVE mykey 1
(integer) 1
redis> EXISTS mykey
(integer) 0
redis> SELECT 1
OK
redis> EXISTS mykey
(integer) 1
```
