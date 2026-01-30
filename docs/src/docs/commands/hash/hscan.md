# HSCAN

The HSCAN command is used to incrementally iterate over the fields and values of a hash.

## Syntax

```
HSCAN key cursor [MATCH pattern] [COUNT count]
```

## Parameters

- `key` - The hash key name
- `cursor` - Cursor, use 0 as cursor for the first iteration
- `pattern` - Optional, matching pattern for field names
- `count` - Optional, specifies the number of field-value pairs to return per iteration, default value is 10

## Return Value

An array list containing two elements:
1. The new cursor for the next iteration
2. An array containing field-value pairs in the format [field1, value1, field2, value2, ...]

If the new cursor returns 0, it indicates that the iteration is complete.

## Examples

### Basic Iteration

```
redis> HSET myhash field1 value1
(integer) 1
redis> HSET myhash field2 value2
(integer) 1
redis> HSET myhash field3 value3
(integer) 1
redis> HSCAN myhash 0
1) (integer) 0
2) 1) "field1"
   2) "value1"
   3) "field2"
   4) "value2"
   5) "field3"
   6) "value3"
```

### Using MATCH Parameter

```
redis> HSET myhash user:1 value1
(integer) 1
redis> HSET myhash user:2 value2
(integer) 1
redis> HSET myhash admin:1 value3
(integer) 1
redis> HSCAN myhash 0 MATCH user:*
1) (integer) 0
2) 1) "user:1"
   2) "value1"
   3) "user:2"
   4) "value2"
```

### Using COUNT Parameter

```
redis> HSET myhash field1 value1
(integer) 1
redis> HSET myhash field2 value2
(integer) 1
redis> HSET myhash field3 value3
(integer) 1
redis> HSET myhash field4 value4
(integer) 1
redis> HSET myhash field5 value5
(integer) 1
redis> HSCAN myhash 0 COUNT 2
1) (integer) 2
2) 1) "field1"
   2) "value1"
   3) "field2"
   4) "value2"
```

### Continue Iteration Using Cursor

```
redis> HSCAN myhash 2 COUNT 2
1) (integer) 4
2) 1) "field3"
   2) "value3"
   3) "field4"
   4) "value4"
redis> HSCAN myhash 4 COUNT 2
1) (integer) 0
2) 1) "field5"
   2) "value5"
```

## Notes

- The iteration is not guaranteed to return fields in any particular order.
- Non-existent keys are treated as empty hashes and return cursor 0 with an empty array.
- The MATCH pattern is applied to field names, not values.
- The COUNT parameter is a hint, the actual number of returned items may vary.

