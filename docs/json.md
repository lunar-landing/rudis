# JSON Commands

Rudis supports Redis-compatible JSON commands for storing, retrieving, and manipulating JSON data.

## Commands

### JSON.SET
Set the JSON value at key

**Syntax:**
```
JSON.SET key [path] value [NX|XX]
```

**Parameters:**
- `key`: The key to set
- `path`: The JSON path (default is "$")
- `value`: The JSON value to set
- `NX`: Only set if key does not exist
- `XX`: Only set if key exists

**Returns:**
- `OK` if successful
- `nil` if condition not met (NX/XX)

**Example:**
```redis
JSON.SET user:1 $ '{"name":"John","age":30}'
JSON.SET user:2 $ '{"name":"Jane","age":25}' NX
```

### JSON.GET
Get the JSON value at key

**Syntax:**
```
JSON.GET key [path]
```

**Parameters:**
- `key`: The key to get
- `path`: The JSON path (default is "$")

**Returns:**
- JSON string if key exists
- `nil` if key does not exist

**Example:**
```redis
JSON.GET user:1
JSON.GET user:1 $
```

### JSON.DEL
Delete a key or path

**Syntax:**
```
JSON.DEL key [path]
```

**Parameters:**
- `key`: The key to delete
- `path`: The JSON path (optional, deletes entire key if not specified)

**Returns:**
- Number of paths deleted (1 if key was deleted, 0 if key did not exist)

**Example:**
```redis
JSON.DEL user:1
JSON.DEL user:2 $
```

## Data Storage

JSON data is stored as strings in the database. The JSON structure is preserved as a serialized string and can be retrieved and manipulated using the JSON commands.

## Limitations

Current implementation has the following limitations:
1. Path operations are not fully implemented (only root path "$" is supported)
2. Advanced JSONPath queries are not supported
3. Complex JSON manipulations are not implemented

These limitations will be addressed in future versions.