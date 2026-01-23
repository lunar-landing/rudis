# SRANDMEMBER

返回集合中的一个或多个随机成员，但不移除这些成员。与 SPOP 不同，SRANDMEMBER 不会修改集合。

## Syntax

```
SRANDMEMBER key [count]
```

## Return

- 当没有指定 count 参数时，返回一个随机成员；如果集合为空，返回 (nil)。
- 当指定了 count 参数时：
  - 如果 count 为正数，返回 count 个不重复的随机成员（数组）。
  - 如果 count 为负数，返回 count 个可能重复的随机成员（数组）。
  - 如果集合为空，返回空数组。

## Examples

```
redis> SADD myset "one"
(integer) 1
redis> SADD myset "two"
(integer) 1
redis> SADD myset "three"
(integer) 1
redis> SADD myset "four"
(integer) 1
redis> SADD myset "five"
(integer) 1
redis> SRANDMEMBER myset
"three"
redis> SRANDMEMBER myset 3
1) "two"
2) "five"
3) "one"
redis> SRANDMEMBER myset -3
1) "three"
2) "three"
3) "one"
redis> SCARD myset
(integer) 5
```

