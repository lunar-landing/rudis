# SETEX

设置键的值，并指定键的过期时间（以秒为单位）。SETEX 是原子操作，等同于 SET + EXPIRE，但比这两个命令的组合更快。

## Syntax

```
SETEX key seconds value
```

## Return

Simple string reply: 如果设置成功，返回 OK。

## Examples

```
redis> SETEX mykey 10 "hello"
OK
redis> TTL mykey
(integer) 10
redis> GET mykey
"hello"
redis> SETEX mykey 20 "world"
OK
redis> TTL mykey
(integer) 20
redis> GET mykey
"world"
```

