# SET

Rudis SET 命令用于设置给定键的值。如果键已经存储了其他值，SET 将覆盖旧值并忽略类型。

## 语法

```
SET key value [NX | XX] [EX seconds | PX milliseconds ]
```

## 选项

SET 命令支持一组修改其行为的选项：

- EX seconds -- 设置指定的过期时间，以秒为单位（正整数）。
- PX milliseconds -- 设置指定的过期时间，以毫秒为单位（正整数）。
- NX -- 只有当键不存在时才设置键。
- XX -- 只有当键已存在时才设置键。

## 返回值

简单字符串回复：如果 SET 正确执行则返回 OK。

空回复：如果由于用户指定了 NX 或 XX 选项但条件不满足而导致 SET 操作未执行，则返回 (nil)。