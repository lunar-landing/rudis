# PFMERGE

The PFMERGE command merges multiple HyperLogLog values into a single destination key.

The resulting HyperLogLog approximates the cardinality of the **union** of all source HyperLogLogs.

## Syntax

```
PFMERGE destkey [sourcekey [sourcekey ...]]
```

## Parameters

- `destkey` - Destination HyperLogLog key
- `sourcekey` - One or more source HyperLogLog keys

## Return

Simple string reply: `OK`.

## Notes

- If `destkey` does not exist, it will be created as an empty HyperLogLog.
- If a `sourcekey` does not exist, it is treated as an empty HyperLogLog (ignored).

## Examples

```
redis> PFADD hll1 element1 element2 element3 element4 element5
(integer) 1
redis> PFADD hll2 a b c d
(integer) 1
redis> PFMERGE hll_dest hll1 hll2
OK
redis> PFCOUNT hll_dest
(integer) 9
```

