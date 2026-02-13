# MSET

Sets multiple key-value pairs in one operation. If a key already exists, it is overwritten.

## Syntax

```
MSET key value [key value ...]
```

## Return

Simple string reply: OK

## Examples

```
redis> MSET key1 "Hello" key2 "World"
OK
redis> GET key1
"Hello"
redis> GET key2
"World"
```
