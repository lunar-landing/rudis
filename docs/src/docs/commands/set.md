---
title: Set Commands
titleTemplate: Commands
description: Overview of Rudis set commands including SADD, SCARD, SINTER, SISMEMBER, SMEMBERS, SPOP, SRANDMEMBER, SREM, SUNION, SUNIONSTORE, SDIFFSTORE, SINTERSTORE, and SMOVE commands.
---

# Set Commands

Set commands allow you to store unique string elements. Sets are unordered and do not allow duplicate elements. Multiple set operations such as intersection and union are supported.

## Command List

<div class="command-cards">
  <a href="./set/sadd" class="command-card">
    <div class="card-title">SADD</div>
    <div class="card-description">Adds one or more members to a set</div>
  </a>
  <a href="./set/scard" class="command-card">
    <div class="card-title">SCARD</div>
    <div class="card-description">Returns the number of elements in a set</div>
  </a>
  <a href="./set/sinter" class="command-card">
    <div class="card-title">SINTER</div>
    <div class="card-description">Returns the intersection of all given sets</div>
  </a>
  <a href="./set/sismember" class="command-card">
    <div class="card-title">SISMEMBER</div>
    <div class="card-description">Determines whether a member is a member of a set</div>
  </a>
  <a href="./set/smembers" class="command-card">
    <div class="card-title">SMEMBERS</div>
    <div class="card-description">Returns all members in a set</div>
  </a>
  <a href="./set/spop" class="command-card">
    <div class="card-title">SPOP</div>
    <div class="card-description">Removes and returns a random element from a set</div>
  </a>
  <a href="./set/srandmember" class="command-card">
    <div class="card-title">SRANDMEMBER</div>
    <div class="card-description">Returns one or more random members from a set without removing them</div>
  </a>
  <a href="./set/srem" class="command-card">
    <div class="card-title">SREM</div>
    <div class="card-description">Removes one or more members from a set</div>
  </a>
  <a href="./set/sunion" class="command-card">
    <div class="card-title">SUNION</div>
    <div class="card-description">Returns the union of all given sets</div>
  </a>
  <a href="./set/sunionstore" class="command-card">
    <div class="card-title">SUNIONSTORE</div>
    <div class="card-description">Stores the union of all given sets in a specified set</div>
  </a>
  <a href="./set/sdiffstore" class="command-card">
    <div class="card-title">SDIFFSTORE</div>
    <div class="card-description">Computes the difference of all given sets and stores it in a specified set</div>
  </a>
  <a href="./set/sinterstore" class="command-card">
    <div class="card-title">SINTERSTORE</div>
    <div class="card-description">Computes the intersection of all given sets and stores it in a specified set</div>
  </a>
  <a href="./set/smove" class="command-card">
    <div class="card-title">SMOVE</div>
    <div class="card-description">Moves a member from one set to another</div>
  </a>
</div>

## Use Cases

Set commands are ideal for tag systems, friend relationships, deduplication statistics, and other scenarios. Due to the uniqueness characteristic of sets, deduplication functions can be easily implemented, while set operations can conveniently handle relational queries.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.