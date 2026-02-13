# RENAMENX

Renames the key to a new key only if the new key does not exist. The operation fails if the new key already exists.

## Syntax

```
RENAMENX key newkey
```

## Return

Integer reply:

- 1 if the key was renamed successfully
- 0 if the new key already exists
- Error if the key does not exist

## Examples

```
redis> SET mykey "Hello"
OK
redis> SET myotherkey "World"
OK
redis> RENAMENX mykey myotherkey
(integer) 0
redis> RENAMENX mykey mynewkey
(integer) 1
```
