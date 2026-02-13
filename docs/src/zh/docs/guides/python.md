# 在 Python 中使用 Rudis

本指南介绍了如何在 Python 应用程序中使用 Rudis，使用 redis-py 客户端库。

## 安装

### pip

在终端中运行：

```bash
pip install redis
```

## 基本用法

```python
import redis

# 创建客户端
r = redis.Redis(
    host='127.0.0.1',
    port=6379,
    db=0,
    password=None  # 如果没有密码则设为 None
)

# 设置键
r.set('key', 'value')

# 获取键
val = r.get('key')
print('值:', val.decode() if val else None)
```

## 连接管理（连接池）

redis-py 默认使用连接池，可在创建客户端时配置：

```python
pool = redis.ConnectionPool(
    host='127.0.0.1',
    port=6379,
    db=0,
    max_connections=10,  # 最大连接数
    decode_responses=True  # 自动将字节解码为字符串
)
r = redis.Redis(connection_pool=pool)
# 客户端是线程安全的，可以在多线程间共享
```

## 错误处理

Python 使用 try-except 进行错误处理：

```python
try:
    val = r.get('non_existent_key')
    if val is None:
        print('键不存在')
    else:
        print('值:', val)
except redis.ConnectionError as e:
    print('连接错误:', e)
except redis.RedisError as e:
    print('Redis 错误:', e)
```

## 高级用法

### 管道 (Pipeline)

```python
pipe = r.pipeline()

pipe.set('key1', 'value1')
pipe.set('key2', 'value2')
pipe.get('key1')

results = pipe.execute()
# 处理 results 结果
```

### 事务 (Transaction)

```python
pipe = r.pipeline(transaction=True)

pipe.set('key1', 'value1')
pipe.set('key2', 'value2')
pipe.get('key1')

results = pipe.execute()
# 处理事务执行结果
```
