# Using Rudis in Go

This guide explains how to use Rudis in Go (Golang) applications with the go-redis library.

## Installation

### Go Modules

Run the following in your project root:

```bash
go get github.com/redis/go-redis/v9
```

## Basic Usage

```go
package main

import (
    "context"
    "fmt"
    
    "github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
    // Create client
    rdb := redis.NewClient(&redis.Options{
        Addr:     "127.0.0.1:6379",
        Password: "", // Leave empty if no password
        DB:       0,  // Use default DB
    })

    // Set key
    err := rdb.Set(ctx, "key", "value", 0).Err()
    if err != nil {
        panic(err)
    }

    // Get key
    val, err := rdb.Get(ctx, "key").Result()
    if err != nil {
        panic(err)
    }
    fmt.Println("Value:", val)
}
```

## Connection Management (Connection Pool)

The go-redis client has built-in connection pooling. Configure it in the Options when initializing; no manual acquire/release is needed.

```go
rdb := redis.NewClient(&redis.Options{
    Addr:         "127.0.0.1:6379",
    PoolSize:     10, // Max number of connections
    MinIdleConns: 1,  // Min idle connections
})
// Client is goroutine-safe and can be shared across goroutines
```

## Error Handling

Go uses explicit error checking:

```go
val, err := rdb.Get(ctx, "non_existent_key").Result()
if err == redis.Nil {
    fmt.Println("Key does not exist")
} else if err != nil {
    fmt.Printf("Connection or other error: %v\n", err)
} else {
    fmt.Println("Value:", val)
}
```

## Advanced Usage

### Pipeline

```go
pipe := rdb.Pipeline()

pipe.Set(ctx, "key1", "value1", 0)
pipe.Set(ctx, "key2", "value2", 0)
pipe.Get(ctx, "key1")

cmds, err := pipe.Exec(ctx)
if err != nil {
    panic(err)
}
// Process cmds results
```
