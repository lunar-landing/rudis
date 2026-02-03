# PFADD

The PFADD command adds elements to the HyperLogLog data structure stored at the specified key.

HyperLogLog is a probabilistic structure: it **estimates** the number of unique elements using a small and fixed amount of memory.

## Syntax

```
PFADD key [element [element ...]]
```

## Parameters

- `key` - HyperLogLog key name
- `element` - One or more elements to add

## Return

Integer reply:

- `1` if at least one HyperLogLog internal register was altered.
- `0` if no HyperLogLog internal registers were altered.

Special case:

- `PFADD key` (no elements) creates an empty HyperLogLog if `key` does not exist and returns `1`. If `key` already exists, it performs no operation and returns `0`.

## Examples

```
redis> PFADD hll1 element1
(integer) 1
redis> PFADD hll1 element2 element3 element4
(integer) 1
redis> PFADD hll1 element1 element2
(integer) 0
redis> PFADD hll1 element5
(integer) 1
```

