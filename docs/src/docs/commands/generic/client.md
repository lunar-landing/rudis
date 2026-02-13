# CLIENT

The CLIENT command is used to get or set client connection information.

## Syntax

```
CLIENT subcommand [arguments]
```

## Subcommands

- SETINFO: Set client library information

## Return

Simple string reply: OK or error message

## Examples

```
redis> CLIENT SETINFO lib-name redis
OK
redis> CLIENT SETINFO lib-ver 1.0
OK
```
