---
title: 首页
layout: home

hero:
  name: Rudis
  text: A high-performance in memory database
  tagline: Remote Dictionary Service, In memory database
  image:
    light: /imgs/logo.png
    dark: /imgs/logo.png
    alt: General
  actions:
    - theme: brand
      text: Why Rudis
      link: /docs/guides/introduce
    - theme: alt
      text: Get Started
      link: /docs/guides/install
    - theme: alt
      text: View on GitHub
      link: https://github.com/sleeprite/rudis


features:
  - title: 跨平台支持
    details: 兼容 Windows、Linux、macOS 系统，可在多种操作系统上部署和运行。
    icon: <svg height="24" width="24" viewBox="0 -960 960 960" fill="var(--vp-c-green-2)" xmlns="http://www.w3.org/2000/svg"><path d="M160-160v-80h110l-16-14q-52-46-73-105t-21-119q0-111 66.5-197.5T400-790v84q-72 26-116 88.5T240-478q0 45 17 87.5t53 78.5l10 10v-98h80v240H160Zm400-10v-84q72-26 116-88.5T720-482q0-45-17-87.5T650-648l-10-10v98h-80v-240h240v80H690l16 14q49 49 71.5 106.5T800-482q0 111-66.5 197.5T560-170Z"/></svg>
    link: /zh/docs/guides/introduce
    linkText: 了解更多
  - title: 多种数据结构
    details: 兼容字符串、集合、哈希、列表、有序集合等多种数据结构，满足不同业务场景需求。
    icon: <svg height="24" width="24" viewBox="0 -960 960 960" fill="var(--vp-c-yellow-2)" xmlns="http://www.w3.org/2000/svg"><path d="M352-120H200q-33 0-56.5-23.5T120-200v-152q48 0 84-30.5t36-77.5q0-47-36-77.5T120-568v-152q0-33 23.5-56.5T200-800h160q0-42 29-71t71-29q42 0 71 29t29 71h160q33 0 56.5 23.5T800-720v160q42 0 71 29t29 71q0 42-29 71t-71 29v160q0 33-23.5 56.5T720-120H568q0-50-31.5-85T460-240q-45 0-76.5 35T352-120Zm-152-80h85q24-66 77-93t98-27q45 0 98 27t77 93h85v-240h80q8 0 14-6t6-14q0-8-6-14t-14-6h-80v-240H480v-80q0-8-6-14t-14-6q-8 0-14 6t-6 14v80H200v88q54 20 87 67t33 105q0 57-33 104t-87 68v88Zm310-310Z"/></svg>
    link: /zh/docs/guides/introduce
    linkText: 了解更多
  - title: 数据持久化
    details: 提供 RDB 与 AOF 机制以支持数据备份和恢复，确保数据安全可靠。
    icon: <svg height="24" width="24" viewBox="0 -960 960 960" fill="var(--vp-c-indigo-2)" xmlns="http://www.w3.org/2000/svg"><path d="M440-120v-240h80v80h320v80H520v80h-80Zm-320-80v-80h240v80H120Zm160-160v-80H120v-80h160v-80h80v240h-80Zm160-80v-80h400v80H440Zm160-160v-240h80v80h160v80H680v80h-80Zm-480-80v-80h400v80H120Z"/></svg>
    link: /zh/docs/advance/persistence
    linkText: 了解更多
  - title: 高性能并发
    details: 拥有卓越的处理速度和即时响应能力，支持多个线程中并发创建和删除键值。
    icon: <svg height="24" width="24" viewBox="0 -960 960 960" fill="var(--vp-c-purple-2)" xmlns="http://www.w3.org/2000/svg"><path d="M480-80q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-400Zm-40 200q17 0 28.5-11.5T480-320v-80q0-17 11.5-28.5T520-440h80q17 0 28.5 11.5T640-400v80q0 17-11.5 28.5T600-280h-80Zm40-280q17 0 28.5-11.5T520-600v-80q0-17-11.5-28.5T480-720h-80q-17 0-28.5 11.5T360-680v80q0 17 11.5 28.5T400-560h80Z"/></svg>
    link: /zh/docs/guides/introduce
    linkText: 了解更多
  - title: Docker 部署
    details: 提供 Docker 部署方式，简化部署流程，便于容器化运维管理。
    icon: <svg height="24" width="24" viewBox="0 -960 960 960" fill="var(--vp-c-blue-2)" xmlns="http://www.w3.org/2000/svg"><path d="M200-120v-60h80v-20h-80v-60h80v-20h-80v-60h80v-20h-80v-60h80v-20h-80v-60h80v-20h-80v-60h80v-20h-80v-60h80v-20h-80v-60h240v60h-160v20h160v60h-160v20h160v60h-160v20h160v60h-160v20h160v60h-160v20h160v60h-160v20h160v60h-160v20h160v60H200Zm440-60h160v-60h-160v60Zm0-80h160v-60h-160v60Zm0-80h160v-60h-160v60Zm0-80h160v-60h-160v60Zm0-80h160v-60h-160v60Zm0-80h160v-60h-160v60Zm0-80h160v-60h-160v60Z"/></svg>
    link: /zh/docs/guides/install
    linkText: 了解更多
  - title: 协议兼容
    details: 兼容 RESP 协议规范，可与现有的 Redis 客户端无缝对接。
    icon: <svg height="24" width="24" viewBox="0 -960 960 960" fill="var(--vp-c-teal-2)" xmlns="http://www.w3.org/2000/svg"><path d="M200-200v-80h560v80H200Zm0-160v-80h560v80H200Zm0-160v-80h560v80H200Zm0-160v-80h560v80H200Z"/></svg>
    link: /zh/docs/guides/protocolSpec
    linkText: 了解更多
---