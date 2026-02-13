# AUTH

Authenticates the connection with the server password. If password protection is enabled, the client must authenticate using the AUTH command before executing other commands.

## Syntax

```
AUTH password
```

## Return

Simple string reply:
- OK if the password is correct
- ERR invalid password if the password is wrong

## Examples

```
redis> AUTH secret
OK
redis> AUTH wrongpassword
ERR invalid password
```
