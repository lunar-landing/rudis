# HINCRBY

为哈希表中的字段值加上指定增量值。

如果哈希表的 key 不存在，一个新的哈希表被创建并执行 HINCRBY 命令。

如果指定的字段不存在，那么在执行命令前，字段的值被初始化为 0 。

对一个储存字符串值的字段执行 HINCRBY 命令将造成一个错误。

本操作的值被限制在 64 位(bit)有符号数字表示之内。

## 语法

```
HINCRBY KEY_NAME FIELD_NAME INCR_BY_NUMBER
```

## 返回值

执行 HINCRBY 命令之后，哈希表中字段的值。

## 示例

```
redis> HSET myhash field1 5
(integer) 1
redis> HINCRBY myhash field1 1
(integer) 6
redis> HINCRBY myhash field1 -1
(integer) 5
redis> HINCRBY myhash field1 -10
(integer) -5
```