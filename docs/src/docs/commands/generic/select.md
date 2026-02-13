# SELECT

Switches to the specified database. Redis has 16 databases by default, numbered from 0 to 15.

## Syntax

```
SELECT index
```

## Return

Simple string reply: OK

## Examples

```
redis> SELECT 0
OK
redis> SELECT 15
OK
redis> SELECT 16
ERR invalid DB index
```
