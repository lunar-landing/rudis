# SCAN

The SCAN command is used to incrementally iterate over the database keys.

## Syntax

```
SCAN cursor [MATCH pattern] [COUNT count]
```

## Parameters

- `cursor` - Cursor, use 0 as cursor for the first iteration
- `pattern` - Optional, matching pattern
- `count` - Optional, specifies the number of keys to return per iteration, default value is 10

## Return Value

An array list containing two elements:
1. The new cursor for the next iteration
2. An array containing all iterated elements

If the new cursor returns 0, it indicates that the iteration is complete.

## Examples

### Basic Iteration

```
redis> scan 0
1) "17"
2)  1) "key:12"
    2) "key:8"
    3) "key:4"
    4) "key:14"
    5) "key:16"
    6) "key:17"
    7) "key:15"
    8) "key:10"
    9) "key:3"
   10) "key:7"
   11) "key:1"
```

### Continue Iteration Using Cursor

```
redis> scan 17
1) "0"
2)  1) "key:5"
    2) "key:18"
    3) "key:0"
    4) "key:2"
    5) "key:19"
    6) "key:13"
    7) "key:6"
    8) "key:9"
    9) "key:11"
```

### Using MATCH Parameter

```
redis> scan 0 MATCH user:*
1) "0"
2)  1) "user:1"
    2) "user:2"
    3) "user:3"
```

### Using COUNT Parameter

```
redis> scan 0 COUNT 5
1) "5"
2)  1) "key:1"
    2) "key:2"
    3) "key:3"
    4) "key:4"
    5) "key:5"
```