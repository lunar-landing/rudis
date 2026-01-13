---
title: 有序集合命令
titleTemplate: 命令
description: Rudis 有序集合命令概述，包括 ZADD、ZCARD、ZCOUNT、ZRANK、ZREM 和 ZSCORE 命令。
---

# 有序集合命令

有序集合命令类似于集合，但每个成员都关联一个分数（score），用于排序。成员是唯一的，但分数可以重复。有序集合按分数从小到大排序。

## 命令列表

<div class="command-cards">
  <a href="./sortedSet/zadd" class="command-card">
    <div class="card-title">ZADD</div>
    <div class="card-description">向有序集合中添加一个或多个成员，或更新已存在成员的分数</div>
  </a>
  <a href="./sortedSet/zcard" class="command-card">
    <div class="card-title">ZCARD</div>
    <div class="card-description">返回有序集合中元素的数量</div>
  </a>
  <a href="./sortedSet/zcount" class="command-card">
    <div class="card-title">ZCOUNT</div>
    <div class="card-description">计算在有序集合中指定分数范围内的成员数量</div>
  </a>
  <a href="./sortedSet/zrank" class="command-card">
    <div class="card-title">ZRANK</div>
    <div class="card-description">返回有序集合中指定成员的排名（从0开始）</div>
  </a>
  <a href="./sortedSet/zrem" class="command-card">
    <div class="card-title">ZREM</div>
    <div class="card-description">移除有序集合中的一个或多个成员</div>
  </a>
  <a href="./sortedSet/zscore" class="command-card">
    <div class="card-title">ZSCORE</div>
    <div class="card-description">返回有序集合中指定成员的分数</div>
  </a>
  <a href="./sortedSet/zincrby" class="command-card">
    <div class="card-title">ZINCRBY</div>
    <div class="card-description">为有序集合中指定成员的分数加上增量 increment</div>
  </a>
</div>

## 使用场景

有序集合命令非常适合用于排行榜、时间线、带权重的队列等场景。通过分数可以轻松实现排序功能，同时还能快速查询特定成员的排名和分数。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。