# BITOP

The BITOP command performs bitwise operations between strings and stores the result in a destination key.

## Syntax

```
BITOP operation destkey key [key ...]
```

## Parameters

- `operation` - The bitwise operation to perform. Can be one of: AND, OR, XOR, NOT
- `destkey` - The destination key where the result is stored
- `key` - One or more source keys (NOT operation accepts only one key)

## Operations

- `AND` - Bitwise AND operation
- `OR` - Bitwise OR operation
- `XOR` - Bitwise XOR operation
- `NOT` - Bitwise NOT operation (requires exactly one source key)

## Return

Integer reply: the length of the string stored in the destination key, which is equal to the length of the longest input string.

## Examples

```
redis> SETBIT key1 0 1
(integer) 0
redis> SETBIT key1 1 0
(integer) 0
redis> SETBIT key2 0 1
(integer) 0
redis> SETBIT key2 1 1
(integer) 0
redis> BITOP AND result key1 key2
(integer) 1
redis> GETBIT result 0
(integer) 1
redis> GETBIT result 1
(integer) 0
redis> BITOP OR result_or key1 key2
(integer) 1
redis> BITOP NOT result_not key1
(integer) 1
```

## Notes

- Non-existent keys are treated as empty strings (all bits set to 0).
- The result length is equal to the length of the longest input string.
- NOT operation requires exactly one source key.
- The operation is performed byte by byte.

