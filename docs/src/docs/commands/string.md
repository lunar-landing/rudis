---
title: String Commands
titleTemplate: Commands
description: Overview of Rudis string commands including APPEND, BITCOUNT, BITOP, DECR, DECRBY, GET, GETBIT, GETRANGE, GETSET, INCR, INCRBY, INCRBYFLOAT, MGET, MSET, MSETNX, SET, SETBIT, SETEX, PSETEX, SETNX, SETRANGE, and STRLEN commands.
---

# String Commands

String commands are the most basic data type commands, which can store strings, integers, or floating-point numbers. Strings can store up to 512 MB of data.

## Command List

<div class="command-cards">
  <a href="./string/append" class="command-card">
    <div class="card-title">APPEND</div>
    <div class="card-description">If the key already exists and is a string, appends the value to the end of the key</div>
  </a>
  <a href="./string/bitcount" class="command-card">
    <div class="card-title">BITCOUNT</div>
    <div class="card-description">Counts the number of set bits (population counting) in a string</div>
  </a>
  <a href="./string/bitop" class="command-card">
    <div class="card-title">BITOP</div>
    <div class="card-description">Performs bitwise operations between strings and stores the result in a destination key</div>
  </a>
  <a href="./string/decr" class="command-card">
    <div class="card-title">DECR</div>
    <div class="card-description">Decrements the numeric value stored in the key by one</div>
  </a>
  <a href="./string/decrby" class="command-card">
    <div class="card-title">DECRBY</div>
    <div class="card-description">Decrements the numeric value stored in the key by the specified value</div>
  </a>
  <a href="./string/get" class="command-card">
    <div class="card-title">GET</div>
    <div class="card-description">Gets the value of the specified key</div>
  </a>
  <a href="./string/getbit" class="command-card">
    <div class="card-title">GETBIT</div>
    <div class="card-description">Returns the bit value at offset in the string value stored at key</div>
  </a>
  <a href="./string/getrange" class="command-card">
    <div class="card-title">GETRANGE</div>
    <div class="card-description">Returns a substring of the string value in the key</div>
  </a>
  <a href="./string/getset" class="command-card">
    <div class="card-title">GETSET</div>
    <div class="card-description">Sets the value of the given key to a new value and returns the old value of the key</div>
  </a>
  <a href="./string/incr" class="command-card">
    <div class="card-title">INCR</div>
    <div class="card-description">Increments the numeric value stored in the key by one</div>
  </a>
  <a href="./string/incrby" class="command-card">
    <div class="card-title">INCRBY</div>
    <div class="card-description">Increments the numeric value stored in the key by the specified value</div>
  </a>
  <a href="./string/incrbyfloat" class="command-card">
    <div class="card-title">INCRBYFLOAT</div>
    <div class="card-description">Increments the numeric value stored in the key by the specified float value</div>
  </a>
  <a href="./string/mget" class="command-card">
    <div class="card-title">MGET</div>
    <div class="card-description">Gets the values of all given keys</div>
  </a>
  <a href="./string/mset" class="command-card">
    <div class="card-title">MSET</div>
    <div class="card-description">Sets one or more key-value pairs simultaneously</div>
  </a>
  <a href="./string/msetnx" class="command-card">
    <div class="card-title">MSETNX</div>
    <div class="card-description">Sets the given keys to their respective values only if all the keys do not exist</div>
  </a>
  <a href="./string/set" class="command-card">
    <div class="card-title">SET</div>
    <div class="card-description">Sets the value of the specified key</div>
  </a>
  <a href="./string/setbit" class="command-card">
    <div class="card-title">SETBIT</div>
    <div class="card-description">Sets or clears the bit at offset in the string value stored at key</div>
  </a>
  <a href="./string/setex" class="command-card">
    <div class="card-title">SETEX</div>
    <div class="card-description">Sets the value of a key with expiration time in seconds</div>
  </a>
  <a href="./string/psetex" class="command-card">
    <div class="card-title">PSETEX</div>
    <div class="card-description">Sets the value of a key with expiration time in milliseconds</div>
  </a>
  <a href="./string/setnx" class="command-card">
    <div class="card-title">SETNX</div>
    <div class="card-description">Sets the value of a key only if it does not exist</div>
  </a>
  <a href="./string/setrange" class="command-card">
    <div class="card-title">SETRANGE</div>
    <div class="card-description">Overwrites part of the string stored at key, starting at the specified offset, with the given value</div>
  </a>
  <a href="./string/strlen" class="command-card">
    <div class="card-title">STRLEN</div>
    <div class="card-description">Returns the length of the string value stored in the key</div>
  </a>
</div>

## Use Cases

String commands are the most commonly used data type commands, suitable for various scenarios such as caching, counters, and distributed locks. Counter functionality can be easily implemented through INCR and DECR series commands.

For detailed usage and parameters of each command, please refer to the individual command documentation linked above.