---
title: 字符串命令
titleTemplate: 命令
description: Rudis 字符串命令概述，包括 APPEND、DECR、DECRBY、GET、GETRANGE、GETSET、INCR、INCRBY、INCRBYFLOAT、MGET、MSET、SET、SETRANGE 和 STRLEN 命令。
---

# 字符串命令

字符串命令是最基本的数据类型命令，可以存储字符串、整数或浮点数。字符串最大可以存储 512 MB 的数据。

## 命令列表

<div class="command-cards">
  <a href="./string/append" class="command-card">
    <div class="card-title">APPEND</div>
    <div class="card-description">如果键已经存在并且是一个字符串，追加值到键的末尾</div>
  </a>
  <a href="./string/decr" class="command-card">
    <div class="card-title">DECR</div>
    <div class="card-description">将键中存储的数字值减一</div>
  </a>
  <a href="./string/decrby" class="command-card">
    <div class="card-title">DECRBY</div>
    <div class="card-description">将键中存储的数字值减去指定的数值</div>
  </a>
  <a href="./string/get" class="command-card">
    <div class="card-title">GET</div>
    <div class="card-description">获取指定键的值</div>
  </a>
  <a href="./string/getrange" class="command-card">
    <div class="card-title">GETRANGE</div>
    <div class="card-description">返回键中字符串值的子字符</div>
  </a>
  <a href="./string/getset" class="command-card">
    <div class="card-title">GETSET</div>
    <div class="card-description">将给定键的值设为新值，并返回键的旧值</div>
  </a>
  <a href="./string/incr" class="command-card">
    <div class="card-title">INCR</div>
    <div class="card-description">将键中存储的数字值增一</div>
  </a>
  <a href="./string/incrby" class="command-card">
    <div class="card-title">INCRBY</div>
    <div class="card-description">将键中存储的数字值加上指定的数值</div>
  </a>
  <a href="./string/incrbyfloat" class="command-card">
    <div class="card-title">INCRBYFLOAT</div>
    <div class="card-description">将键中存储的数字值加上指定的浮点数</div>
  </a>
  <a href="./string/mget" class="command-card">
    <div class="card-title">MGET</div>
    <div class="card-description">获取所有给定键的值</div>
  </a>
  <a href="./string/mset" class="command-card">
    <div class="card-title">MSET</div>
    <div class="card-description">同时设置一个或多个键值对</div>
  </a>
  <a href="./string/set" class="command-card">
    <div class="card-title">SET</div>
    <div class="card-description">设置指定键的值</div>
  </a>
  <a href="./string/setrange" class="command-card">
    <div class="card-title">SETRANGE</div>
    <div class="card-description">用指定的字符串覆盖给定 key 所储存的字符串值，覆盖的位置从偏移量 offset 开始</div>
  </a>
  <a href="./string/strlen" class="command-card">
    <div class="card-title">STRLEN</div>
    <div class="card-description">返回键所存储的字符串值的长度</div>
  </a>
</div>

## 使用场景

字符串命令是最常用的数据类型命令，适用于各种场景，如缓存、计数器、分布式锁等。通过 INCR 和 DECR 系列命令可以轻松实现计数器功能。

详细了解每个命令的用法和参数，请参阅上述链接的各个命令文档。