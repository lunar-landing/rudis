---
title: 键命令
titleTemplate: 命令
description: Rudis 键命令概述，包括 DEL、EXISTS、EXPIRE、EXPIREAT、KEYS、MOVE、PERSIST、PEXPIRE、PEXPIREAT、PTTL、RANDOMKEY、RENAME、RENAMENX、SCAN 和 TYPE 命令。
---

# 键命令

键命令用于管理 Redis 中的键。这些命令允许您创建、删除、检查和修改键及其属性，如过期时间。

## 命令列表

<div class="command-cards">
  <a href="./key/del" class="command-card">
    <div class="card-title">DEL</div>
    <div class="card-description">删除指定的键</div>
  </a>
  <a href="./key/exists" class="command-card">
    <div class="card-title">EXISTS</div>
    <div class="card-description">检查给定的一个或多个键是否存在</div>
  </a>
  <a href="./key/expire" class="command-card">
    <div class="card-title">EXPIRE</div>
    <div class="card-description">为给定的键设置过期时间（以秒为单位）</div>
  </a>
  <a href="./key/expireat" class="command-card">
    <div class="card-title">EXPIREAT</div>
    <div class="card-description">为给定的键设置过期时间戳（以秒为单位的 Unix 时间戳）</div>
  </a>
  <a href="./key/keys" class="command-card">
    <div class="card-title">KEYS</div>
    <div class="card-description">查找所有符合给定模式的键</div>
  </a>
  <a href="./key/move" class="command-card">
    <div class="card-title">MOVE</div>
    <div class="card-description">将指定的键从当前数据库移动到指定编号的数据库</div>
  </a>
  <a href="./key/persist" class="command-card">
    <div class="card-title">PERSIST</div>
    <div class="card-description">移除给定键的过期时间，使键成为持久化的键</div>
  </a>
  <a href="./key/pexpire" class="command-card">
    <div class="card-title">PEXPIRE</div>
    <div class="card-description">为给定的键设置过期时间（以毫秒为单位）</div>
  </a>
  <a href="./key/pexpireat" class="command-card">
    <div class="card-title">PEXPIREAT</div>
    <div class="card-description">为给定的键设置过期时间戳（以毫秒为单位的 Unix 时间戳）</div>
  </a>
  <a href="./key/pttl" class="command-card">
    <div class="card-title">PTTL</div>
    <div class="card-description">类似于 TTL 命令，但以毫秒为单位返回键的剩余生存时间</div>
  </a>
  <a href="./key/randomkey" class="command-card">
    <div class="card-title">RANDOMKEY</div>
    <div class="card-description">从当前数据库中随机返回一个键</div>
  </a>
  <a href="./key/rename" class="command-card">
    <div class="card-title">RENAME</div>
    <div class="card-description">将键重命名为新键</div>
  </a>
  <a href="./key/renamenx" class="command-card">
    <div class="card-title">RENAMENX</div>
    <div class="card-description">仅在新键不存在时，将键重命名为新键</div>
  </a>
  <a href="./key/scan" class="command-card">
    <div class="card-title">SCAN</div>
    <div class="card-description">增量迭代数据库中的键</div>
  </a>
  <a href="./key/ttl" class="command-card">
    <div class="card-title">TTL</div>
    <div class="card-description">返回键的剩余生存时间（以秒为单位）</div>
  </a>
  <a href="./key/type" class="command-card">
    <div class="card-title">TYPE</div>
    <div class="card-description">返回存储在键中的值的类型</div>
  </a>
</div>

## 使用场景

键命令是 Redis 中最基础和最重要的命令之一。它们用于管理键的生命周期，包括创建、查询、更新和删除键。EXPIRE 和 TTL 相关命令特别适用于缓存场景，可以自动清理过期数据。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。