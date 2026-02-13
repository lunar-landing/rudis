# LSET

Sets the element at the specified index in the list. The index is 0-based; negative indices count from the end of the list.

## Syntax

```
LSET key index value
```

## Return

Simple string reply: OK, or an error if the index is out of range.

## Examples

```
redis> LPUSH mylist "one"
(integer) 1
redis> LPUSH mylist "two"
(integer) 2
redis> LPUSH mylist "three"
(integer) 3
redis> LSET mylist 0 "four"
OK
redis> LSET mylist -2 "five"
OK
redis> LRANGE mylist 0 -1
1) "four"
2) "five"
3) "one"
```
