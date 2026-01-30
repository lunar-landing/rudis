---
title: HyperLogLog 命令
titleTemplate: 命令
description: Rudis HyperLogLog 命令概述，包括 PFADD、PFCOUNT、PFMERGE。
---

# HyperLogLog 命令

HyperLogLog 是一种 **概率型** 数据结构，用于估算集合的 **基数**（唯一元素数量），并且使用固定且较小的内存开销。

- **结果为估算值**：不是精确计数，通常误差较小（Redis 风格的 HLL 典型误差约 0.81%）。
- **适用场景**：UV 统计、唯一搜索词统计、去重计数等。

## 命令列表

<div class="command-cards">
  <a href="./hyperloglog/pfadd" class="command-card">
    <div class="card-title">PFADD</div>
    <div class="card-description">向 HyperLogLog 添加元素</div>
  </a>
  <a href="./hyperloglog/pfcount" class="command-card">
    <div class="card-title">PFCOUNT</div>
    <div class="card-description">返回基数估计值</div>
  </a>
  <a href="./hyperloglog/pfmerge" class="command-card">
    <div class="card-title">PFMERGE</div>
    <div class="card-description">合并多个 HyperLogLog</div>
  </a>
</div>

## 说明

- 返回的基数是 **估算值**，不是精确值。
- 在 Redis 中 HyperLogLog 内部以字符串表示。Rudis 为了兼容性，`TYPE` 对 HyperLogLog 也会返回 `string`。

