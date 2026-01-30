# BITCOUNT

BITCOUNT 命令用于统计字符串中设置为 1 的位的数量（人口计数）。

## 语法

```
BITCOUNT key [start] [end]
```

## 参数

- `key` - 键名
- `start` - 可选。开始计数的字节索引（从 0 开始）。可以使用负数从末尾开始计数。
- `end` - 可选。结束计数的字节索引（从 0 开始）。可以使用负数从末尾开始计数。

## 返回值

整数回复：设置为 1 的位的数量。

## 示例

```
redis> SETBIT mykey 0 1
(integer) 0
redis> SETBIT mykey 1 1
(integer) 0
redis> SETBIT mykey 2 1
(integer) 0
redis> BITCOUNT mykey
(integer) 3
redis> BITCOUNT mykey 0 0
(integer) 3
redis> SET test_string "foobar"
OK
redis> BITCOUNT test_string 0 0
(integer) 4
```

## 说明

- 如果未指定范围，则统计整个字符串中的位。
- 如果范围超出边界，返回 0。
- 负数索引从字符串末尾开始计数。
- 范围以字节为单位指定，而不是位。

