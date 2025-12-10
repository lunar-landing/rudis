---
title: Key Commands
titleTemplate: Commands
description: Overview of Rudis key commands including DEL, EXISTS, EXPIRE, EXPIREAT, KEYS, MOVE, PERSIST, PEXPIRE, PEXPIREAT, PTTL, RANDOMKEY, RENAME, RENAMENX, SCAN, and TYPE commands.
---

# Key Commands

Key commands are used to manage keys in Redis. These commands allow you to create, delete, check, and modify keys and their attributes, such as expiration time.

## Command List

<div class="command-cards">
  <a href="./key/del" class="command-card">
    <div class="card-title">DEL</div>
    <div class="card-description">Deletes the specified key</div>
  </a>
  <a href="./key/exists" class="command-card">
    <div class="card-title">EXISTS</div>
    <div class="card-description">Checks if one or more given keys exist</div>
  </a>
  <a href="./key/expire" class="command-card">
    <div class="card-title">EXPIRE</div>
    <div class="card-description">Sets the expiration time for a given key (in seconds)</div>
  </a>
  <a href="./key/expireat" class="command-card">
    <div class="card-title">EXPIREAT</div>
    <div class="card-description">Sets the expiration timestamp for a given key (Unix timestamp in seconds)</div>
  </a>
  <a href="./key/keys" class="command-card">
    <div class="card-title">KEYS</div>
    <div class="card-description">Finds all keys matching the given pattern</div>
  </a>
  <a href="./key/move" class="command-card">
    <div class="card-title">MOVE</div>
    <div class="card-description">Moves the specified key from the current database to the specified database</div>
  </a>
  <a href="./key/persist" class="command-card">
    <div class="card-title">PERSIST</div>
    <div class="card-description">Removes the expiration time from a given key, making it persistent</div>
  </a>
  <a href="./key/pexpire" class="command-card">
    <div class="card-title">PEXPIRE</div>
    <div class="card-description">Sets the expiration time for a given key (in milliseconds)</div>
  </a>
  <a href="./key/pexpireat" class="command-card">
    <div class="card-title">PEXPIREAT</div>
    <div class="card-description">Sets the expiration timestamp for a given key (Unix timestamp in milliseconds)</div>
  </a>
  <a href="./key/pttl" class="command-card">
    <div class="card-title">PTTL</div>
    <div class="card-description">Similar to the TTL command, but returns the remaining survival time of a key in milliseconds</div>
  </a>
  <a href="./key/randomkey" class="command-card">
    <div class="card-title">RANDOMKEY</div>
    <div class="card-description">Randomly returns a key from the current database</div>
  </a>
  <a href="./key/rename" class="command-card">
    <div class="card-title">RENAME</div>
    <div class="card-description">Renames a key to a new key</div>
  </a>
  <a href="./key/renamenx" class="command-card">
    <div class="card-title">RENAMENX</div>
    <div class="card-description">Renames a key to a new key only if the new key does not exist</div>
  </a>
  <a href="./key/scan" class="command-card">
    <div class="card-title">SCAN</div>
    <div class="card-description">Incrementally iterates over keys in the database</div>
  </a>
  <a href="./key/ttl" class="command-card">
    <div class="card-title">TTL</div>
    <div class="card-description">Returns the remaining survival time of a key (in seconds)</div>
  </a>
  <a href="./key/type" class="command-card">
    <div class="card-title">TYPE</div>
    <div class="card-description">Returns the type of value stored in a key</div>
  </a>
</div>

## Use Cases

Key commands are among the most fundamental and important commands in Redis. They are used to manage the lifecycle of keys, including creating, querying, updating, and deleting keys. EXPIRE and TTL related commands are particularly suitable for caching scenarios, where expired data can be automatically cleaned up.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.