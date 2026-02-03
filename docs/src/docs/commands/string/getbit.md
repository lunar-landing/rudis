# GETBIT

The GETBIT command returns the bit value at offset in the string value stored at key.

## Syntax

```
GETBIT key offset
```

## Parameters

- `key` - The key name
- `offset` - The bit offset (0-based). The offset must be a non-negative integer.

## Return

Integer reply: the bit value stored at offset (0 or 1). Returns 0 if the offset is beyond the string length.

## Examples

```
redis> SETBIT mykey 0 1
(integer) 0
redis> SETBIT mykey 1 1
(integer) 0
redis> GETBIT mykey 0
(integer) 1
redis> GETBIT mykey 1
(integer) 1
redis> GETBIT mykey 100
(integer) 0
```

## Notes

- Returns 0 if the key does not exist.
- Returns 0 if the offset is beyond the string length.
- Bit offsets are numbered from 0, starting from the most significant bit of the first byte.

