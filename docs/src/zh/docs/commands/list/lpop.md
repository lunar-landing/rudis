# LPOP

Redis Lpop 命令用于移除并返回列表的第一个元素。

## 语法

```
LPOP key [count]
```

## 返回值

列表的第一个元素。当列表键不存在时，返回 nil。