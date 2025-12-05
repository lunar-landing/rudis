---
title: 备份与恢复
titleTemplate: 高级
description: Rudis 数据持久化机制详解
---

# 备份与恢复

Rudis 如何将数据写入磁盘。

持久化是指将数据写入持久性存储设备，如固态硬盘 (SSD)。Rudis 提供了一系列持久化选项，确保数据的安全性和可恢复性。

## RDB 快照

默认情况下，Rudis 会将数据集的快照保存到磁盘上的二进制文件中，文件名为 dump.rdb。您可以配置 Rudis，使其在数据集发生至少 M 次更改的情况下，每 N 秒自动保存一次数据集。

例如，以下配置将使 Rudis 每 60 秒自动将数据集转储到磁盘：

```
save 60 1
```

这种策略被称为快照（snapshotting）。

```
dbfilename dump.rdb
```

默认情况下，数据将保留在 Rudis 安装目录中的 dump.rdb 文件中，您可以通过 dbfilename 配置项来配置和修改位置。

RDB 文件采用二进制格式存储，具有文件体积小、恢复速度快的特点，适合用于备份和灾难恢复。

## AOF 日志

AOF（Append Only File）是一种替代的、完全持久化的策略。从 Rudis 1.0.0 版本开始提供。

您可以在配置文件中启用 AOF：

```
appendonly yes
```

从现在开始，每当 Rudis 接收到更改数据集的命令（例如 SET）时，它都会将其追加到 AOF 文件中。当您重新启动 Rudis 时，它将重新播放 AOF 文件以重建状态。

```
appendfilename ./data/appendonly.aof
```

默认情况下，数据将持久化到 Rudis 安装目录中的 appendonly.aof 文件中，您可以通过 appendfilename 配置项来配置和修改位置。

AOF 文件记录了所有写操作命令，具有数据完整性高的特点，适合对数据安全性要求较高的场景。

### AOF 同步策略

Rudis 提供了三种 AOF 同步策略，可通过 appendfsync 配置项设置：

- `always`：每次写操作都同步到磁盘，数据最安全但性能最低
- `everysec`：每秒同步一次，平衡了性能和数据安全性
- `no`：不主动同步，由操作系统决定何时同步，性能最高但数据安全性最低

默认配置为 `everysec`，在性能和数据安全性之间取得了良好的平衡。

## 持久化配置示例

以下是一个典型的持久化配置示例：

```conf
# RDB 快照配置
# 900秒内至少有1个键被更改则触发快照
save 900 1
# 300秒内至少有10个键被更改则触发快照
save 300 10
# 60秒内至少有10000个键被更改则触发快照
save 60 10000

# RDB 文件名
dbfilename data/dump.rdb

# 启用 AOF 持久化
appendonly yes

# AOF 文件名
appendfilename data/dump.aof

# AOF 同步策略
appendfsync everysec
```

通过合理配置这两种持久化机制，您可以根据应用需求在性能和数据安全性之间找到最佳平衡点。