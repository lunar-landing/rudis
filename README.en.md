<div align="center">

<br />

<img src="./logo/logo.png" height="80"/>

<br />

[ROADMAP 2025](https://github.com/sleeprite/rudis/issues/11)

[Github](https://github.com/sleeprite/rudis) | [Gitee](https://gitee.com/Jmysy/rudis) | [Packages](./release) | [Docker](https://github.com/sleeprite/rudis/blob/master/docker/README.md) 

<a href='https://gitee.com/rudis/rudis/stargazers'><img src='https://gitee.com/rudis/rudis/badge/star.svg?theme=gvp' alt='star'/></a>
<a href="https://gitcode.com/rudis/rudis/stargazers"><img src="https://gitcode.com/rudis/rudis/star/badge.svg"/></a>
<a href="https://github.com/sleeprite/rudis"><img src="https://img.shields.io/github/stars/sleeprite/rudis?style=flat-square&logo=GitHub"/></a>
<a href="https://github.com/sleeprite/rudis/blob/master/LICENSE"><img src="https://img.shields.io/github/license/sleeprite/rudis.svg?style=flat-square"/></a>

<h4>High Performance In-Memory Database</h4>

**[🔶 Explore the docs »](https://sleeprite.github.io/rudis)**

</div>

## Project Introduction

Rudis is a high-performance key-value storage system written in Rust, designed to leverage Rust's advantages to reimplement Redis's core functionality, meeting users' needs for high performance, reliability, and security while ensuring compatibility with Redis APIs.

### 🌟 Features

- Cross-platform, compatible with Windows, Linux, and macOS systems.
- Compatible with data structures such as strings, sets, hashes, lists, sorted sets, and JSON.
- Provides RDB and AOF mechanisms to support data backup and recovery.
- Offers excellent processing speed and instant response capabilities.
- Concurrently creates and deletes key-values across multiple threads.
- Provides Docker deployment method.
- Compatible with RESP protocol specification.

## Changelog

### v0.2.0

- Added Scan and Ltrim commands.
- Added maxclients configuration to limit the number of client connections.
- Added appendsync configuration to configure the execution strategy for AOF persistence.
- Added Set type commands: Sscan, Sdiff, and Setrange.
- Optimized the Keys command algorithm, improving performance by 88%-99.55%.
- Fixed compatibility issues with Redis-insight's Set preview.

### v0.1.0

- Upgraded Tokio to version 1.48.0.
- Adapted to redis-rust version 1.0.0-rc.1.
- Upgraded thread model, replacing shared-everything architecture with shared-nothing architecture.
- Fixed issues preventing Redis-insight visualization tool from connecting properly.
- Added transaction functionality, supporting exec, multi, and discard commands.
- Optimized log printing in loglevel=debug mode.
- Optimized RESP parser to resolve command packet sticking issues.
- Added replication master-slave replication mode.
- Refactored RDB persistence underlying logic.
- Added numerous test cases.
- Added 43 new commands.

## Quick Start

```

         /\_____/\
        /  o   o  \          Rudis 0.1.0
       ( ==  ^  == )
        )         (          Bind: 6379 PID: 40252
       (           )
      ( (  )   (  ) )        Role: master
     (__(__)___(__)__)

    Rudis is a high-performance in memory database.
    
⣷ [████████████████████████████████████████] 200000/200000 (100%) Status: Completed

[2025-12-03T03:49:43Z INFO  rudis_server::server] Server initialized
[2025-12-03T03:49:43Z INFO  rudis_server::server] Ready to accept connections
```

### Standard Installation

Based on system environment requirements, [download](./release) the matching Rudis version.

Start the Rudis service using conventional system commands:

```sh 
// Windows standard startup
start rudis-server.exe

// Windows configuration file startup
start rudis-server.exe --config ./config/rudis.conf

// Windows parameterized startup
start rudis-server.exe --port 6379
```

### Container Installation

Start the Rudis service via Docker container:

For more installation commands, please visit [docker/README.md](./docker/README.md):

```sh 
// Docker standard startup
docker run -p 6379:6379 ghcr.io/sleeprite/rudis:latest

// Docker parameterized startup
docker run -p 6379:8848 ghcr.io/sleeprite/rudis:latest --port 8848
```

## Configuration Description

- Configuration file (config): Specifies the Rudis configuration file path.
- Host address to bind (bind): Specifies the Rudis server binding address.
- Port (port): Rudis server listening port, default is 6379.
- Password (password): Sets the Rudis access password.
- Number of databases (databases): Number of Rudis databases, default is 16.
- Data persistence directory (dir): Directory for storing RDB and AOF files, default is "./".
- Persistence log path (appendfilename): Path for AOF log file storage.
- Enable persistence (appendonly): Whether to enable AOF persistence.
- Data filename (dbfilename): RDB persistence filename, default is "dump.rdb".
- Session limit (maxclients): Maximum number of client connections, default is 1000.
- Scheduled task frequency (hz): Frequency of scheduled task execution, default is 10 times/second.
- RDB save policy (save): Sets conditions for automatic RDB saving.

## Network Architecture

![alt text](./images/image.png)

## Project Structure

### cmds

The Cmds package is a component written in Rust that simulates the Rudis server, primarily responsible for implementing Rudis protocol parsing, database operation execution, and related result responses. This package internally contains implementations for different Rudis commands such as SELECT, GET, SET, etc. Its core functionality is to parse command requests from clients according to Rudis protocol specifications, execute corresponding operations on the simulated Rudis database, and return results to clients. By implementing various Rudis command handlers, it achieves complete support for the Rudis protocol and provides a simple yet effective strategy for handling different types of commands.

### network

The Network module is the core network communication component of Rudis, responsible for handling client connections, session management, and network data transmission. Built on the Tokio asynchronous runtime, this module provides high-performance TCP connection handling and concurrent connection support. Through Connection, it encapsulates read/write operations of the underlying TCP stream; Session manages client session states; SessionManager provides thread-safe session storage and retrieval; and SessionRole defines different types of client roles. The entire module adopts an asynchronous non-blocking design concept, capable of effectively handling a large number of concurrent connections, ensuring high performance and stability of the server at the network level.

### persistence

The Persistence module provides two persistence mechanisms: AOF (Append-Only File) and RDB (Rudis Database), which together ensure data persistence and consistency of the Rudis database. The AOF mechanism records each write operation and appends them to the AOF file, achieving continuous data updates and integrity. This mechanism is crucial for data accuracy and reliability, especially for data recovery after system failures or restarts.

### store

The Store module is Rudis's core in-memory database engine, providing high-performance key-value storage functionality. This module implements multiple data structures including strings, hash tables, lists, sets, and sorted sets, supporting rich data operation commands. Through thread-safe design and efficient memory management mechanisms, the Store module can provide stable read-write performance in high-concurrency environments. Additionally, this module has built-in advanced features such as key expiration time management and lazy deletion, ensuring data consistency and system stability.

### args

The Args module is Rudis's command-line argument and configuration file parser, responsible for handling various configuration options during server startup. Based on the clap library, this module supports rich command-line arguments and configuration file loading functions, flexibly configuring server parameters including network binding, port settings, authentication passwords, persistence options, and database counts. Through an intelligent configuration merging mechanism, command-line arguments take precedence over configuration files, ensuring configuration flexibility and override capability.

### command

The Command module is Rudis's command parsing and dispatch center, responsible for parsing client-sent command requests into specific command objects and dispatching them to corresponding processors for execution. This module implements a complete Redis command system, supporting data structure operation commands such as strings, hashes, lists, sets, and sorted sets, as well as advanced function commands such as server management, transaction processing, and master-slave replication. Through a unified command parsing interface, it can convert command frames in RESP protocol format into internal command objects and decide whether to persist to AOF files or propagate to slave nodes based on command types.

### frame

The Frame module is the core component responsible for handling the RESP (Redis Serialization Protocol) in Rudis, defining the data structure of command frames and providing complete serialization and deserialization functions. This module supports multiple RESP data types such as Simple String, Bulk String, Integer, Array, Error, and Null, accurately parsing command requests from clients and converting them into internally processable data structures. The Frame module particularly implements a packet sticking handling mechanism that can effectively handle multiple command frames that may stick together during network transmission, ensuring correct command parsing and execution. Through efficient encoding and decoding implementation, this module ensures efficient and stable communication between Redis clients and servers.

### replication

The Replication module implements Rudis's master-slave replication functionality, allowing one or more slave nodes to connect to the master node to achieve data synchronization and high availability. This module supports the complete master-slave replication process, including connection establishment, handshaking, full synchronization, and incremental synchronization. Master-slave node connection and data synchronization are achieved through the PSYNC command, supporting disconnection reconnection and incremental data transmission, effectively reducing network bandwidth consumption and synchronization time. During full synchronization, the master node generates an RDB snapshot file and transmits it to the slave node, while in the incremental synchronization phase, write commands are propagated in real-time. This module also maintains replication connection state management, ensuring the stability of master-slave relationships and data consistency, providing a solid foundation for building highly available Rudis clusters.

### server

The Server module is Rudis's core entry point, responsible for the entire server's startup, configuration parsing, and client request processing. It integrates functional modules such as network communication, database management, persistence, and replication, constituting a complete Rudis server implementation.

## Common Commands

echo command
```
127.0.0.1:6379> echo helloword
helloword
```

ping command
```
127.0.0.1:6379> ping
PONG
```

set command
```
127.0.0.1:6379> set user bailiang
OK
```

get command
```
127.0.0.1:6379> get user
bailiang
```

del command
```
127.0.0.1:6379> del username
(integer) 1
127.0.0.1:6379> del username password
(integer) 2
```

exists command
```
127.0.0.1:6379> exists user
(integer) 0
```

keys command
```
127.0.0.1:6379> keys *
(empty list or set)
```

auth command
```
127.0.0.1:6379> auth 123456
OK
```

expire command
```
127.0.0.1:6379> expire user 10000
(integer) 0
```

select command
```
127.0.0.1:6379> select 1
OK
```

dbsize command
```
127.0.0.1:6379> dbsize
(integer) 2
```

append command
```
127.0.0.1:6379> append user bailiang
(integer) 10
```

move command
```
127.0.0.1:6379> move user 0
OK
```

> For detailed information about transaction functionality, please refer to [Transaction Function Description](README-TRANSACTIONS.md)

## Building from Source

If you want to build from source to get the distribution package, please use common cargo commands:

```
// Standard startup
cargo run

// Parameterized startup
cargo run -- --port 8848
cargo run -- --save 20/1 60/2

// Specify configuration
cargo run -- --config rudis.conf

// Build program
cargo build

cargo build --release --target=x86_64-unknown-linux-musl

cargo build --release

// Code checking
cargo clippy
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=sleeprite/rudis&type=Date)](https://www.star-history.com/#sleeprite/rudis&Date)

## Open Source Collaboration

The Rudis project follows the [GNU GENERAL PUBLIC LICENSE](https://github.com/sleeprite/rudis/blob/master/LICENSE) open-source license. Thanks to these excellent [Contributors](https://github.com/sleeprite/rudis/graphs/contributors).

<a href="https://github.com/sleeprite/rudis/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=sleeprite/rudis" />
</a>