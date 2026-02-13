# 在 PHP 中使用 Rudis

本指南介绍了如何在 PHP 应用程序中使用 Rudis，使用 Predis 客户端库。

## 安装

### Composer

在项目根目录下运行：

```bash
composer require predis/predis
```

## 基本用法

```php
<?php
require 'vendor/autoload.php';

use Predis\Client;

// 创建客户端
$client = new Client([
    'scheme' => 'tcp',
    'host'   => '127.0.0.1',
    'port'   => 6379,
]);

// 设置键
$client->set('key', 'value');

// 获取键
$val = $client->get('key');
echo "值: " . $val . "\n";
```

## 连接管理（密码与数据库）

如需使用密码或选择数据库，可在配置中指定：

```php
$client = new Client([
    'scheme'   => 'tcp',
    'host'     => '127.0.0.1',
    'port'     => 6379,
    'password' => null,  // 如果没有密码则设为 null
    'database' => 0,     // 使用默认 DB
]);
```

## 错误处理

PHP 使用 try-catch 进行错误处理：

```php
try {
    $val = $client->get('non_existent_key');
    if ($val === null) {
        echo "键不存在\n";
    } else {
        echo "值: " . $val . "\n";
    }
} catch (\Predis\Connection\ConnectionException $e) {
    echo "连接错误: " . $e->getMessage() . "\n";
} catch (\Exception $e) {
    echo "其他错误: " . $e->getMessage() . "\n";
}
```

## 高级用法

### 管道 (Pipeline)

```php
$responses = $client->pipeline(function ($pipe) {
    $pipe->set('key1', 'value1');
    $pipe->set('key2', 'value2');
    $pipe->get('key1');
});

// 处理 $responses 结果
foreach ($responses as $response) {
    var_dump($response);
}
```

### 事务 (Transaction)

```php
$responses = $client->transaction(function ($tx) {
    $tx->set('key1', 'value1');
    $tx->set('key2', 'value2');
    $tx->get('key1');
});

// 处理事务执行结果
```
