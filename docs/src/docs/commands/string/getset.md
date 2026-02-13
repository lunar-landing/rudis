# GETSET

Sets the string value of the key and returns its old value. Returns nil if the key does not exist.

## Syntax

```
GETSET key value
```

## Return

Bulk string reply: The old value of the key, or nil if the key did not exist.

## Examples

```
redis> GETSET mykey "Hello"
(nil)
redis> GETSET mykey "World"
"Hello"
redis> GET mykey
"World"
```
