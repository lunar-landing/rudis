# Using Rudis in Python

This guide explains how to use Rudis in Python applications with the redis-py client library.

## Installation

### pip

Run the following in your terminal:

```bash
pip install redis
```

## Basic Usage

```python
import redis

# Create client
r = redis.Redis(
    host='127.0.0.1',
    port=6379,
    db=0,
    password=None  # Set to None if no password
)

# Set key
r.set('key', 'value')

# Get key
val = r.get('key')
print('Value:', val.decode() if val else None)
```

## Connection Management (Connection Pool)

redis-py uses a connection pool by default. Configure it when creating the client:

```python
pool = redis.ConnectionPool(
    host='127.0.0.1',
    port=6379,
    db=0,
    max_connections=10,   # Max number of connections
    decode_responses=True  # Auto-decode bytes to strings
)
r = redis.Redis(connection_pool=pool)
# Client is thread-safe and can be shared across threads
```

## Error Handling

Python uses try-except for error handling:

```python
try:
    val = r.get('non_existent_key')
    if val is None:
        print('Key does not exist')
    else:
        print('Value:', val)
except redis.ConnectionError as e:
    print('Connection error:', e)
except redis.RedisError as e:
    print('Redis error:', e)
```

## Advanced Usage

### Pipeline

```python
pipe = r.pipeline()

pipe.set('key1', 'value1')
pipe.set('key2', 'value2')
pipe.get('key1')

results = pipe.execute()
# Process results
```

### Transaction

```python
pipe = r.pipeline(transaction=True)

pipe.set('key1', 'value1')
pipe.set('key2', 'value2')
pipe.get('key1')

results = pipe.execute()
# Process transaction results
```
