---
title: SETRANGE
titleTemplate: String Commands
description: The Redis SETRANGE command overwrites part of the string stored at key, starting at the specified offset, with the given value.
---

# SETRANGE

The Redis `SETRANGE` command overwrites part of the string stored at key, starting at the specified offset, with the given value.

## Syntax

```
SETRANGE key offset value
```

## Available Versions

>= 2.2.0

## Return Value

Integer reply: the length of the string after it was modified by the command.

## Examples

```
redis> SET key1 "Hello World"
OK
redis> SETRANGE key1 6 "Redis"
(integer) 11
redis> GET key1
"Hello Redis"
```

If the offset exceeds the current length of the string, the gap is filled with null bytes (`\x00`):

```
redis> SET key2 "Hello"
OK
redis> SETRANGE key2 10 "World"
(integer) 15
redis> GET key2
"Hello\x00\x00\x00\x00\x00World"
```

When the key does not exist, a new string is created, and the characters before the offset are filled with null bytes (`\x00`):

```
redis> SETRANGE key3 5 "World"
(integer) 10
redis> GET key3
"\x00\x00\x00\x00\x00World"
```