---
title: HyperLogLog Commands
titleTemplate: Commands
description: Overview of Rudis HyperLogLog commands including PFADD, PFCOUNT, and PFMERGE.
---

# HyperLogLog Commands

HyperLogLog is a **probabilistic** data structure used to estimate the **cardinality** (number of unique elements) of a set, using a small and fixed amount of memory.

- **Approximate result**: not exact, with a small error rate (typically ~0.81% in Redis-style HLL).
- **Use cases**: UV counting, unique search queries, deduplication statistics, etc.

## Command List

<div class="command-cards">
  <a href="./hyperloglog/pfadd" class="command-card">
    <div class="card-title">PFADD</div>
    <div class="card-description">Adds elements to a HyperLogLog</div>
  </a>
  <a href="./hyperloglog/pfcount" class="command-card">
    <div class="card-title">PFCOUNT</div>
    <div class="card-description">Returns the estimated cardinality</div>
  </a>
  <a href="./hyperloglog/pfmerge" class="command-card">
    <div class="card-title">PFMERGE</div>
    <div class="card-description">Merges multiple HyperLogLogs into one</div>
  </a>
</div>

## Notes

- The returned cardinality is an **estimate**, not an exact count.
- In Redis, HyperLogLog is represented as a string internally. Rudis also reports `TYPE` as `string` for compatibility.

