# LINDEX

Returns the element at the specified index in the list. The index is 0-based; negative indices count from the end of the list.

## Syntax

```
LINDEX key index
```

## Return

Bulk string reply: The element at the specified index, or nil if the index is out of range.

## Examples

```
redis> LPUSH mylist "World"
(integer) 1
redis> LPUSH mylist "Hello"
(integer) 2
redis> LINDEX mylist 0
"Hello"
redis> LINDEX mylist -1
"World"
redis> LINDEX mylist 3
(nil)
```
