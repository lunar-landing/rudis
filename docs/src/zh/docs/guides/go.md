# 在 Go 中使用 Rudis
本指南介绍了如何在 Go (Golang) 应用程序中使用 Rudis，使用 go-redis 库。

## 安装
### Go Modules
在项目根目录下运行：
```bash
go get github.com/redis/go-redis/v9
```
## 基本用法
```Go
package main

import (
    "context"
    "fmt"
    
    "github.com/redis/go-redis/v9"
)

var ctx = context.Background()

func main() {
    // 创建客户端
    rdb := redis.NewClient(&redis.Options{
        Addr:     "127.0.0.1:6379",
        Password: "", // 如果没有密码则留空
        DB:       0,  // 使用默认 DB
    })

    // 设置键
    err := rdb.Set(ctx, "key", "value", 0).Err()
    if err != nil {
        panic(err)
    }

    // 获取键
    val, err := rdb.Get(ctx, "key").Result()
    if err != nil {
        panic(err)
    }
    fmt.Println("值:", val)
}
```

## 连接管理 (连接池)
go-redis 客户端内部自动实现了连接池，只需要在初始化 Options 时进行配置，无需手动获取/释放资源。
```go
rdb := redis.NewClient(&redis.Options{
    Addr:         "127.0.0.1:6379",
    PoolSize:     10, // 最大连接数
    MinIdleConns: 1,  // 最小空闲连接数
})
// 客户端是线程安全的，可以在协程间共享
```

## 错误处理
Go 使用显式的错误检查：
```go
val, err := rdb.Get(ctx, "non_existent_key").Result()
if err == redis.Nil {
    fmt.Println("键不存在")
} else if err != nil {
    fmt.Printf("连接或其他错误: %v\n", err)
} else {
    fmt.Println("值:", val)
}
```

## 高级用法
### 管道 (Pipeline)
```go
pipe := rdb.Pipeline()

pipe.Set(ctx, "key1", "value1", 0)
pipe.Set(ctx, "key2", "value2", 0)
pipe.Get(ctx, "key1")

cmds, err := pipe.Exec(ctx)
if err != nil {
    panic(err)
}
// 处理 cmds 结果
```
