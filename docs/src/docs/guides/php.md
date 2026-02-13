# Using Rudis in PHP

This guide explains how to use Rudis in PHP applications with the Predis client library.

## Installation

### Composer

Run the following in your project root:

```bash
composer require predis/predis
```

## Basic Usage

```php
<?php
require 'vendor/autoload.php';

use Predis\Client;

// Create client
$client = new Client([
    'scheme' => 'tcp',
    'host'   => '127.0.0.1',
    'port'   => 6379,
]);

// Set key
$client->set('key', 'value');

// Get key
$val = $client->get('key');
echo "Value: " . $val . "\n";
```

## Connection Management (Password and Database)

To use a password or select a database, specify them in the configuration:

```php
$client = new Client([
    'scheme'   => 'tcp',
    'host'     => '127.0.0.1',
    'port'     => 6379,
    'password' => null,  // Set to null if no password
    'database' => 0,     // Use default DB
]);
```

## Error Handling

PHP uses try-catch for error handling:

```php
try {
    $val = $client->get('non_existent_key');
    if ($val === null) {
        echo "Key does not exist\n";
    } else {
        echo "Value: " . $val . "\n";
    }
} catch (\Predis\Connection\ConnectionException $e) {
    echo "Connection error: " . $e->getMessage() . "\n";
} catch (\Exception $e) {
    echo "Other error: " . $e->getMessage() . "\n";
}
```

## Advanced Usage

### Pipeline

```php
$responses = $client->pipeline(function ($pipe) {
    $pipe->set('key1', 'value1');
    $pipe->set('key2', 'value2');
    $pipe->get('key1');
});

// Process $responses results
foreach ($responses as $response) {
    var_dump($response);
}
```

### Transaction

```php
$responses = $client->transaction(function ($tx) {
    $tx->set('key1', 'value1');
    $tx->set('key2', 'value2');
    $tx->get('key1');
});

// Process transaction results
```
