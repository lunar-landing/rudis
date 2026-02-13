# HMSET

Sets multiple field-value pairs in the hash in one operation. If a field already exists, it is overwritten.

## Syntax

```
HMSET key field value [field value ...]
```

## Return

Simple string reply: OK

## Examples

```
redis> HMSET myhash field1 "Hello" field2 "World"
OK
redis> HGET myhash field1
"Hello"
redis> HGET myhash field2
"World"
```
