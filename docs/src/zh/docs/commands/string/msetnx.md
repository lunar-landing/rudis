# MSETNX

当所有给定的键都不存在时，同时设置一个或多个键值对。

## Syntax

```
MSETNX key value [key value ...]
```

## Return

Integer reply: 

- 1 如果所有键都被成功设置
- 0 如果所有给定键的设置都失败（至少有一个键已经存在）

## Examples

```
redis> MSETNX key1 "Hello" key2 "World"
(integer) 1
redis> MSETNX key2 "New" key3 "Value"
(integer) 0
redis> GET key1
"Hello"
redis> EXISTS key3
(integer) 0
```