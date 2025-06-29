# Cargo MCP Server

A Model Context Protocol (MCP) server that provides tools for managing Rust projects using the `cargo` command-line tool.

## Disclaimer

This entire repository was built with generative AI assistance, guided by human oversight throughout the development process. The code, documentation, and architecture were collaboratively developed between human direction and AI implementation.

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

This MCP server provides comprehensive cargo tools organized by category:

### Build Tools
- **check** - Analyze code without producing executables
- **build** - Compile the current package
- **clippy** - Run Clippy lints on the current package
- **fmt** - Format Rust code using rustfmt

### Execution Tools
- **run** - Run a binary or example of the local package
- **test** - Run unit and integration tests
- **bench** - Run benchmarks

### Dependency Management
- **add** - Add dependencies to a Cargo.toml manifest file
- **remove** - Remove dependencies from a Cargo.toml manifest file
- **update** - Update dependencies as recorded in the local lock file
- **tree** - Display a tree visualization of a dependency graph

### Project Management
- **new** - Create a new cargo package at <path>
- **init** - Create a new cargo package in an existing directory
- **clean** - Remove artifacts that cargo has generated in the past
- **doc** - Build this package's and its dependencies' documentation

### Registry Operations
- **search** - Search packages in crates.io
- **info** - Display information about a package in the registry
- **install** - Install a Rust binary
- **uninstall** - Remove a Rust binary

### Utility Tools
- **metadata** - Output the resolved dependencies of a package in machine-readable format
- **version** - Show version information for cargo and rust

## Installation

1. Clone this repository
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Configure your MCP client to use the server. Example configuration:
   ```json
   {
     "mcpServers": {
       "cargo-mcp": {
         "command": "/path/to/cargo-mcp/target/release/cargo-mcp",
         "args": []
       }
     }
   }
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

#### Search for packages
```json
{
  "jsonrpc": "2.0",
  "id": 7,
  "method": "tools/call",
  "params": {
    "name": "search",
    "arguments": {
      "working_directory": "/path/to/project",
      "query": "tokio",
      "limit": 5
    }
  }
}
```

#### Run tests
```json
{
  "jsonrpc": "2.0",
  "id": 8,
  "method": "tools/call",
  "params": {
    "name": "test",
    "arguments": {
      "working_directory": "/path/to/project",
      "release": true
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

For check, build, clippy, test, and bench:

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

### Build-specific Parameters

For check and build:
- `profile` (string, optional) - Build artifacts with the specified profile
- `message_format` (string, optional) - Error format (human, short, json, etc.)
- `workspace` (boolean, optional) - Build all packages in the workspace
- `exclude` (array of strings, optional) - Exclude packages from the operation

### Clippy-specific Parameters

- `fix` (boolean, optional) - Automatically apply lint suggestions
- `allow_dirty` (boolean, optional) - Fix code even if working directory has changes
- `allow_staged` (boolean, optional) - Fix code even if working directory has staged changes

### Test-specific Parameters

- `exact` (boolean, optional) - Exactly match filters rather than by substring
- `ignored` (boolean, optional) - Run ignored tests
- `include_ignored` (boolean, optional) - Run both ignored and not ignored tests
- `jobs` (integer, optional) - Number of parallel jobs
- `nocapture` (boolean, optional) - Don't capture stdout/stderr
- `test_threads` (integer, optional) - Number of threads for running tests

### Dependency Management Parameters

#### add Parameters

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
- `default_features` (boolean, optional) - Re-enable default features
- `registry` (string, optional) - Registry to use

#### remove Parameters

- `dependency` (string, required) - Dependency to remove
- `dev` (boolean, optional) - Remove as development dependency
- `build` (boolean, optional) - Remove as build dependency

#### update Parameters

- `aggressive` (boolean, optional) - Force updating all dependencies
- `dry_run` (boolean, optional) - Don't actually write the lockfile
- `precise` (string, optional) - Update to exactly this version
- `workspace` (boolean, optional) - Update all packages in workspace

#### tree Parameters

- `duplicates` (boolean, optional) - Show only dependencies with multiple versions
- `edges` (string, optional) - Kinds of dependencies to display
- `format` (string, optional) - Format string for printing dependencies
- `invert` (array of strings, optional) - Invert dependency graph
- `no_dedupe` (boolean, optional) - Repeat shared dependencies
- `prefix` (string, optional) - How to display the tree
- `prune` (array of strings, optional) - Prune packages from display
- `depth` (integer, optional) - Maximum display depth
- `charset` (string, optional) - Character set (utf8, ascii)

### Project Management Parameters

#### new/init Parameters

- `path` (string, required for new) - Path for the new package
- `name` (string, optional) - Package name
- `bin_template` (boolean, optional) - Use binary template
- `lib_template` (boolean, optional) - Use library template
- `edition` (string, optional) - Edition to set
- `registry` (string, optional) - Registry to use

#### doc Parameters

- `open` (boolean, optional) - Open docs in browser
- `no_deps` (boolean, optional) - Don't build documentation for dependencies
- `document_private_items` (boolean, optional) - Document private items
- `jobs` (integer, optional) - Number of parallel jobs

### Registry Parameters

#### search Parameters

- `query` (string, required) - Search query
- `limit` (integer, optional) - Limit results (default: 10, max: 100)
- `registry` (string, optional) - Registry to use

#### info Parameters

- `query` (string, required) - Package name
- `registry` (string, optional) - Registry to use

#### install Parameters

- `query` (string, optional) - Package name to install
- `version` (string, optional) - Specify version
- `git_url` (string, optional) - Git URL to install from
- `branch_install` (string, optional) - Git branch
- `tag_install` (string, optional) - Git tag
- `rev_install` (string, optional) - Git commit
- `path_install` (string, optional) - Local path
- `bin_install` (string, optional) - Install only specified binary
- `bins_install` (boolean, optional) - Install all binaries
- `example_install` (string, optional) - Install only specified example
- `examples_install` (boolean, optional) - Install all examples
- `force` (boolean, optional) - Force overwrite
- `no_track` (boolean, optional) - Don't save tracking information
- `locked` (boolean, optional) - Assert Cargo.lock unchanged
- `root` (string, optional) - Installation directory
- `registry` (string, optional) - Registry to use
- `index` (string, optional) - Registry index
- `list` (boolean, optional) - List installed packages

#### uninstall Parameters

- `query` (string, required) - Package name to uninstall
- `bin_install` (string, optional) - Only uninstall specified binary
- `root` (string, optional) - Directory to uninstall from

### Utility Parameters

#### metadata Parameters

- `no_deps` (boolean, optional) - Don't fetch dependencies
- `format_version` (integer, optional) - Format version

## Protocol Support

This server implements MCP protocol version 2024-11-05 and supports:

- `initialize` - Server initialization
- `tools/list` - List available tools
- `tools/call` - Execute cargo commands

## Development

To contribute to this project:

1. Clone the repository
2. Make your changes
3. Test with `cargo check` and `cargo build`
4. Format code with `cargo fmt`
5. Run lints with `cargo clippy`

## Error Handling

The server provides detailed error messages when cargo commands fail, including both stdout and stderr output from the cargo process.
