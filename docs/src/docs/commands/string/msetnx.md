# MSETNX

Sets the given keys to their respective values only if all the keys do not exist.

## Syntax

```
MSETNX key value [key value ...]
```

## Return

Integer reply: 

- 1 if all keys were set successfully.
- 0 if no key was set (at least one key already exists).

## Examples

```
redis> MSETNX key1 "Hello" key2 "World"
(integer) 1
redis> MSETNX key2 "New" key3 "Value"
(integer) 0
redis> GET key1
"Hello"
redis> EXISTS key3
(integer) 0
```