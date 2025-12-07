---
title: 安装
description: Rudis 安装指南
lastUpdated: false
editLink: false
prev: false
next: false
---

# 安装

本指南介绍如何在 Linux、macOS 和 Windows 系统上安装并运行 Rudis 服务器。

## 普通安装

根据系统环境要求，[下载](https://github.com/sleeprite/rudis/tree/master/release) 匹配的 Rudis 版本

通过系统常规命令启动 Rudis 服务

### Windows 系统

```sh
// windows 常规启动
start rudis-server.exe

// windows 配置文件启动
start rudis-server.exe --config ./config/rudis.conf

// windows 指定参数启动
start rudis-server.exe --port 6379
```


### Macos 系统

```sh
// macos 常规启动
./rudis-server

// macos 配置文件启动
./rudis-server --config ./config/rudis.conf

// macos 指定参数启动
./rudis-server --port 6379
```

### Linux 系统

```sh
// linux 常规启动
./rudis-server

// linux 配置文件启动
./rudis-server --config ./config/rudis.conf

// linux 指定参数启动
./rudis-server --port 6379
```

## 容器安装

通过 docker 容器启动 Rudis 服务

```sh
// docker 常规启动
docker run -p 6379:6379 ghcr.io/sleeprite/rudis:latest

// docker 指定参数启动
docker run -p 6379:8848 ghcr.io/sleeprite/rudis:latest --port 8848
```

## 源码构建

如果您希望通过构建源码的方式得到发行包，请使用 cargo 常用命令。

```sh
// 普通启动
cargo run

// 带参启动
cargo run -- --port 8848
cargo run -- --save 20/1 60/2

// 指定配置
cargo run -- --config rudis.conf

// 构建程序
cargo build

cargo build --release --target=x86_64-unknown-linux-musl

cargo build --release

// 代码检测
cargo clippy
```

<!-- 
<AddRepoButton/>

<ExtensionsWrapper/>

<script setup>
import AddRepoButton from '@theme/components/AddRepoButton.vue'
import ExtensionsWrapper from '@theme/components/Extensions/ExtensionsWrapper.vue'
</script> -->