# GETBIT

GETBIT 命令返回字符串值中指定偏移量的位值。

## 语法

```
GETBIT key offset
```

## 参数

- `key` - 键名
- `offset` - 位偏移量（从 0 开始）。偏移量必须是非负整数。

## 返回值

整数回复：存储在偏移量处的位值（0 或 1）。如果偏移量超出字符串长度，返回 0。

## 示例

```
redis> SETBIT mykey 0 1
(integer) 0
redis> SETBIT mykey 1 1
(integer) 0
redis> GETBIT mykey 0
(integer) 1
redis> GETBIT mykey 1
(integer) 1
redis> GETBIT mykey 100
(integer) 0
```

## 说明

- 如果键不存在，返回 0。
- 如果偏移量超出字符串长度，返回 0。
- 位偏移量从 0 开始编号，从第一个字节的最高有效位开始。

