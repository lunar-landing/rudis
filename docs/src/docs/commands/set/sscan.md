# SSCAN

The SSCAN command is used to incrementally iterate over elements in a set.

## Syntax

```
SSCAN key cursor [MATCH pattern] [COUNT count]
```

## Parameters

| Parameter | Description |
|-----------|-------------|
| key | The key of the set to iterate over |
| cursor | The cursor for iteration, use 0 for the first call, then use the cursor returned by the previous call |
| MATCH pattern | Optional parameter to specify a pattern for matching returned values |
| COUNT count | Optional parameter to specify the number of elements to return per iteration, defaults to 10 |

## Return Value

Array reply: An array containing two elements.
- The first element is an integer representing the new cursor for the next iteration. A value of 0 indicates that the iteration is complete.
- The second element is an array containing the elements returned by this iteration.

## Examples

### Basic Usage

```
redis> SADD myset1 "Google"
(integer) 1
redis> SADD myset1 "Runoob"
(integer) 1
redis> SADD myset1 "Taobao"
(integer) 1
redis> SSCAN myset1 0
1) "0"
2) 1) "Google"
   2) "Runoob"
   3) "Taobao"
```

### Using MATCH Parameter

```
redis> SSCAN myset1 0 MATCH R*
1) "0"
2) 1) "Runoob"
```

### Using COUNT Parameter

```
redis> SSCAN myset1 0 COUNT 2
1) "2"
2) 1) "Google"
   2) "Runoob"
```

## Use Cases

The SSCAN command is suitable for iterating over large sets without blocking the server. It allows for progressive iteration over elements in a set, avoiding performance issues caused by retrieving large amounts of data at once.