use super::schemas::CommonSchemas;
use crate::types::Tool;
use serde_json::json;

pub fn get_execution_tools() -> Vec<Tool> {
    vec![
        get_cargo_run_tool(),
        get_cargo_test_tool(),
        get_cargo_bench_tool(),
    ]
}

fn get_cargo_run_tool() -> Tool {
    Tool {
        name: "run".to_string(),
        description: "Run a binary or example of the local package".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
                "bin": {
                    "type": "string",
                    "description": "Name of the bin target to run"
                },
                "example": {
                    "type": "string",
                    "description": "Name of the example target to run"
                },
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
                "release": CommonSchemas::release(),
                "target": CommonSchemas::target()
            }
        }),
    }
}

fn get_cargo_test_tool() -> Tool {
    Tool {
        name: "test".to_string(),
        description: "Run unit and integration tests".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
                "lib": {
                    "type": "boolean",
                    "description": "Test only this package's library unit tests"
                },
                "bin": {
                    "type": "string",
                    "description": "Test only the specified binary"
                },
                "bins": {
                    "type": "boolean",
                    "description": "Test all binaries"
                },
                "example": {
                    "type": "string",
                    "description": "Test only the specified example"
                },
                "examples": {
                    "type": "boolean",
                    "description": "Test all examples"
                },
                "test": {
                    "type": "string",
                    "description": "Test only the specified test target"
                },
                "tests": {
                    "type": "boolean",
                    "description": "Test all tests"
                },
                "bench": {
                    "type": "string",
                    "description": "Test only the specified bench target"
                },
                "benches": {
                    "type": "boolean",
                    "description": "Test all benches"
                },
                "all_targets": {
                    "type": "boolean",
                    "description": "Test all targets"
                },
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
                "release": CommonSchemas::release(),
                "target": CommonSchemas::target(),
                "nocapture": {
                    "type": "boolean",
                    "description": "Don't capture stdout/stderr of each task, allow printing directly"
                },
                "ignored": {
                    "type": "boolean",
                    "description": "Run ignored tests"
                },
                "include_ignored": {
                    "type": "boolean",
                    "description": "Run both ignored and not ignored tests"
                },
                "exact": {
                    "type": "boolean",
                    "description": "Exactly match filters rather than by substring"
                },
                "jobs": CommonSchemas::jobs(),
                "test_threads": {
                    "type": "integer",
                    "description": "Number of threads used for running tests in parallel"
                }
            }
        }),
    }
}

fn get_cargo_bench_tool() -> Tool {
    Tool {
        name: "bench".to_string(),
        description: "Run benchmarks".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
                "lib": {
                    "type": "boolean",
                    "description": "Benchmark only this package's library"
                },
                "bin": {
                    "type": "string",
                    "description": "Benchmark only the specified binary"
                },
                "bins": {
                    "type": "boolean",
                    "description": "Benchmark all binaries"
                },
                "example": {
                    "type": "string",
                    "description": "Benchmark only the specified example"
                },
                "examples": {
                    "type": "boolean",
                    "description": "Benchmark all examples"
                },
                "test": {
                    "type": "string",
                    "description": "Benchmark only the specified test target"
                },
                "tests": {
                    "type": "boolean",
                    "description": "Benchmark all tests"
                },
                "bench": {
                    "type": "string",
                    "description": "Benchmark only the specified bench target"
                },
                "benches": {
                    "type": "boolean",
                    "description": "Benchmark all benches"
                },
                "all_targets": {
                    "type": "boolean",
                    "description": "Benchmark all targets"
                },
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
                "target": CommonSchemas::target(),
                "jobs": CommonSchemas::jobs()
            }
        }),
    }
}
