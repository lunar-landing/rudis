<div align="center">

<br />

<img src="./logo/logo.png" height="80"/>

<br />

[ROADMAP 2026](./ROADMAP-2026.md)

[Github](https://github.com/lunar-landing/rudis) | [Gitee](https://gitee.com/lunarlanding/rudis) | [Packages](./release) | [Docker](https://github.com/lunar-landing/rudis/blob/master/docker/README.md) 

<a href='https://gitee.com/lunarlanding/rudis/stargazers'><img src='https://gitee.com/lunarlanding/rudis/badge/star.svg?theme=gvp' alt='star'/></a>
<a href="https://gitcode.com/rudis/rudis/stargazers"><img src="https://gitcode.com/rudis/rudis/star/badge.svg"/></a>
<a href="https://github.com/lunar-landing/rudis"><img src="https://img.shields.io/github/stars/lunar-landing/rudis?style=flat-square&logo=GitHub"/></a>
<a href="https://github.com/lunar-landing/rudis/blob/master/LICENSE"><img src="https://img.shields.io/github/license/lunar-landing/rudis.svg?style=flat-square"/></a>

<h4>High Performance In-Memory Database</h4>

**[🔶 Explore the docs »](https://sleeprite.github.io/rudis)**

</div>

## Introduction

Rudis is a high-performance key-value storage system written in Rust, designed to recreate Redis's core functionality by leveraging Rust's advantages to meet user demands for high performance, reliability, and security, while maintaining compatibility with the Redis API.

### 🌟 Features

- Cross-platform, compatible with Windows, Linux, and macOS systems.
- Supports String, Set, Hash, List, Sorted Set, HyperLogLog, and JSON data structures.
- Provides RDB and AOF mechanisms to support data backup and recovery.
- Delivers exceptional processing speed and instant response capability.
- Concurrent key-value creation and deletion across multiple threads.
- Provides Docker deployment option.
- Compatible with RESP protocol specification.

## Quick Start


```

         /\_____/\
        /  o   o  \          Rudis 0.4.0
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

Download the matching Rudis version according to your system requirements from [release](./release)

Start the Rudis service using standard system commands

```sh 
// Windows standard startup
start rudis-server.exe

// Windows startup with configuration file
start rudis-server.exe --config ./config/rudis.conf

// Windows startup with specified parameters
start rudis-server.exe --port 6379
```

### Container Installation

Start Rudis service through Docker container

For more installation commands, please visit [docker/README.md](./docker/README.md)

```sh 
// Docker standard startup
docker run -p 6379:6379 ghcr.io/sleeprite/rudis:latest

// Docker startup with specified parameters
docker run -p 6379:8848 ghcr.io/sleeprite/rudis:latest --port 8848
```

## Configuration

- Configuration file (config): Specify the Rudis configuration file path.
- Bind host address (bind): Specify the Rudis server bind address.
- Port (port): Rudis server listening port, default 6379.
- Password (password): Set Rudis access password.
- Number of databases (databases): Number of Rudis databases, default 16.
- Data persistence directory (dir): RDB and AOF file storage directory, default "./".
- Persistence log path (appendfilename): AOF log file storage path.
- Enable persistence (appendonly): Whether to enable AOF persistence.
- Data filename (dbfilename): RDB persistence filename, default "dump.rdb".
- Session limit (maxclients): Maximum client connections, default 1000.
- Scheduled task frequency (hz): Scheduled task execution frequency, default 10 times/second.
- RDB save strategy (save): Set RDB automatic save conditions.

## Network Architecture

![alt text](./images/image.png)

## Project Structure

### cmds

The Cmds package is a component written in Rust that simulates a Rudis server. It is primarily responsible for parsing the Rudis protocol, executing database operations, and responding with results. The package contains implementations for various Rudis commands such as SELECT, GET, SET, and more. Its core functionality is to parse command requests from clients according to the Rudis protocol specification, execute corresponding operations on the simulated Rudis database, and return results to clients. By implementing handlers for each Rudis command, it achieves complete support for the Rudis protocol and provides a simple yet effective strategy for handling different types of commands.

### network

The Network module is the core network communication component of Rudis, responsible for handling client connections, session management, and network data transmission. Built on the Tokio async runtime, this module provides high-performance TCP connection handling capabilities and concurrent connection support. Connection encapsulates read/write operations for underlying TCP streams, Session manages client session state, SessionManager provides thread-safe session storage and retrieval, and SessionRole defines different types of client roles. The entire module adopts an async non-blocking design philosophy, enabling efficient handling of large numbers of concurrent connections and ensuring high performance and stability at the network layer.

### persistence

The Persistence module provides two persistence mechanisms: AOF (Append-Only File) and RDB (Rudis Database), which together ensure data persistence and consistency in the Rudis database. The AOF mechanism records each write operation and appends them to the AOF file, achieving continuous data updates and integrity. This mechanism is crucial for data accuracy and reliability, especially ensuring data recovery after system failures or restarts.

### store

The Store module is Rudis's core in-memory database engine, providing high-performance key-value storage functionality. This module implements multiple data structures including strings, hash tables, lists, sets, and sorted sets, supporting a rich set of data operation commands. Through thread-safe design and efficient memory management mechanisms, the Store module delivers stable read/write performance in high-concurrency environments. Additionally, this module includes advanced features such as key expiration time management and lazy deletion, ensuring data consistency and system stability.

### args

The Args module is Rudis's command-line argument and configuration file parser, responsible for handling various configuration options during server startup. Built on the clap library, this module supports rich command-line parameters and configuration file loading capabilities, enabling flexible configuration of server parameters including network binding, port settings, authentication passwords, persistence options, and database count. Through intelligent configuration merging mechanisms, command-line arguments take precedence over configuration files, ensuring configuration flexibility and overridability.

### command

The Command module is Rudis's command parsing and dispatch center, responsible for parsing command requests sent by clients into specific command objects and dispatching them to appropriate handlers for execution. This module implements a complete Redis command system, supporting operation commands for data structures such as strings, hashes, lists, sets, and sorted sets, as well as advanced functionality commands including server management, transaction processing, and master-slave replication. Through a unified command parsing interface, it converts RESP protocol format command frames into internal command objects and determines whether persistence to AOF files or propagation to slave nodes is needed based on command type.

### frame

The Frame module is a core component in Rudis responsible for handling the RESP (Redis Serialization Protocol), defining command frame data structures and providing complete serialization and deserialization functionality. This module supports multiple RESP data types including Simple String, Bulk String, Integer, Array, Error, and Null, accurately parsing command requests from clients and converting them into internally processable data structures. The Frame module also implements a sticky command handling mechanism that effectively processes multiple concatenated command frames that may occur during network transmission, ensuring correct command parsing and execution. Through efficient encoding and decoding implementations, this module ensures efficient and stable communication between Redis clients and servers.

### replication

The Replication module implements Rudis's master-slave replication functionality, allowing one or more slave nodes to connect to a master node to achieve data synchronization and high availability. This module supports the complete master-slave replication process, including connection establishment, handshake, full synchronization, and incremental synchronization. Through the PSYNC command, it implements master-slave node connection and data synchronization, supporting disconnection reconnection and incremental data transmission, effectively reducing network bandwidth consumption and synchronization time. During full synchronization, the master node generates an RDB snapshot file and transfers it to slave nodes, while during incremental synchronization, it propagates write commands in real-time. This module also maintains replication connection state management, ensuring stability of master-slave relationships and data consistency, providing a solid foundation for building highly available Rudis clusters.

### server

The Server module is Rudis's core entry point, responsible for server startup, configuration parsing, and client request handling. It integrates functional modules including network communication, database management, persistence, and replication, forming a complete Rudis server implementation.

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

> For detailed information about transaction functionality, please refer to [Transaction Functionality Documentation](README-TRANSACTIONS.md)

## Build from Source

If you wish to obtain the release package by building from source code.

Please use common cargo commands.

```
// Standard startup
cargo run

// Startup with parameters
cargo run -- --port 8848
cargo run -- --save 20/1 60/2

// Specify configuration
cargo run -- --config rudis.conf

// Build program
cargo build

cargo build --release --target=x86_64-unknown-linux-musl

cargo build --release

// Code linting
cargo clippy
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=sleeprite/rudis&type=Date)](https://www.star-history.com/#sleeprite/rudis&Date)

## Open Source Collaboration

The Rudis project follows the [GNU GENERAL PUBLIC LICENSE](https://github.com/lunar-landing/rudis/blob/master/LICENSE) open source license. Thanks to these outstanding [Contributors](https://github.com/lunar-landing/rudis/graphs/contributors).

<a href="https://github.com/lunar-landing/rudis/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=sleeprite/rudis" />
</a>
