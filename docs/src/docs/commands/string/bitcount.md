# BITCOUNT

The BITCOUNT command counts the number of set bits (population counting) in a string.

## Syntax

```
BITCOUNT key [start] [end]
```

## Parameters

- `key` - The key name
- `start` - Optional. Byte index to start counting (0-based). Can be negative to count from the end.
- `end` - Optional. Byte index to end counting (0-based). Can be negative to count from the end.

## Return

Integer reply: the number of bits set to 1.

## Examples

```
redis> SETBIT mykey 0 1
(integer) 0
redis> SETBIT mykey 1 1
(integer) 0
redis> SETBIT mykey 2 1
(integer) 0
redis> BITCOUNT mykey
(integer) 3
redis> BITCOUNT mykey 0 0
(integer) 3
redis> SET test_string "foobar"
OK
redis> BITCOUNT test_string 0 0
(integer) 4
```

## Notes

- If no range is specified, counts bits in the entire string.
- If the range is out of bounds, returns 0.
- Negative indices count from the end of the string.
- The range is specified in bytes, not bits.

