---
title: SETRANGE
titleTemplate: 字符串命令
description: Redis SETRANGE 命令用指定的字符串覆盖给定 key 所储存的字符串值，覆盖的位置从偏移量 offset 开始。
---

# SETRANGE

Redis `SETRANGE` 命令用指定的字符串覆盖给定 key 所储存的字符串值，覆盖的位置从偏移量 offset 开始。

## 语法

```
SETRANGE key offset value
```

## 可用版本

>= 2.2.0

## 返回值

Integer reply: 被修改后的字符串长度。

## 示例

```
redis> SET key1 "Hello World"
OK
redis> SETRANGE key1 6 "Redis"
(integer) 11
redis> GET key1
"Hello Redis"
```

如果 offset 超过了当前字符串的长度，中间会用 `\x00` 字节填充：

```
redis> SET key2 "Hello"
OK
redis> SETRANGE key2 10 "World"
(integer) 15
redis> GET key2
"Hello\x00\x00\x00\x00\x00World"
```

当 key 不存在时，会创建一个新的字符串，offset 之前的字符用 `\x00` 填充：

```
redis> SETRANGE key3 5 "World"
(integer) 10
redis> GET key3
"\x00\x00\x00\x00\x00World"
```