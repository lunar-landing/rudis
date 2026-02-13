# STRLEN

Returns the length of the string value stored at the key. Returns 0 if the key does not exist.

## Syntax

```
STRLEN key
```

## Return

Integer reply: The length of the string value, or 0 if the key does not exist.

## Examples

```
redis> SET mykey "Hello world"
OK
redis> STRLEN mykey
(integer) 11
redis> STRLEN nonexisting
(integer) 0
```
