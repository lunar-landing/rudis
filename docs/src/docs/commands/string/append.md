# APPEND

If the key already exists and is a string, appends the value to the end of the key's string value. If the key does not exist, it is first set to an empty string, then the append operation is performed.

## Syntax

```
APPEND key value
```

## Return

Integer reply: The length of the string after the append operation.

## Examples

```
redis> EXISTS mykey
(integer) 0
redis> APPEND mykey "Hello"
(integer) 5
redis> APPEND mykey " World"
(integer) 11
redis> GET mykey
"Hello World"
```
