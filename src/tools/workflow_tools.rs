use crate::types::Tool;

pub fn get_workflow_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "pre_build".to_string(),
            description: "Run cargo check with tests enabled by default, with options to specify package and disable tests".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "package": {
                        "type": "string", 
                        "description": "Package to check (equivalent to -p flag)"
                    },
                    "no_tests": {
                        "type": "boolean",
                        "description": "Disable test checking (tests are checked by default)"
                    }
                }
            }),
        },
        Tool {
            name: "build".to_string(),
            description: "Build the project with minimal options - tests enabled by default, optional release mode".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "package": {
                        "type": "string", 
                        "description": "Package to build (equivalent to -p flag)"
                    },
                    "release": {
                        "type": "boolean",
                        "description": "Build in release mode with optimizations"
                    }
                }
            }),
        },
        Tool {
            name: "lint".to_string(),
            description: "Lint code with pedantic checks, options to soften to warnings and ignore missing docs".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "package": {
                        "type": "string", 
                        "description": "Package to lint (equivalent to -p flag)"
                    },
                    "no_tests": {
                        "type": "boolean",
                        "description": "Disable test linting (tests are linted by default)"
                    },
                    "warn_only": {
                        "type": "boolean",
                        "description": "Use warnings instead of errors for pedantic checks"
                    },
                    "ignore_docs": {
                        "type": "boolean",
                        "description": "Ignore missing documentation warnings"
                    }
                }
            }),
        },
        Tool {
            name: "clean".to_string(),
            description: "Clean build artifacts".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    }
                }
            }),
        },
        Tool {
            name: "search_crates".to_string(),
            description: "Search for crates on crates.io".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "query": {
                        "type": "string",
                        "description": "Search query for crate names"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results (default: 10)"
                    }
                },
                "required": ["query"]
            }),
        },
        Tool {
            name: "crate_info".to_string(),
            description: "Get detailed information about a crate including features".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "crate_name": {
                        "type": "string",
                        "description": "Name of the crate to get information about"
                    }
                },
                "required": ["crate_name"]
            }),
        },
        Tool {
            name: "add_crate".to_string(),
            description: "Add a crate dependency with optional features".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "package": {
                        "type": "string",
                        "description": "Package to add dependency to"
                    },
                    "crate_name": {
                        "type": "string",
                        "description": "Name of the crate to add"
                    },
                    "features": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Features to enable for this crate"
                    }
                },
                "required": ["crate_name"]
            }),
        },
        Tool {
            name: "remove_crate".to_string(),
            description: "Remove a crate dependency".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "package": {
                        "type": "string",
                        "description": "Package to remove dependency from"
                    },
                    "crate_name": {
                        "type": "string",
                        "description": "Name of the crate to remove"
                    }
                },
                "required": ["crate_name"]
            }),
        },
        Tool {
            name: "test".to_string(),
            description: "Run tests with optional filtering".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "working_directory": {
                        "type": "string",
                        "description": "Working directory to run cargo command in"
                    },
                    "package": {
                        "type": "string",
                        "description": "Package to test"
                    },
                    "test_name": {
                        "type": "string",
                        "description": "Specific test name to run"
                    }
                }
            }),
        }
    ]
}
