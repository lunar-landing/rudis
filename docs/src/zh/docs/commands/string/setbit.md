# SETBIT

SETBIT 命令用于设置或清除字符串值中指定偏移量的位。

## 语法

```
SETBIT key offset value
```

## 参数

- `key` - 键名
- `offset` - 位偏移量（从 0 开始）。偏移量必须是非负整数。
- `value` - 位值，必须是 0 或 1

## 返回值

整数回复：存储在偏移量处的原始位值。

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
```

## 说明

- 如果偏移量超出当前字符串长度，字符串会自动扩展。
- 如果需要，字符串会用零字节填充以填补间隙。
- 位偏移量从 0 开始编号，从第一个字节的最高有效位开始。

