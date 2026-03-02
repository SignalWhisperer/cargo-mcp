# AGENTS.md - AI Assistant Guide for cargo-mcp

> **Purpose**: This document provides AI coding assistants with essential context about the cargo-mcp codebase, focusing on information not readily available in README.md or other standard documentation.

**Last Updated**: 2026-03-02 (Commit: 01f8748461f1b62be87ee41f81f97d69c503de88)

---

## Table of Contents

1. [Quick Reference](#quick-reference)
2. [Project Structure](#project-structure)
3. [Coding Patterns](#coding-patterns)
4. [Development Workflow](#development-workflow)
5. [Architecture Insights](#architecture-insights)
6. [Common Tasks](#common-tasks)
7. [Testing Strategy](#testing-strategy)
8. [Extension Guidelines](#extension-guidelines)
9. [Troubleshooting](#troubleshooting)

---

## Quick Reference

### Project Essentials
- **Language**: Rust (Edition 2024)
- **Type**: MCP Server (Model Context Protocol)
- **Protocol Version**: 2024-11-05
- **Communication**: JSON-RPC over stdin/stdout
- **Async Runtime**: tokio
- **Total LOC**: 1,553 across 9 source files

### Key Files by Complexity
1. `src/tools/executor.rs` (920 LOC) - Core execution engine
2. `src/types.rs` (211 LOC) - Data structures
3. `src/tools/workflow_tools.rs` (200 LOC) - Tool schemas
4. `src/server.rs` (150 LOC) - MCP server implementation
5. `src/error.rs` (44 LOC) - Error handling

### Build Commands
```bash
cargo check          # Fast compilation check
cargo build          # Debug build
cargo build --release # Release build
cargo clippy         # Linting
cargo fmt            # Formatting
```

---

## Project Structure

### Directory Layout
```
cargo-mcp/
├── src/
│   ├── main.rs              # Entry point (7 LOC)
│   ├── lib.rs               # Library exports (9 LOC)
│   ├── server.rs            # MCP server (150 LOC)
│   ├── types.rs             # Data structures (211 LOC)
│   ├── error.rs             # Error handling (44 LOC)
│   └── tools/
│       ├── mod.rs           # Module exports (6 LOC)
│       ├── definitions.rs   # Tool registry (6 LOC)
│       ├── workflow_tools.rs # Tool schemas (200 LOC)
│       └── executor.rs      # Execution engine (920 LOC)
├── Cargo.toml               # Project manifest
├── README.md                # User documentation
└── mcp-config-example.json  # Example MCP client config
```

### Module Organization

**Core Modules** (exported via `lib.rs`):
- `error` - Error types and constructors
- `server` - MCP server implementation
- `tools` - Tool system (registry, schemas, executor)
- `types` - Protocol and parameter types

**Tool Submodules**:
- `definitions` - Central tool registry
- `workflow_tools` - JSON schema definitions for all tools
- `executor` - Command execution logic

---

## Coding Patterns

### 1. Error Handling Pattern

**Internal Errors** use `anyhow::Result`:
```rust
use anyhow::{Context, Result};

fn execute_cargo_command(cmd: &mut Command) -> Result<Value> {
    let output = cmd.output().context("Failed to execute cargo")?;
    // ... handle output
}
```

**Protocol Errors** use `McpError`:
```rust
use crate::error::McpError;

// Predefined constructors
McpError::parse_error("Invalid JSON".to_string())
McpError::invalid_params("Missing required parameter".to_string())
McpError::method_not_found("unknown_method".to_string())
McpError::internal_error("Execution failed".to_string())
```

**Conversion Pattern**:
```rust
// In executor: anyhow::Result
fn handle_tool(params: &CargoToolParams) -> Result<Value> { ... }

// In server: Convert to McpError
match handle_tool_call(name, params) {
    Ok(result) => McpResponse { result: Some(result), error: None, ... },
    Err(e) => McpResponse { 
        error: Some(McpError::internal_error(e.to_string())), 
        result: None, 
        ... 
    }
}
```

### 2. Command Building Pattern

All cargo commands follow this pattern:
```rust
fn handle_example(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("subcommand");
    
    // Set working directory
    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }
    
    // Add conditional flags
    if let Some(true) = params.release {
        cmd.arg("--release");
    }
    
    // Add options with values
    if let Some(package) = &params.package {
        cmd.arg("--package").arg(package);
    }
    
    // Add array parameters
    if let Some(features) = &params.features {
        for feature in features {
            cmd.arg("--features").arg(feature);
        }
    }
    
    // Execute
    execute_cargo_command(&mut cmd)
}
```

### 3. Parameter Deserialization Pattern

```rust
pub fn handle_tool_call(tool_name: &str, params: Value) -> Result<Value> {
    // Deserialize to CargoToolParams
    let params: CargoToolParams = serde_json::from_value(params)
        .context("Failed to parse tool parameters")?;
    
    // Route to handler
    match tool_name {
        "build" => handle_build(&params),
        "test" => handle_test(&params),
        // ... more tools
        _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name))
    }
}
```

### 4. Response Formatting Pattern

```rust
fn execute_cargo_command(cmd: &mut Command) -> Result<Value> {
    let start = Instant::now();
    let output = cmd.output()?;
    let duration = start.elapsed();
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    let result = if output.status.success() {
        format!("{}\n\nFinished in {:.1}s", stdout.trim(), duration.as_secs_f64())
    } else {
        format!("Error:\n{}\n\n{}", stderr.trim(), stdout.trim())
    };
    
    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}
```

### 5. Async Server Pattern

```rust
pub async fn run() -> Result<()> {
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = AsyncBufReader::new(stdin).lines();
    
    while let Some(line) = reader.next_line().await? {
        let request: McpRequest = serde_json::from_str(&line)?;
        let response = Self::handle_request(request);
        let response_json = serde_json::to_string(&response)?;
        stdout.write_all(response_json.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }
    
    Ok(())
}
```

---

## Development Workflow

### Adding a New Tool

**Step 1**: Define tool schema in `src/tools/workflow_tools.rs`
```rust
Tool {
    name: "my_tool".to_string(),
    description: "Description of what the tool does".to_string(),
    input_schema: json!({
        "type": "object",
        "properties": {
            "working_directory": {"type": "string"},
            "my_param": {"type": "boolean"}
        }
    })
}
```

**Step 2**: Add handler in `src/tools/executor.rs`
```rust
fn handle_my_tool(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("my-subcommand");
    
    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }
    
    // Add tool-specific logic
    
    execute_cargo_command(&mut cmd)
}
```

**Step 3**: Register in `handle_tool_call()` match statement
```rust
pub fn handle_tool_call(tool_name: &str, params: Value) -> Result<Value> {
    let params: CargoToolParams = serde_json::from_value(params)?;
    
    match tool_name {
        // ... existing tools
        "my_tool" => handle_my_tool(&params),
        _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name))
    }
}
```

**Step 4**: Add parameters to `CargoToolParams` if needed
```rust
pub struct CargoToolParams {
    // ... existing fields
    #[serde(default)]
    pub my_param: Option<bool>,
}
```

### Code Style Guidelines

**Naming Conventions**:
- Functions: `snake_case` (e.g., `handle_tool_call`)
- Structs/Enums: `PascalCase` (e.g., `CargoToolParams`)
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

**Serde Attributes**:
- Use `#[serde(default)]` for all optional fields in `CargoToolParams`
- Use `#[serde(rename = "camelCase")]` for JSON field name mapping
- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional response fields

**Error Messages**:
- Use `.context()` to add helpful error context
- Include relevant details (file paths, parameter values)
- Keep messages concise but informative

**Comments**:
- Document public APIs with `///` doc comments
- Use `//` for implementation notes
- Explain "why" not "what" in comments

---

## Architecture Insights

### Request Flow
```
Client → stdin → Server::run() → handle_request() → handle_tool_call() 
→ handler function → Command::new("cargo") → subprocess → output 
→ format result → McpResponse → stdout → Client
```

### Key Design Decisions

**1. Stateless Design**
- No persistent state between requests
- Each request is independent
- Simplifies error handling and recovery

**2. Synchronous Cargo Execution**
- Cargo commands block until completion
- Async only for stdin/stdout I/O
- Simplifies implementation, acceptable for use case

**3. Single Parameter Struct**
- `CargoToolParams` handles all tool parameters
- All fields optional with `#[serde(default)]`
- Simplifies deserialization and extension

**4. Comprehensive Error Context**
- Capture both stdout and stderr
- Include exit codes in error messages
- Preserve cargo's original error formatting

**5. No Command Injection**
- Parameters passed as separate args, not shell strings
- No shell interpolation
- Safe by design

### Component Responsibilities

**server.rs**:
- JSON-RPC protocol handling
- Request routing
- Response serialization
- Async I/O management

**executor.rs**:
- Parameter deserialization
- Command construction
- Subprocess execution
- Output formatting

**types.rs**:
- Protocol message types
- Parameter definitions
- Serialization rules

**error.rs**:
- Error type definition
- Standard error constructors
- JSON-RPC error codes

---

## Common Tasks

### Debugging Tool Execution

**Add debug output** (temporary):
```rust
eprintln!("Executing: {:?}", cmd);  // Prints to stderr, not captured by MCP
```

**Check command construction**:
```rust
let mut cmd = Command::new("cargo");
// ... build command
eprintln!("Command: {:?}", cmd);
```

**Inspect parameters**:
```rust
let params: CargoToolParams = serde_json::from_value(params)?;
eprintln!("Params: {:?}", params);
```

### Testing Manually

**Run server**:
```bash
cargo run
```

**Send test request** (in another terminal):
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cargo run
```

**Test specific tool**:
```bash
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"version","arguments":{}}}' | cargo run
```

### Handling New Cargo Flags

**1. Add to CargoToolParams**:
```rust
#[serde(default)]
pub my_new_flag: Option<bool>,
```

**2. Use in handler**:
```rust
if let Some(true) = params.my_new_flag {
    cmd.arg("--my-new-flag");
}
```

**3. Update tool schema**:
```rust
"properties": {
    "my_new_flag": {
        "type": "boolean",
        "description": "Description of the flag"
    }
}
```

---

## Testing Strategy

### Current State
⚠️ **No test files currently exist in the codebase**

### Recommended Testing Approach

**Unit Tests** (for individual handlers):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_building() {
        let params = CargoToolParams {
            working_directory: Some("/tmp".to_string()),
            release: Some(true),
            ..Default::default()
        };
        
        // Test command construction logic
    }
}
```

**Integration Tests** (for full request/response cycle):
```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_version_tool() {
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "version",
            "arguments": {}
        }
    });
    
    // Test full cycle
}
```

**Mocking Cargo** (for deterministic tests):
```rust
// Use test doubles or mock subprocess execution
// Consider using `assert_cmd` crate for CLI testing
```

### Test Execution
```bash
cargo test              # Run all tests
cargo test --lib        # Run library tests only
cargo test --test integration_test  # Run specific integration test
```

---

## Extension Guidelines

### Adding Support for New Cargo Commands

**Example**: Adding `cargo audit` support

**1. Research the command**:
```bash
cargo audit --help
```

**2. Identify parameters**:
- `--deny`: Deny warnings
- `--file`: Path to Cargo.lock
- `--json`: Output as JSON

**3. Add to CargoToolParams**:
```rust
#[serde(default)]
pub deny_audit: Option<bool>,
#[serde(default)]
pub audit_file: Option<String>,
#[serde(default)]
pub json_output: Option<bool>,
```

**4. Create handler**:
```rust
fn handle_audit(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("audit");
    
    if let Some(true) = params.deny_audit {
        cmd.arg("--deny");
    }
    
    if let Some(file) = &params.audit_file {
        cmd.arg("--file").arg(file);
    }
    
    if let Some(true) = params.json_output {
        cmd.arg("--json");
    }
    
    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }
    
    execute_cargo_command(&mut cmd)
}
```

**5. Add tool definition**:
```rust
Tool {
    name: "audit".to_string(),
    description: "Audit Cargo.lock for security vulnerabilities".to_string(),
    input_schema: json!({
        "type": "object",
        "properties": {
            "working_directory": {"type": "string"},
            "deny_audit": {"type": "boolean"},
            "audit_file": {"type": "string"},
            "json_output": {"type": "boolean"}
        }
    })
}
```

**6. Register in executor**:
```rust
"audit" => handle_audit(&params),
```

### Adding Custom Error Types

If you need more specific error handling:

**1. Extend McpError**:
```rust
impl McpError {
    pub fn cargo_not_found() -> Self {
        Self {
            code: -32603,
            message: "cargo command not found".to_string(),
            data: Some(json!({"hint": "Install Rust toolchain"}))
        }
    }
}
```

**2. Use in executor**:
```rust
let output = cmd.output()
    .map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            return anyhow::anyhow!("cargo not found");
        }
        e.into()
    })?;
```

---

## Troubleshooting

### Common Issues

**Issue**: Tool execution fails with "cargo not found"
**Solution**: Ensure cargo is in PATH. Check with `which cargo` or `cargo --version`

**Issue**: Working directory doesn't exist
**Solution**: Validate `working_directory` parameter before execution:
```rust
if let Some(dir) = &params.working_directory {
    if !std::path::Path::new(dir).exists() {
        return Err(anyhow::anyhow!("Working directory does not exist: {}", dir));
    }
}
```

**Issue**: JSON parsing errors
**Solution**: Validate JSON schema matches `CargoToolParams` structure. Use `serde_json::from_value` error messages for debugging.

**Issue**: Cargo command hangs
**Solution**: Some cargo commands may wait for input. Ensure stdin is not inherited:
```rust
cmd.stdin(Stdio::null());
```

**Issue**: Output encoding issues
**Solution**: Use `String::from_utf8_lossy` to handle non-UTF8 output gracefully (already implemented).

### Debugging Checklist

1. ✅ Verify cargo is installed and accessible
2. ✅ Check working directory exists and is accessible
3. ✅ Validate JSON request format
4. ✅ Confirm tool name matches registered tools
5. ✅ Check parameter types match schema
6. ✅ Review cargo command construction (add debug output)
7. ✅ Examine stdout and stderr from cargo
8. ✅ Check exit code from cargo subprocess

### Performance Considerations

**Slow compilation**:
- Use `cargo check` instead of `cargo build` for faster feedback
- Consider `--release` only when necessary
- Use target selection to limit scope

**Large output**:
- Cargo output is captured in memory
- Very large builds may consume significant memory
- Consider streaming output for long-running commands (future enhancement)

**Concurrent requests**:
- Current design processes requests sequentially
- Multiple clients can connect, but requests are serialized
- For parallel execution, consider spawning multiple server instances

---

## Additional Resources

### Internal Documentation
- `.agents/summary/index.md` - Documentation index for AI assistants
- `.agents/summary/architecture.md` - Detailed architecture documentation
- `.agents/summary/components.md` - Component-level documentation
- `.agents/summary/interfaces.md` - Complete API reference
- `.agents/summary/workflows.md` - Common workflow patterns

### External References
- [MCP Protocol Specification](https://modelcontextprotocol.io/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Tokio Documentation](https://tokio.rs/)
- [Serde Documentation](https://serde.rs/)

### Quick Links
- **Repository**: (Add your repo URL)
- **Issues**: (Add your issues URL)
- **Discussions**: (Add your discussions URL)

---

## Notes for AI Assistants

### When Helping with This Codebase

**DO**:
- ✅ Follow existing patterns (command building, error handling)
- ✅ Add `#[serde(default)]` to new optional parameters
- ✅ Use `anyhow::Result` for internal errors
- ✅ Use `McpError` for protocol errors
- ✅ Validate working directory when provided
- ✅ Capture both stdout and stderr
- ✅ Include execution time in success messages
- ✅ Use `.context()` for error messages
- ✅ Run `fmt` tool separately after making code changes (not automatic)

**DON'T**:
- ❌ Use shell interpolation for commands (security risk)
- ❌ Make parameters required unless absolutely necessary
- ❌ Ignore stderr output
- ❌ Forget to handle `working_directory` parameter
- ❌ Add state to the server (keep it stateless)
- ❌ Block async functions with synchronous I/O
- ❌ Panic on errors (use `Result` types)
- ❌ Assume code is auto-formatted (formatting is manual via `fmt` tool)

### Code Generation Guidelines

When generating code for this project:
1. Match the existing code style and patterns
2. Use the same error handling approach
3. Follow the command building pattern
4. Include appropriate error context
5. Add doc comments for public APIs
6. Keep functions focused and single-purpose
7. Prefer explicit over implicit

### Understanding User Intent

**"Add support for X"** → Follow "Adding a New Tool" workflow
**"Fix error Y"** → Check "Troubleshooting" section first
**"How does Z work?"** → Refer to architecture documentation
**"Optimize performance"** → Review "Performance Considerations"

---

**End of AGENTS.md**

*This document is automatically generated and maintained. For detailed technical documentation, see `.agents/summary/` directory.*
