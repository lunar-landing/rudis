# SCAN

SCAN 命令用于迭代数据库中的数据库键。

## 语法

```
SCAN cursor [MATCH pattern] [COUNT count]
```

## 参数

- `cursor` - 游标，第一次迭代使用 0 作为游标
- `pattern` - 可选，匹配的模式
- `count` - 可选，用于指定每次迭代返回的 key 的数量，默认值为 10

## 返回值

数组列表，包含两个元素：
1. 用于进行下一次迭代的新游标
2. 包含所有被迭代元素的数组

如果新游标返回 0 表示迭代已结束。

## 示例

### 基本迭代

```
redis> scan 0
1) "17"
2)  1) "key:12"
    2) "key:8"
    3) "key:4"
    4) "key:14"
    5) "key:16"
    6) "key:17"
    7) "key:15"
    8) "key:10"
    9) "key:3"
   10) "key:7"
   11) "key:1"
```

### 使用游标继续迭代

```
redis> scan 17
1) "0"
2)  1) "key:5"
    2) "key:18"
    3) "key:0"
    4) "key:2"
    5) "key:19"
    6) "key:13"
    7) "key:6"
    8) "key:9"
    9) "key:11"
```

### 使用 MATCH 参数

```
redis> scan 0 MATCH user:*
1) "0"
2)  1) "user:1"
    2) "user:2"
    3) "user:3"
```

### 使用 COUNT 参数

```
redis> scan 0 COUNT 5
1) "5"
2)  1) "key:1"
    2) "key:2"
    3) "key:3"
    4) "key:4"
    5) "key:5"
```