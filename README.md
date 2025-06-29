# Cargo MCP Server

A Model Context Protocol (MCP) server that provides tools for managing Rust projects using the `cargo` command-line tool.

## Architecture

The server is organized into several modules for maintainability:

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library exports
├── server.rs            # MCP server implementation
├── tools/
│   ├── mod.rs           # Tool module exports
│   ├── definitions.rs   # Main tool registry
│   ├── schemas.rs       # Common schema utilities
│   ├── build_tools.rs   # Build-related tools (check, build, clippy, fmt)
│   ├── execution_tools.rs # Execution tools (run, test, bench)
│   ├── dependency_tools.rs # Dependency management (add, remove, update, tree)
│   ├── project_tools.rs # Project management (new, init, clean, doc)
│   ├── registry_tools.rs # Registry operations (search, info, install, uninstall)
│   ├── utility_tools.rs # Utility tools (metadata, version)
│   └── executor.rs      # Command execution logic
├── types.rs             # Data structures and types
└── error.rs             # Error handling
```

## Features

This MCP server provides the following cargo tools:

- **check** - Analyze code without producing executables
- **build** - Compile the current package
- **run** - Run a binary or example of the local package
- **fmt** - Format Rust code using rustfmt
- **clippy** - Run Clippy lints on the current package
- **add** - Add dependencies to a Cargo.toml manifest file

## Installation

1. Clone this repository
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

The server communicates via JSON-RPC over stdin/stdout. It's designed to be used with MCP-compatible clients.

### Example Tool Calls

#### Check a project
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "check",
    "arguments": {
      "working_directory": "/path/to/project",
      "all_targets": true
    }
  }
}
```

#### Build in release mode
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "build",
    "arguments": {
      "working_directory": "/path/to/project",
      "release": true
    }
  }
}
```

#### Run a specific binary
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "run",
    "arguments": {
      "working_directory": "/path/to/project",
      "bin": "my-binary"
    }
  }
}
```

#### Format code
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "fmt",
    "arguments": {
      "working_directory": "/path/to/project"
    }
  }
}
```

#### Run Clippy with fixes
```json
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "tools/call",
  "params": {
    "name": "clippy",
    "arguments": {
      "working_directory": "/path/to/project",
      "fix": true,
      "allow_dirty": true
    }
  }
}
```

#### Add a dependency
```json
{
  "jsonrpc": "2.0",
  "id": 6,
  "method": "tools/call",
  "params": {
    "name": "add",
    "arguments": {
      "working_directory": "/path/to/project",
      "dependency": "serde",
      "features": ["derive"]
    }
  }
}
```

## Tool Parameters

### Common Parameters

Most tools support these common parameters:

- `working_directory` (string, optional) - Working directory to run cargo in
- `package` (string, optional) - Package to operate on
- `features` (array of strings, optional) - Features to activate
- `all_features` (boolean, optional) - Activate all available features
- `no_default_features` (boolean, optional) - Do not activate default features
- `release` (boolean, optional) - Use release profile
- `target` (string, optional) - Target triple

### Target Selection Parameters

For check, build, and clippy:

- `lib` (boolean, optional) - Only this package's library
- `bin` (string, optional) - Only the specified binary
- `bins` (boolean, optional) - All binaries
- `example` (string, optional) - Only the specified example
- `examples` (boolean, optional) - All examples
- `test` (string, optional) - Only the specified test target
- `tests` (boolean, optional) - All tests
- `bench` (string, optional) - Only the specified bench target
- `benches` (boolean, optional) - All benches
- `all_targets` (boolean, optional) - All targets

### Clippy-specific Parameters

- `fix` (boolean, optional) - Automatically apply lint suggestions
- `allow_dirty` (boolean, optional) - Fix code even if working directory has changes
- `allow_staged` (boolean, optional) - Fix code even if working directory has staged changes

### add Parameters

- `dependency` (string, required) - Dependency to add
- `dev` (boolean, optional) - Add as development dependency
- `build` (boolean, optional) - Add as build dependency
- `optional` (boolean, optional) - Mark the dependency as optional
- `rename` (string, optional) - Rename the dependency
- `path` (string, optional) - Filesystem path to local crate
- `git` (string, optional) - Git repository location
- `branch` (string, optional) - Git branch
- `tag` (string, optional) - Git tag
- `rev` (string, optional) - Git reference

## Protocol Support

This server implements MCP protocol version 2024-11-05 and supports:

- `initialize` - Server initialization
- `tools/list` - List available tools
- `tools/call` - Execute cargo commands

## Error Handling

The server provides detailed error messages when cargo commands fail, including both stdout and stderr output from the cargo process.
