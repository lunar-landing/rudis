---
title: Hash Commands
titleTemplate: Commands
description: Overview of Rudis hash commands including HDEL, HEXISTS, HGET, HGETALL, HKEYS, HLEN, HMGET, HMSET, HSET, HSETNX, HSCAN, HSTRLEN, and HVALS commands.
---

# Hash Commands

Hash commands allow you to store key-value pairs as hash tables (also known as maps or dictionaries). Each hash can store up to 2^32 - 1 field-value pairs.

## Command List

<div class="command-cards">
  <a href="./hash/hdel" class="command-card">
    <div class="card-title">HDEL</div>
    <div class="card-description">Deletes one or more specified fields from a hash table</div>
  </a>
  <a href="./hash/hexists" class="command-card">
    <div class="card-title">HEXISTS</div>
    <div class="card-description">Checks if a specified field exists in a hash table</div>
  </a>
  <a href="./hash/hget" class="command-card">
    <div class="card-title">HGET</div>
    <div class="card-description">Gets the value of a specified field stored in a hash table</div>
  </a>
  <a href="./hash/hgetall" class="command-card">
    <div class="card-title">HGETALL</div>
    <div class="card-description">Gets all fields and values in a hash table</div>
  </a>
  <a href="./hash/hscan" class="command-card">
    <div class="card-title">HSCAN</div>
    <div class="card-description">Incrementally iterates over the fields and values of a hash</div>
  </a>
  <a href="./hash/hkeys" class="command-card">
    <div class="card-title">HKEYS</div>
    <div class="card-description">Gets all field names in a hash table</div>
  </a>
  <a href="./hash/hlen" class="command-card">
    <div class="card-title">HLEN</div>
    <div class="card-description">Gets the number of fields in a hash table</div>
  </a>
  <a href="./hash/hmget" class="command-card">
    <div class="card-title">HMGET</div>
    <div class="card-description">Gets the values of all given fields</div>
  </a>
  <a href="./hash/hmset" class="command-card">
    <div class="card-title">HMSET</div>
    <div class="card-description">Sets multiple field-value pairs to a hash table simultaneously</div>
  </a>
  <a href="./hash/hset" class="command-card">
    <div class="card-title">HSET</div>
    <div class="card-description">Sets a field-value pair to a hash table</div>
  </a>
  <a href="./hash/hsetnx" class="command-card">
    <div class="card-title">HSETNX</div>
    <div class="card-description">Sets the value of a field in a hash table only if the field does not exist</div>
  </a>
  <a href="./hash/hstrlen" class="command-card">
    <div class="card-title">HSTRLEN</div>
    <div class="card-description">Returns the string length of the value of a specified field in a hash table</div>
  </a>
  <a href="./hash/hvals" class="command-card">
    <div class="card-title">HVALS</div>
    <div class="card-description">Returns all values in a hash table</div>
  </a>
  <a href="./hash/hincrby" class="command-card">
    <div class="card-title">HINCRBY</div>
    <div class="card-description">Increments the integer value of a hash field by the given number</div>
  </a>
  <a href="./hash/hincrbyfloat" class="command-card">
    <div class="card-title">HINCRBYFLOAT</div>
    <div class="card-description">Increments the float value of a hash field by the given amount</div>
  </a>
</div>

## Use Cases

Hash commands are ideal for representing objects, such as user profiles, product information, etc. You can store all properties of an object in a single hash, with each property as a field.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.