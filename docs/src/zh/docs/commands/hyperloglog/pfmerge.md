# PFMERGE

PFMERGE 命令用于把多个 HyperLogLog 合并到一个目标 key 中。

合并后的 HyperLogLog 近似表示所有源 HyperLogLog 所观察集合的 **并集** 的基数估计。

## 语法

```
PFMERGE destkey [sourcekey [sourcekey ...]]
```

## 参数

- `destkey` - 目标 HyperLogLog 键
- `sourcekey` - 一个或多个源 HyperLogLog 键

## 返回值

简单字符串回复：`OK`。

## 说明

- 如果 `destkey` 不存在，会创建一个空的 HyperLogLog。
- 如果某个 `sourcekey` 不存在，会按空 HyperLogLog 处理（忽略）。

## 示例

```
redis> PFADD hll1 element1 element2 element3 element4 element5
(integer) 1
redis> PFADD hll2 a b c d
(integer) 1
redis> PFMERGE hll_dest hll1 hll2
OK
redis> PFCOUNT hll_dest
(integer) 9
```

