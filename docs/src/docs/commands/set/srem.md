# SREM

Removes one or more elements from the set.

## Syntax

```
SREM key member [member ...]
```

## Return

Integer reply: The number of elements that were successfully removed, not including non-existent elements.

## Examples

```
redis> SADD myset "Hello"
(integer) 1
redis> SADD myset "World"
(integer) 1
redis> SREM myset "Hello"
(integer) 1
redis> SMEMBERS myset
1) "World"
```
