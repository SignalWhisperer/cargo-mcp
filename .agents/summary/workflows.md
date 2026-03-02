# Workflows Documentation

## Primary Workflows

### 1. Server Initialization Workflow

```mermaid
sequenceDiagram
    participant Client
    participant Server
    participant Tokio
    
    Client->>Server: Start process
    Server->>Tokio: Initialize runtime
    Tokio->>Server: Runtime ready
    Server->>Server: Setup stdin/stdout
    
    Client->>Server: initialize request
    Server->>Server: Build capabilities
    Server-->>Client: Server info + capabilities
    
    Client->>Server: notifications/initialized
    Server-->>Client: Acknowledgment
    
    Note over Client,Server: Server ready for tool calls
```

**Steps**:
1. Client spawns cargo-mcp process
2. Server initializes tokio async runtime
3. Server sets up stdin/stdout readers/writers
4. Client sends `initialize` request
5. Server responds with protocol version and capabilities
6. Client sends `notifications/initialized`
7. Server is ready to handle tool calls

---

### 2. Tool Discovery Workflow

```mermaid
sequenceDiagram
    participant Client
    participant Server
    participant Registry
    participant Schemas
    
    Client->>Server: tools/list request
    Server->>Registry: get_available_tools()
    Registry->>Schemas: get_workflow_tools()
    Schemas-->>Registry: Tool array
    Registry-->>Server: Tool definitions
    Server-->>Client: tools/list response
    
    Note over Client: Client now knows available tools
```

**Steps**:
1. Client requests tool list
2. Server calls tool registry
3. Registry retrieves tool schemas
4. Server returns array of tool definitions with schemas
5. Client caches tool information

---

### 3. Tool Execution Workflow

```mermaid
sequenceDiagram
    participant Client
    participant Server
    participant Executor
    participant Process
    participant Cargo
    
    Client->>Server: tools/call request
    Server->>Executor: handle_tool_call(name, params)
    Executor->>Executor: Deserialize params
    Executor->>Executor: Route to handler
    Executor->>Process: Build cargo command
    Process->>Cargo: Spawn subprocess
    
    Note over Cargo: Execute cargo operation
    
    Cargo-->>Process: stdout + stderr + exit code
    Process-->>Executor: Command output
    Executor->>Executor: Format result
    Executor-->>Server: JSON result
    Server-->>Client: tools/call response
```

**Steps**:
1. Client sends tool call with name and arguments
2. Server routes to executor
3. Executor deserializes parameters to `CargoToolParams`
4. Executor selects appropriate handler based on tool name
5. Handler builds cargo command with flags
6. Process spawns cargo subprocess
7. Cargo executes and returns output
8. Executor formats output as MCP response
9. Server sends response to client

---

## Development Workflows

### 4. Build and Check Workflow

```mermaid
graph TB
    Start[Start Development] --> Check[cargo check]
    Check -->|Errors| Fix[Fix Errors]
    Fix --> Check
    Check -->|Success| Clippy[cargo clippy]
    Clippy -->|Warnings| FixLint[Fix Lints]
    FixLint --> Clippy
    Clippy -->|Success| Fmt[cargo fmt]
    Fmt --> Build[cargo build]
    Build -->|Success| Test[cargo test]
    Test -->|Pass| Done[Ready to Commit]
    Test -->|Fail| FixTest[Fix Tests]
    FixTest --> Test
    
    style Start fill:#e1f5ff
    style Done fill:#e8f5e9
    style Fix fill:#ffebee
    style FixLint fill:#ffebee
    style FixTest fill:#ffebee
```

**MCP Tool Sequence**:
1. `check` - Fast compilation check
2. `clippy` - Lint analysis
3. `build` - Full compilation
4. `test` - Run test suite
5. `fmt` - Code formatting (run separately as needed)

---

### 5. Dependency Management Workflow

```mermaid
graph TB
    Need[Need New Dependency] --> Search[cargo search]
    Search --> Info[cargo info]
    Info --> Add[cargo add]
    Add --> Check[cargo check]
    Check -->|Errors| Remove[cargo remove]
    Check -->|Success| Tree[cargo tree]
    Tree --> Update[cargo update]
    Update --> Done[Dependency Added]
    
    style Need fill:#e1f5ff
    style Done fill:#e8f5e9
```

**MCP Tool Sequence**:
1. `search` - Find packages on crates.io
2. `info` - Get package details
3. `add` - Add dependency to Cargo.toml
4. `check` - Verify compilation
5. `tree` - Visualize dependency graph
6. `update` - Update lock file

---

### 6. Project Creation Workflow

```mermaid
graph LR
    A[Decide Project Type] --> B{Binary or Library?}
    B -->|Binary| C[cargo new --bin]
    B -->|Library| D[cargo new --lib]
    C --> E[Initialize Git]
    D --> E
    E --> F[cargo build]
    F --> G[cargo test]
    G --> H[Project Ready]
    
    style A fill:#e1f5ff
    style H fill:#e8f5e9
```

**MCP Tool Sequence**:
1. `new` - Create new project with template
2. `build` - Initial compilation
3. `test` - Verify default tests pass

---

### 7. Release Workflow

