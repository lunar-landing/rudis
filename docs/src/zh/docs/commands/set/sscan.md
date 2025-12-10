# SSCAN

SSCAN 命令用于迭代集合中的元素。

## 语法

```
SSCAN key cursor [MATCH pattern] [COUNT count]
```

## 参数说明

| 参数 | 说明 |
|------|------|
| key | 要迭代的集合键名 |
| cursor | 游标，第一次迭代使用 0，之后使用上次调用返回的游标 |
| MATCH pattern | 可选参数，指定返回值的模式匹配 |
| COUNT count | 可选参数，指定每次迭代返回的元素数量，默认为 10 |

## 返回值

数组回复：包含两个元素的数组。
- 第一个元素是整数，表示下次迭代的新游标，如果为 0 表示迭代已完成。
- 第二个元素是数组，包含本次迭代返回的元素。

## 示例

### 基本使用

```
redis> SADD myset1 "Google"
(integer) 1
redis> SADD myset1 "Runoob"
(integer) 1
redis> SADD myset1 "Taobao"
(integer) 1
redis> SSCAN myset1 0
1) "0"
2) 1) "Google"
   2) "Runoob"
   3) "Taobao"
```

### 使用 MATCH 参数

```
redis> SSCAN myset1 0 MATCH R*
1) "0"
2) 1) "Runoob"
```

### 使用 COUNT 参数

```
redis> SSCAN myset1 0 COUNT 2
1) "2"
2) 1) "Google"
   2) "Runoob"
```

## 使用场景

SSCAN 命令适用于需要遍历大型集合而不阻塞服务器的情况。它允许渐进式地迭代集合中的元素，避免一次性获取大量数据造成的性能问题。