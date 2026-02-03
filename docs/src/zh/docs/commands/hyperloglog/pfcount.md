# PFCOUNT

PFCOUNT 命令用于返回一个或多个 HyperLogLog 的 **基数估计值**（唯一元素数量）。

当传入多个 key 时，PFCOUNT 会在内部把多个 HyperLogLog 做合并，并返回它们的 **并集** 的基数估计值。

## 语法

```
PFCOUNT key [key ...]
```

## 参数

- `key` - 一个或多个 HyperLogLog 键

## 返回值

整数回复：估计的唯一元素数量。如果 key 不存在，则按空 HyperLogLog 处理（计数为 0）。

## 说明

- 返回值是 **估算值**，不是精确值。
- 多 key 版本需要临时合并寄存器，通常会比单 key 版本慢一些。

## 示例

```
redis> PFADD hll1 element1 element2 element3 element4 element5
(integer) 1
redis> PFADD hll2 a b c d
(integer) 1
redis> PFCOUNT hll1
(integer) 5
redis> PFCOUNT hll2
(integer) 4
redis> PFCOUNT hll1 hll2
(integer) 9
```

