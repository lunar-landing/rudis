# PFCOUNT

The PFCOUNT command returns the **estimated cardinality** (number of unique elements) observed by one or more HyperLogLog keys.

When multiple keys are provided, PFCOUNT returns the estimated cardinality of the **union** of the HyperLogLogs (computed by merging them internally).

## Syntax

```
PFCOUNT key [key ...]
```

## Parameters

- `key` - One or more HyperLogLog keys

## Return

Integer reply: the estimated number of unique elements. If a key does not exist, it is treated as an empty HyperLogLog (count = 0).

## Notes

- The result is **approximate**.
- With multiple keys, PFCOUNT is slower because it needs to merge registers on-the-fly.

## Examples

```
redis> PFADD hll1 element1 element2 element3 element4 element5
(integer) 1
redis> PFADD hll2 a b c d
(integer) 1
redis> PFCOUNT hll1
(integer) 5
redis> PFCOUNT hll2
(integer) 4
redis> PFCOUNT hll1 hll2
(integer) 9
```

