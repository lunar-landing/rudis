---
title: 哈希命令
titleTemplate: 命令
description: Rudis 哈希命令概述，包括 HDEL、HEXISTS、HGET、HGETALL、HKEYS、HLEN、HMGET、HMSET、HSET、HSETNX、HSCAN、HSTRLEN 和 HVALS 命令。
---

# 哈希命令

哈希命令允许您将键值对存储为哈希表（也称为映射或字典）。每个哈希可以存储多达 2^32 - 1 个字段-值对。

## 命令列表

<div class="command-cards">
  <a href="./hash/hdel" class="command-card">
    <div class="card-title">HDEL</div>
    <div class="card-description">删除哈希表中的一个或多个指定字段</div>
  </a>
  <a href="./hash/hexists" class="command-card">
    <div class="card-title">HEXISTS</div>
    <div class="card-description">检查哈希表中指定字段是否存在</div>
  </a>
  <a href="./hash/hget" class="command-card">
    <div class="card-title">HGET</div>
    <div class="card-description">获取存储在哈希表中指定字段的值</div>
  </a>
  <a href="./hash/hgetall" class="command-card">
    <div class="card-title">HGETALL</div>
    <div class="card-description">获取在哈希表中所有的字段和值</div>
  </a>
  <a href="./hash/hscan" class="command-card">
    <div class="card-title">HSCAN</div>
    <div class="card-description">增量迭代哈希表中的字段和值</div>
  </a>
  <a href="./hash/hkeys" class="command-card">
    <div class="card-title">HKEYS</div>
    <div class="card-description">获取哈希表中所有的字段名</div>
  </a>
  <a href="./hash/hlen" class="command-card">
    <div class="card-title">HLEN</div>
    <div class="card-description">获取哈希表中字段的数量</div>
  </a>
  <a href="./hash/hmget" class="command-card">
    <div class="card-title">HMGET</div>
    <div class="card-description">获取所有给定字段的值</div>
  </a>
  <a href="./hash/hmset" class="command-card">
    <div class="card-title">HMSET</div>
    <div class="card-description">同时将多个字段-值对设置到哈希表中</div>
  </a>
  <a href="./hash/hset" class="command-card">
    <div class="card-title">HSET</div>
    <div class="card-description">将字段-值对设置到哈希表中</div>
  </a>
  <a href="./hash/hsetnx" class="command-card">
    <div class="card-title">HSETNX</div>
    <div class="card-description">仅当字段不存在时，为哈希表中的字段赋值</div>
  </a>
  <a href="./hash/hstrlen" class="command-card">
    <div class="card-title">HSTRLEN</div>
    <div class="card-description">返回哈希表中指定字段值的字符串长度</div>
  </a>
  <a href="./hash/hvals" class="command-card">
    <div class="card-title">HVALS</div>
    <div class="card-description">返回哈希表中所有值</div>
  </a>
  <a href="./hash/hincrby" class="command-card">
    <div class="card-title">HINCRBY</div>
    <div class="card-description">为哈希表中的字段值加上指定增量值</div>
  </a>
  <a href="./hash/hincrbyfloat" class="command-card">
    <div class="card-title">HINCRBYFLOAT</div>
    <div class="card-description">为哈希表中的字段值加上指定浮点数增量值</div>
  </a>
</div>

## 使用场景

哈希命令非常适合用于表示对象，如用户资料、商品信息等。您可以将对象的所有属性存储在一个哈希中，每个属性作为一个字段。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。