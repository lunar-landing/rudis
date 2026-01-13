# ZINCRBY

为有序集合中指定成员的分数加上增量 increment。

如果指定成员不在有序集合中，则会将其添加到有序集合中，并将其分数设置为 increment（就像之前的分数为 0.0 一样）。如果 key 不存在，则会创建一个新的有序集合，并将指定成员作为唯一成员。

当 key 存在但不持有有序集合时，会返回错误。

分数值应该是数值的字符串表示形式，接受双精度浮点数。可以提供负值来减少分数。

## 语法

```
ZINCRBY key increment member
```

## 返回值

批量字符串回复：成员的新分数（双精度浮点数），以字符串形式表示。

## 示例

```
redis> ZADD myzset 1 "one"
(integer) 1
redis> ZADD myzset 2 "two"
(integer) 1
redis> ZINCRBY myzset 2 "one"
"3"
redis> ZINCRBY myzset -1 "two"
"1"
redis> ZINCRBY myzset 1 "three"
"1"
```