```mermaid
graph TB
    Start[Ready to Release] --> Test[cargo test --release]
    Test -->|Pass| Clippy[cargo clippy --release]
    Clippy -->|Clean| Doc[cargo doc]
    Doc --> Build[cargo build --release]
    Build --> Bench[cargo bench]
    Bench --> Package[Prepare Package]
    Package --> Done[Ready for Distribution]
    
    Test -->|Fail| Fix[Fix Issues]
    Clippy -->|Warnings| Fix
    Fix --> Start
    
    style Start fill:#e1f5ff
    style Done fill:#e8f5e9
    style Fix fill:#ffebee
```

**MCP Tool Sequence**:
1. `test` with `release: true` - Test optimized build
2. `clippy` with `release: true` - Lint release build
3. `doc` - Generate documentation
4. `build` with `release: true` - Create release binary
5. `bench` - Run benchmarks

---

### 8. Continuous Integration Workflow

```mermaid
graph TB
    Push[Git Push] --> Check[cargo check --all-targets]
    Check --> Fmt[cargo fmt --check]
    Fmt --> Clippy[cargo clippy -- -D warnings]
    Clippy --> Test[cargo test]
    Test --> Build[cargo build --release]
    Build --> Success[CI Pass]
    
    Check -->|Fail| Fail[CI Fail]
    Fmt -->|Fail| Fail
    Clippy -->|Fail| Fail
    Test -->|Fail| Fail
    Build -->|Fail| Fail
    
    style Push fill:#e1f5ff
    style Success fill:#e8f5e9
    style Fail fill:#ffebee
```

**MCP Tool Sequence**:
1. `check` with `all_targets: true` - Check all code
2. `fmt` - Verify formatting
3. `clippy` - Enforce lint rules
4. `test` - Run full test suite
5. `build` with `release: true` - Verify release build

---

## Error Handling Workflows

### 9. Error Recovery Workflow

```mermaid
graph TB
    Error[Tool Execution Error] --> Type{Error Type}
    
    Type -->|Compilation Error| ShowError[Display stderr]
    Type -->|Missing Tool| Install[Install Tool]
    Type -->|Invalid Params| FixParams[Correct Parameters]
    Type -->|Network Error| Retry[Retry Operation]
    
    ShowError --> UserFix[User Fixes Code]
    Install --> Rerun[Rerun Tool]
    FixParams --> Rerun
    Retry --> Rerun
    
    UserFix --> Rerun
    Rerun --> Success[Operation Success]
    
    style Error fill:#ffebee
    style Success fill:#e8f5e9
```

**Error Handling**:
1. Executor catches cargo errors
2. Formats error with stdout/stderr
3. Returns as `internal_error` with details
4. Client displays error to user
5. User corrects issue and retries

---

### 10. Workspace Workflow

```mermaid
graph TB
    Start[Workspace Project] --> Check[cargo check --workspace]
    Check --> Test[cargo test --workspace]
    Test --> Member{Specific Member?}
    
    Member -->|Yes| PkgCheck[cargo check -p member]
    Member -->|No| Build[cargo build --workspace]
    
    PkgCheck --> PkgTest[cargo test -p member]
    PkgTest --> Build
    
    Build --> Done[Workspace Verified]
    
    style Start fill:#e1f5ff
    style Done fill:#e8f5e9
```

**MCP Tool Sequence**:
1. `check` with `workspace: true` - Check all packages
2. `test` with `workspace: true` - Test all packages
3. `check` with `package: "member"` - Check specific package
4. `build` with `workspace: true` - Build all packages

---

## Tool Combination Patterns

### Quick Validation
```
check → clippy
```

### Full Build Cycle
```
check → build → test → doc
```

### With Formatting
```
fmt → check → build → test
```

### Dependency Update
```
update → tree → check → test
```

### New Feature Development
```
add → check → test → clippy
```

### Code Formatting
```
fmt (run separately as needed)
```

### Release Preparation
```
test --release → clippy --release → doc → build --release → bench
```

---

## Async Operation Flow

### Request Processing Loop

```mermaid
graph TB
    Start[Server Start] --> Listen[Listen on stdin]
    Listen --> Read[Read line]
    Read --> Parse[Parse JSON]
    Parse --> Handle[Handle request]
    Handle --> Execute[Execute tool]
    Execute --> Format[Format response]
    Format --> Write[Write to stdout]
    Write --> Listen
    
    Parse -->|Error| ErrorResp[Error response]
    Handle -->|Error| ErrorResp
    Execute -->|Error| ErrorResp
    ErrorResp --> Write
    
    style Start fill:#e1f5ff
    style Listen fill:#fff4e1
```

**Async Characteristics**:
- Non-blocking stdin/stdout
- Synchronous cargo execution (blocking)
- Sequential request processing
- No concurrent tool execution

---

## Best Practices

### Efficient Tool Usage
1. Use `check` before `build` for faster feedback
2. Run `clippy` with `--fix` to auto-correct issues
3. Use `tree` to understand dependency conflicts
4. Run `test` with specific filters for faster iteration

### Error Prevention
1. Always specify `working_directory` for clarity
2. Use `dry_run` for destructive operations
3. Check `metadata` before complex operations
4. Validate with `check` before `build`

### Performance Optimization
1. Use `release: false` during development
2. Limit `all_targets` to specific targets when possible
3. Use `jobs` parameter to control parallelism
4. Cache dependencies with `update` sparingly
