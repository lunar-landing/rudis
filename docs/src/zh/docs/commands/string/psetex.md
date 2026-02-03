# PSETEX

设置键的值，并指定键的过期时间（以毫秒为单位）。PSETEX 是原子操作，等同于 SET + PEXPIRE，但比这两个命令的组合更快。

## Syntax

```
PSETEX key milliseconds value
```

## Return

Simple string reply: 如果设置成功，返回 OK。

## Examples

```
redis> PSETEX mykey 10000 "hello"
OK
redis> PTTL mykey
(integer) 10000
redis> GET mykey
"hello"
redis> PSETEX mykey 20000 "world"
OK
redis> PTTL mykey
(integer) 20000
redis> GET mykey
"world"
```

