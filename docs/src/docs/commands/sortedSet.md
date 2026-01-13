---
title: Sorted Set Commands
titleTemplate: Commands
description: Overview of Rudis sorted set commands including ZADD, ZCARD, ZCOUNT, ZRANK, ZREM, and ZSCORE commands.
---

# Sorted Set Commands

Sorted set commands are similar to sets, but each member is associated with a score for sorting. Members are unique, but scores can be duplicated. Sorted sets are sorted from smallest to largest score.

## Command List

<div class="command-cards">
  <a href="./sortedSet/zadd" class="command-card">
    <div class="card-title">ZADD</div>
    <div class="card-description">Adds one or more members to a sorted set, or updates the score of an existing member</div>
  </a>
  <a href="./sortedSet/zcard" class="command-card">
    <div class="card-title">ZCARD</div>
    <div class="card-description">Returns the number of elements in a sorted set</div>
  </a>
  <a href="./sortedSet/zcount" class="command-card">
    <div class="card-title">ZCOUNT</div>
    <div class="card-description">Counts the number of members in a sorted set within a specified score range</div>
  </a>
  <a href="./sortedSet/zrank" class="command-card">
    <div class="card-title">ZRANK</div>
    <div class="card-description">Returns the rank of a specified member in a sorted set (starting from 0)</div>
  </a>
  <a href="./sortedSet/zrem" class="command-card">
    <div class="card-title">ZREM</div>
    <div class="card-description">Removes one or more members from a sorted set</div>
  </a>
  <a href="./sortedSet/zscore" class="command-card">
    <div class="card-title">ZSCORE</div>
    <div class="card-description">Returns the score of a specified member in a sorted set</div>
  </a>
  <a href="./sortedSet/zincrby" class="command-card">
    <div class="card-title">ZINCRBY</div>
    <div class="card-description">Increments the score of member in the sorted set by increment</div>
  </a>
</div>

## Use Cases

Sorted set commands are ideal for leaderboards, timelines, weighted queues, and other scenarios. Sorting functionality can be easily implemented through scores, while specific member rankings and scores can be quickly queried.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.