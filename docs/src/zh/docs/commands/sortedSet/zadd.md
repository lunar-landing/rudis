# ZADD

Redis Zadd 命令用于向有序集合中添加一个或多个成员元素及其分数值。

## 语法

```
ZADD key score member [score member ...]
```

## 返回值

成功添加的新成员数量，不包括那些已被更新或已存在的成员。