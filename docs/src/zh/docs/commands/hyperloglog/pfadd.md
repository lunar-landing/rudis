# PFADD

PFADD 命令用于向指定 key 的 HyperLogLog 数据结构添加元素。

HyperLogLog 是概率型数据结构：它会用较小且固定的内存开销来 **估算** 唯一元素数量。

## 语法

```
PFADD key [element [element ...]]
```

## 参数

- `key` - HyperLogLog 键名
- `element` - 一个或多个待添加的元素

## 返回值

整数回复：

- `1`：如果至少有一个 HyperLogLog 内部寄存器被更新
- `0`：如果没有任何寄存器被更新

特殊情况：

- `PFADD key`（不带元素）如果 `key` 不存在，会创建一个空的 HyperLogLog 并返回 `1`；如果 `key` 已存在，则不做任何操作并返回 `0`。

## 示例

```
redis> PFADD hll1 element1
(integer) 1
redis> PFADD hll1 element2 element3 element4
(integer) 1
redis> PFADD hll1 element1 element2
(integer) 0
redis> PFADD hll1 element5
(integer) 1
```

