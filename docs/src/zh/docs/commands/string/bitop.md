# BITOP

BITOP 命令对多个字符串执行位操作，并将结果存储在目标键中。

## 语法

```
BITOP operation destkey key [key ...]
```

## 参数

- `operation` - 要执行的位操作。可以是以下之一：AND、OR、XOR、NOT
- `destkey` - 存储结果的目标键
- `key` - 一个或多个源键（NOT 操作只接受一个键）

## 操作类型

- `AND` - 位与操作
- `OR` - 位或操作
- `XOR` - 位异或操作
- `NOT` - 位非操作（需要且仅需要一个源键）

## 返回值

整数回复：存储在目标键中的字符串长度，等于最长输入字符串的长度。

## 示例

```
redis> SETBIT key1 0 1
(integer) 0
redis> SETBIT key1 1 0
(integer) 0
redis> SETBIT key2 0 1
(integer) 0
redis> SETBIT key2 1 1
(integer) 0
redis> BITOP AND result key1 key2
(integer) 1
redis> GETBIT result 0
(integer) 1
redis> GETBIT result 1
(integer) 0
redis> BITOP OR result_or key1 key2
(integer) 1
redis> BITOP NOT result_not key1
(integer) 1
```

## 说明

- 不存在的键被视为空字符串（所有位设置为 0）。
- 结果长度等于最长输入字符串的长度。
- NOT 操作需要且仅需要一个源键。
- 操作按字节执行。

