# SETBIT

The SETBIT command sets or clears the bit at offset in the string value stored at key.

## Syntax

```
SETBIT key offset value
```

## Parameters

- `key` - The key name
- `offset` - The bit offset (0-based). The offset must be a non-negative integer.
- `value` - The bit value, must be 0 or 1

## Return

Integer reply: the original bit value stored at offset.

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
```

## Notes

- The string is automatically extended if the offset is beyond the current string length.
- The string is padded with zero bytes to fill the gap if necessary.
- Bit offsets are numbered from 0, starting from the most significant bit of the first byte.

