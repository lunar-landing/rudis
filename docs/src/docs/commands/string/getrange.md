# GETRANGE

Returns the substring of the string value stored at the key, determined by the start and end offsets. Both offsets are 0-based; negative offsets count from the end of the string.

## Syntax

```
GETRANGE key start end
```

## Return

Bulk string reply: The substring within the specified range.

## Examples

```
redis> SET mykey "This is a string"
OK
redis> GETRANGE mykey 0 3
"This"
redis> GETRANGE mykey -3 -1
"ing"
redis> GETRANGE mykey 0 -1
"This is a string"
redis> GETRANGE mykey 10 100
"string"
```
