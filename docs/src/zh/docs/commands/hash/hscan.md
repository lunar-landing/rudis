# HSCAN

HSCAN 命令用于增量迭代哈希表中的字段和值。

## 语法

```
HSCAN key cursor [MATCH pattern] [COUNT count]
```

## 参数

- `key` - 哈希键名
- `cursor` - 游标，第一次迭代使用 0 作为游标
- `pattern` - 可选，字段名的匹配模式
- `count` - 可选，指定每次迭代返回的字段-值对数量，默认值为 10

## 返回值

包含两个元素的数组：
1. 下一次迭代的新游标
2. 包含字段-值对的数组，格式为 [field1, value1, field2, value2, ...]

如果新游标返回 0，表示迭代已完成。

## 示例

### 基本迭代

```
redis> HSET myhash field1 value1
(integer) 1
redis> HSET myhash field2 value2
(integer) 1
redis> HSET myhash field3 value3
(integer) 1
redis> HSCAN myhash 0
1) (integer) 0
2) 1) "field1"
   2) "value1"
   3) "field2"
   4) "value2"
   5) "field3"
   6) "value3"
```

### 使用 MATCH 参数

```
redis> HSET myhash user:1 value1
(integer) 1
redis> HSET myhash user:2 value2
(integer) 1
redis> HSET myhash admin:1 value3
(integer) 1
redis> HSCAN myhash 0 MATCH user:*
1) (integer) 0
2) 1) "user:1"
   2) "value1"
   3) "user:2"
   4) "value2"
```

### 使用 COUNT 参数

```
redis> HSET myhash field1 value1
(integer) 1
redis> HSET myhash field2 value2
(integer) 1
redis> HSET myhash field3 value3
(integer) 1
redis> HSET myhash field4 value4
(integer) 1
redis> HSET myhash field5 value5
(integer) 1
redis> HSCAN myhash 0 COUNT 2
1) (integer) 2
2) 1) "field1"
   2) "value1"
   3) "field2"
   4) "value2"
```

### 使用游标继续迭代

```
redis> HSCAN myhash 2 COUNT 2
1) (integer) 4
2) 1) "field3"
   2) "value3"
   3) "field4"
   4) "value4"
redis> HSCAN myhash 4 COUNT 2
1) (integer) 0
2) 1) "field5"
   2) "value5"
```

## 说明

- 迭代不保证以任何特定顺序返回字段。
- 不存在的键被视为空哈希，返回游标 0 和空数组。
- MATCH 模式应用于字段名，而不是值。
- COUNT 参数是一个提示，实际返回的项目数量可能有所不同。

