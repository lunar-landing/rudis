# HSET

Redis Hset 命令用于为哈希表中的字段赋值。

## 语法

```
HSET key field value [field value ...]
```

## 返回值

如果字段是哈希表中新创建的字段且值设置成功，则返回 1。如果哈希表中的字段已存在且旧值已被新值覆盖，则返回 0。