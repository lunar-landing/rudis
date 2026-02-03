---
title: 集合命令
titleTemplate: 命令
description: Rudis 集合命令概述，包括 SADD、SCARD、SINTER、SISMEMBER、SMEMBERS、SPOP、SRANDMEMBER、SREM、SUNION、SUNIONSTORE、SDIFFSTORE、SINTERSTORE、SMOVE 命令。
---

# 集合命令

集合命令允许您存储唯一的字符串元素。集合是无序的，不允许重复元素。支持多种集合操作，如交集、并集等。

## 命令列表

<div class="command-cards">
  <a href="./set/sadd" class="command-card">
    <div class="card-title">SADD</div>
    <div class="card-description">向集合中添加一个或多个成员</div>
  </a>
  <a href="./set/scard" class="command-card">
    <div class="card-title">SCARD</div>
    <div class="card-description">返回集合中元素的数量</div>
  </a>
  <a href="./set/sinter" class="command-card">
    <div class="card-title">SINTER</div>
    <div class="card-description">返回给定所有集合的交集</div>
  </a>
  <a href="./set/sismember" class="command-card">
    <div class="card-title">SISMEMBER</div>
    <div class="card-description">判断成员是否是集合的成员</div>
  </a>
  <a href="./set/smembers" class="command-card">
    <div class="card-title">SMEMBERS</div>
    <div class="card-description">返回集合中的所有成员</div>
  </a>
  <a href="./set/spop" class="command-card">
    <div class="card-title">SPOP</div>
    <div class="card-description">移除并返回集合中的一个随机元素</div>
  </a>
  <a href="./set/srandmember" class="command-card">
    <div class="card-title">SRANDMEMBER</div>
    <div class="card-description">返回集合中的一个或多个随机成员，但不移除</div>
  </a>
  <a href="./set/srem" class="command-card">
    <div class="card-title">SREM</div>
    <div class="card-description">移除集合中的一个或多个成员</div>
  </a>
  <a href="./set/sunion" class="command-card">
    <div class="card-title">SUNION</div>
    <div class="card-description">返回给定所有集合的并集</div>
  </a>
  <a href="./set/sunionstore" class="command-card">
    <div class="card-title">SUNIONSTORE</div>
    <div class="card-description">将给定所有集合的并集存储在指定的集合中</div>
  </a>
  <a href="./set/sdiffstore" class="command-card">
    <div class="card-title">SDIFFSTORE</div>
    <div class="card-description">计算给定所有集合的差集并存储在指定的集合中</div>
  </a>
  <a href="./set/sinterstore" class="command-card">
    <div class="card-title">SINTERSTORE</div>
    <div class="card-description">计算给定所有集合的交集并存储在指定的集合中</div>
  </a>
  <a href="./set/smove" class="command-card">
    <div class="card-title">SMOVE</div>
    <div class="card-description">将成员从一个集合移动到另一个集合</div>
  </a>
</div>

## 使用场景

集合命令非常适合用于标签系统、好友关系、去重统计等场景。由于集合的唯一性特性，可以轻松实现去重功能，而集合运算则可以方便地处理关系查询。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。