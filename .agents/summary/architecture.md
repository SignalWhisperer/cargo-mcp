# Architecture Documentation

## System Architecture

cargo-mcp is a Model Context Protocol (MCP) server that acts as a bridge between MCP clients and the Rust cargo toolchain. It follows a layered architecture with clear separation of concerns.

```mermaid
graph TB
    subgraph "Client Layer"
        client[MCP Client<br/>Claude, IDEs, etc.]
    end
    
    subgraph "Protocol Layer"
        stdio[stdin/stdout<br/>JSON-RPC]
    end
    
    subgraph "Server Layer"
        server[CargoMcpServer<br/>Request Handler]
        router[Method Router<br/>initialize, tools/list, tools/call]
    end
    
    subgraph "Tool Layer"
        registry[Tool Registry<br/>get_available_tools]
        executor[Tool Executor<br/>handle_tool_call]
    end
    
    subgraph "Execution Layer"
        cmdbuilder[Command Builder<br/>Build cargo commands]
        process[Process Manager<br/>Execute & capture output]
    end
    
    subgraph "External"
        cargo[cargo CLI<br/>Rust toolchain]
    end
    
    client <-->|JSON-RPC| stdio
    stdio <--> server
    server --> router
    router --> registry
    router --> executor
    executor --> cmdbuilder
    cmdbuilder --> process
    process -->|spawn| cargo
    
    style client fill:#e1f5ff
    style server fill:#fff4e1
    style executor fill:#e8f5e9
    style cargo fill:#ffebee
```

## Architectural Layers

### 1. Protocol Layer
**Responsibility**: JSON-RPC communication over stdin/stdout

- Implements MCP protocol version 2024-11-05
- Async I/O using tokio
- Bidirectional message passing
- Protocol-compliant error handling

### 2. Server Layer (`server.rs`)
**Responsibility**: Request routing and response formatting

**Key Components**:
- `CargoMcpServer`: Main server struct
- `run()`: Async server loop
- `handle_request()`: Request dispatcher

**Supported Methods**:
- `initialize`: Server capability negotiation
- `notifications/initialized`: Initialization acknowledgment
- `tools/list`: Return available tools
- `tools/call`: Execute a specific tool

### 3. Tool Layer (`tools/`)
**Responsibility**: Tool definition and execution orchestration

**Components**:
- `definitions.rs`: Tool registry
- `workflow_tools.rs`: Tool schema definitions (200 LOC)
- `executor.rs`: Execution logic (920 LOC)

### 4. Execution Layer (`executor.rs`)
**Responsibility**: Cargo command construction and execution

**Specialized Handlers**:
- `handle_pre_build()`: check, build operations
- `handle_build()`: Compilation tasks
- `handle_lint()`: clippy, fmt operations
- `handle_test()`: test, bench execution
- `handle_add_crate()`: Dependency addition
- `handle_remove_crate()`: Dependency removal
- `handle_crate_info()`: Registry queries
- `handle_search_crates()`: Package search
- `handle_clean()`: Artifact cleanup

## Design Patterns

### 1. Command Pattern
Each cargo operation is encapsulated as a tool with:
- Name and description
- JSON schema for parameters
- Execution handler

### 2. Builder Pattern
Cargo commands are constructed incrementally:
```rust
let mut cmd = Command::new("cargo");
cmd.arg("build");
if release { cmd.arg("--release"); }
if let Some(pkg) = package { cmd.arg("--package").arg(pkg); }
```

### 3. Error Handling Strategy
- `anyhow::Result` for internal error propagation
- `McpError` for protocol-level errors
- Structured error responses with codes and messages

### 4. Async I/O
- Tokio runtime for non-blocking operations
- Async stdin/stdout handling
- Synchronous cargo execution (subprocess)

## Data Flow

### Request Processing Flow
```mermaid
sequenceDiagram
    participant C as Client
    participant S as Server
    participant R as Router
    participant E as Executor
    participant P as Process
    participant Cargo
    
    C->>S: JSON-RPC Request
    S->>S: Parse JSON
    S->>R: Route by method
    
    alt tools/call
        R->>E: handle_tool_call(name, params)
        E->>E: Deserialize params
        E->>E: Select handler
        E->>P: Build command
        P->>Cargo: Execute
        Cargo-->>P: stdout/stderr
        P-->>E: Output + exit code
        E->>E: Format result
        E-->>R: JSON result
    else tools/list
        R->>E: get_available_tools()
        E-->>R: Tool array
    else initialize
        R->>R: Build capabilities
    end
    
    R-->>S: Response data
    S->>S: Serialize JSON
    S-->>C: JSON-RPC Response
```

### Error Flow
```mermaid
graph LR
    A[Error Occurs] --> B{Error Type}
    B -->|Parse Error| C[parse_error]
    B -->|Invalid Params| D[invalid_params]
    B -->|Unknown Method| E[method_not_found]
    B -->|Execution Error| F[internal_error]
    
    C --> G[McpError]
    D --> G
    E --> G
    F --> G
    
    G --> H[JSON-RPC Error Response]
    
    style A fill:#ffebee
    style G fill:#fff4e1
    style H fill:#e1f5ff
```

## Concurrency Model

- **Single-threaded async**: One tokio runtime
- **Sequential request processing**: Requests handled one at a time
- **Blocking subprocess calls**: Cargo execution blocks until completion
- **No shared state**: Stateless server design

## Extension Points

### Adding New Tools
1. Define tool schema in `workflow_tools.rs`
2. Add handler function in `executor.rs`
3. Register in `handle_tool_call()` match statement

### Custom Parameters
Extend `CargoToolParams` struct in `types.rs` with new fields

### Error Types
Add new error constructors in `error.rs` for specific error cases

## Performance Considerations

- **Startup time**: Minimal (no heavy initialization)
- **Memory footprint**: Low (stateless, no caching)
- **Execution time**: Dominated by cargo subprocess
- **Scalability**: Limited by sequential processing

## Security Considerations

- **Command injection**: Parameters are passed as separate args (not shell interpolation)
- **Working directory**: Validated and set per-command
- **No privilege escalation**: Runs with user permissions
- **Input validation**: JSON schema validation via serde
