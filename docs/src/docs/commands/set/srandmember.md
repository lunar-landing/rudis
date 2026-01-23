# SRANDMEMBER

Returns one or more random members from a set without removing them. Unlike SPOP, SRANDMEMBER does not modify the set.

## Syntax

```
SRANDMEMBER key [count]
```

## Return

- When no count argument is provided, returns a random member; returns (nil) if the set is empty.
- When a count argument is provided:
  - If count is positive, returns count unique random members (array).
  - If count is negative, returns count possibly duplicate random members (array).
  - If the set is empty, returns an empty array.

## Examples

```
redis> SADD myset "one"
(integer) 1
redis> SADD myset "two"
(integer) 1
redis> SADD myset "three"
(integer) 1
redis> SADD myset "four"
(integer) 1
redis> SADD myset "five"
(integer) 1
redis> SRANDMEMBER myset
"three"
redis> SRANDMEMBER myset 3
1) "two"
2) "five"
3) "one"
redis> SRANDMEMBER myset -3
1) "three"
2) "three"
3) "one"
redis> SCARD myset
(integer) 5
```

