use serde_json::json;

use crate::types::Tool;

pub fn get_available_tools() -> Vec<Tool> {
    vec![
        get_cargo_check_tool(),
        get_cargo_build_tool(),
        get_cargo_run_tool(),
        get_cargo_fmt_tool(),
        get_cargo_clippy_tool(),
        get_cargo_add_tool(),
        get_cargo_test_tool(),
        get_cargo_bench_tool(),
        get_cargo_doc_tool(),
        get_cargo_clean_tool(),
        get_cargo_update_tool(),
        get_cargo_tree_tool(),
        get_cargo_search_tool(),
        get_cargo_remove_tool(),
        get_cargo_new_tool(),
        get_cargo_init_tool(),
        get_cargo_metadata_tool(),
        get_cargo_info_tool(),
        get_cargo_version_tool(),
        get_cargo_install_tool(),
        get_cargo_uninstall_tool(),
    ]
}

fn get_cargo_check_tool() -> Tool {
    Tool {
        name: "check".to_string(),
        description: "Run cargo check to analyze code without producing executables".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo check in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to check"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "release": {
                    "type": "boolean",
                    "description": "Check optimized artifacts with the release profile"
                },
                "target": {
                    "type": "string",
                    "description": "Check for the target triple"
                },
                "lib": {
                    "type": "boolean",
                    "description": "Check only this package's library"
                },
                "bin": {
                    "type": "string",
                    "description": "Check only the specified binary"
                },
                "bins": {
                    "type": "boolean",
                    "description": "Check all binaries"
                },
                "example": {
                    "type": "string",
                    "description": "Check only the specified example"
                },
                "examples": {
                    "type": "boolean",
                    "description": "Check all examples"
                },
                "test": {
                    "type": "string",
                    "description": "Check only the specified test target"
                },
                "tests": {
                    "type": "boolean",
                    "description": "Check all tests"
                },
                "bench": {
                    "type": "string",
                    "description": "Check only the specified bench target"
                },
                "benches": {
                    "type": "boolean",
                    "description": "Check all benches"
                },
                "all_targets": {
                    "type": "boolean",
                    "description": "Check all targets"
                },
                "profile": {
                    "type": "string",
                    "description": "Check artifacts with the specified profile"
                },
                "message_format": {
                    "type": "string",
                    "description": "Error format (human, short, json, json-diagnostic-short, json-diagnostic-rendered-ansi, json-render-diagnostics). Defaults to 'short' for concise agent-friendly output"
                },
                "workspace": {
                    "type": "boolean",
                    "description": "Check all packages in the workspace"
                },
                "exclude": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Exclude packages from the check"
                }
            }
        }),
    }
}

fn get_cargo_build_tool() -> Tool {
    Tool {
        name: "build".to_string(),
        description: "Compile the current package".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo build in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to build"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "release": {
                    "type": "boolean",
                    "description": "Build optimized artifacts with the release profile"
                },
                "target": {
                    "type": "string",
                    "description": "Build for the target triple"
                },
                "lib": {
                    "type": "boolean",
                    "description": "Build only this package's library"
                },
                "bin": {
                    "type": "string",
                    "description": "Build only the specified binary"
                },
                "bins": {
                    "type": "boolean",
                    "description": "Build all binaries"
                },
                "example": {
                    "type": "string",
                    "description": "Build only the specified example"
                },
                "examples": {
                    "type": "boolean",
                    "description": "Build all examples"
                },
                "test": {
                    "type": "string",
                    "description": "Build only the specified test target"
                },
                "tests": {
                    "type": "boolean",
                    "description": "Build all tests"
                },
                "bench": {
                    "type": "string",
                    "description": "Build only the specified bench target"
                },
                "benches": {
                    "type": "boolean",
                    "description": "Build all benches"
                },
                "all_targets": {
                    "type": "boolean",
                    "description": "Build all targets"
                },
                "profile": {
                    "type": "string",
                    "description": "Build artifacts with the specified profile"
                },
                "message_format": {
                    "type": "string",
                    "description": "Error format (human, short, json, json-diagnostic-short, json-diagnostic-rendered-ansi, json-render-diagnostics). Defaults to 'short' for concise agent-friendly output"
                },
                "workspace": {
                    "type": "boolean",
                    "description": "Build all packages in the workspace"
                },
                "exclude": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Exclude packages from the build"
                }
            }
        }),
    }
}

fn get_cargo_run_tool() -> Tool {
    Tool {
        name: "run".to_string(),
        description: "Run a binary or example of the local package".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo run in"
                },
                "package": {
                    "type": "string",
                    "description": "Package with the target to run"
                },
                "bin": {
                    "type": "string",
                    "description": "Name of the bin target to run"
                },
                "example": {
                    "type": "string",
                    "description": "Name of the example target to run"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "release": {
                    "type": "boolean",
                    "description": "Build in release mode, with optimizations"
                },
                "target": {
                    "type": "string",
                    "description": "Build for the target triple"
                }
            }
        }),
    }
}

fn get_cargo_fmt_tool() -> Tool {
    Tool {
        name: "fmt".to_string(),
        description: "Format Rust code using rustfmt".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo fmt in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to format"
                }
            }
        }),
    }
}

fn get_cargo_clippy_tool() -> Tool {
    Tool {
        name: "clippy".to_string(),
        description: "Run Clippy lints on the current package".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo clippy in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to run clippy on"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "release": {
                    "type": "boolean",
                    "description": "Check optimized artifacts with the release profile"
                },
                "target": {
                    "type": "string",
                    "description": "Check for the target triple"
                },
                "lib": {
                    "type": "boolean",
                    "description": "Check only this package's library"
                },
                "bin": {
                    "type": "string",
                    "description": "Check only the specified binary"
                },
                "bins": {
                    "type": "boolean",
                    "description": "Check all binaries"
                },
                "example": {
                    "type": "string",
                    "description": "Check only the specified example"
                },
                "examples": {
                    "type": "boolean",
                    "description": "Check all examples"
                },
                "test": {
                    "type": "string",
                    "description": "Check only the specified test target"
                },
                "tests": {
                    "type": "boolean",
                    "description": "Check all tests"
                },
                "bench": {
                    "type": "string",
                    "description": "Check only the specified bench target"
                },
                "benches": {
                    "type": "boolean",
                    "description": "Check all benches"
                },
                "all_targets": {
                    "type": "boolean",
                    "description": "Check all targets"
                },
                "fix": {
                    "type": "boolean",
                    "description": "Automatically apply lint suggestions"
                },
                "allow_dirty": {
                    "type": "boolean",
                    "description": "Fix code even if the working directory has changes"
                },
                "allow_staged": {
                    "type": "boolean",
                    "description": "Fix code even if the working directory has staged changes"
                },
                "profile": {
                    "type": "string",
                    "description": "Check artifacts with the specified profile"
                },
                "message_format": {
                    "type": "string",
                    "description": "Error format (human, short, json, json-diagnostic-short, json-diagnostic-rendered-ansi, json-render-diagnostics). Defaults to 'short' for concise agent-friendly output"
                },
                "workspace": {
                    "type": "boolean",
                    "description": "Check all packages in the workspace"
                },
                "exclude": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Exclude packages from the check"
                }
            }
        }),
    }
}

fn get_cargo_add_tool() -> Tool {
    Tool {
        name: "add".to_string(),
        description: "Add dependencies to a Cargo.toml manifest file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo add in"
                },
                "dependency": {
                    "type": "string",
                    "description": "Dependency to add"
                },
                "package": {
                    "type": "string",
                    "description": "Package to modify"
                },
                "dev": {
                    "type": "boolean",
                    "description": "Add as development dependency"
                },
                "build": {
                    "type": "boolean",
                    "description": "Add as build dependency"
                },
                "optional": {
                    "type": "boolean",
                    "description": "Mark the dependency as optional"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Disable the default features"
                },
                "default_features": {
                    "type": "boolean",
                    "description": "Re-enable the default features"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "rename": {
                    "type": "string",
                    "description": "Rename the dependency"
                },
                "registry": {
                    "type": "string",
                    "description": "Registry to use"
                },
                "path": {
                    "type": "string",
                    "description": "Filesystem path to local crate to add"
                },
                "git": {
                    "type": "string",
                    "description": "Git repository location"
                },
                "branch": {
                    "type": "string",
                    "description": "Git branch to download the crate from"
                },
                "tag": {
                    "type": "string",
                    "description": "Git tag to download the crate from"
                },
                "rev": {
                    "type": "string",
                    "description": "Git reference to download the crate from"
                }
            },
            "required": ["dependency"]
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
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo test in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to test"
                },
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
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "release": {
                    "type": "boolean",
                    "description": "Build artifacts in release mode, with optimizations"
                },
                "target": {
                    "type": "string",
                    "description": "Build for the target triple"
                },
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
                "jobs": {
                    "type": "integer",
                    "description": "Number of parallel jobs, defaults to # of CPUs"
                },
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
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo bench in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to benchmark"
                },
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
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "target": {
                    "type": "string",
                    "description": "Build for the target triple"
                },
                "jobs": {
                    "type": "integer",
                    "description": "Number of parallel jobs, defaults to # of CPUs"
                }
            }
        }),
    }
}

fn get_cargo_doc_tool() -> Tool {
    Tool {
        name: "doc".to_string(),
        description: "Build this package's and its dependencies' documentation".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo doc in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to document"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "release": {
                    "type": "boolean",
                    "description": "Build artifacts in release mode, with optimizations"
                },
                "target": {
                    "type": "string",
                    "description": "Build for the target triple"
                },
                "lib": {
                    "type": "boolean",
                    "description": "Document only this package's library"
                },
                "bin": {
                    "type": "string",
                    "description": "Document only the specified binary"
                },
                "bins": {
                    "type": "boolean",
                    "description": "Document all binaries"
                },
                "open": {
                    "type": "boolean",
                    "description": "Opens the docs in a browser after the operation"
                },
                "no_deps": {
                    "type": "boolean",
                    "description": "Don't build documentation for dependencies"
                },
                "document_private_items": {
                    "type": "boolean",
                    "description": "Document private items"
                },
                "jobs": {
                    "type": "integer",
                    "description": "Number of parallel jobs, defaults to # of CPUs"
                }
            }
        }),
    }
}

fn get_cargo_clean_tool() -> Tool {
    Tool {
        name: "clean".to_string(),
        description: "Remove artifacts that cargo has generated in the past".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo clean in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to clean"
                },
                "release": {
                    "type": "boolean",
                    "description": "Whether or not to clean release artifacts"
                },
                "target": {
                    "type": "string",
                    "description": "Target triple to clean output for"
                }
            }
        }),
    }
}

fn get_cargo_update_tool() -> Tool {
    Tool {
        name: "update".to_string(),
        description: "Update dependencies as recorded in the local lock file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo update in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to update"
                },
                "aggressive": {
                    "type": "boolean",
                    "description": "Force updating all dependencies of SPEC as well"
                },
                "precise": {
                    "type": "string",
                    "description": "Update a single dependency to exactly PRECISE"
                },
                "workspace": {
                    "type": "boolean",
                    "description": "Attempt to update only packages in the workspace"
                },
                "dry_run": {
                    "type": "boolean",
                    "description": "Don't actually write the lockfile"
                }
            }
        }),
    }
}

fn get_cargo_tree_tool() -> Tool {
    Tool {
        name: "tree".to_string(),
        description: "Display a tree visualization of a dependency graph".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo tree in"
                },
                "package": {
                    "type": "string",
                    "description": "Package to be used as the root of the tree"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "target": {
                    "type": "string",
                    "description": "Filter dependencies matching the given target-triple"
                },
                "edges": {
                    "type": "string",
                    "description": "The kinds of dependencies to display (features, normal, build, dev, all)"
                },
                "invert": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Invert the dependency graph and display the packages that depend on the given package"
                },
                "prune": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Prune the given package from the display of the dependency tree"
                },
                "depth": {
                    "type": "integer",
                    "description": "Maximum display depth of the dependency tree"
                },
                "prefix": {
                    "type": "string",
                    "description": "How to display the dependency tree (indent, depth, none)"
                },
                "no_dedupe": {
                    "type": "boolean",
                    "description": "Repeat shared dependencies for each package"
                },
                "duplicates": {
                    "type": "boolean",
                    "description": "Show only dependencies which come in multiple versions"
                },
                "charset": {
                    "type": "string",
                    "description": "Character set to use in output (utf8, ascii)"
                },
                "format": {
                    "type": "string",
                    "description": "Format string used for printing dependencies"
                }
            }
        }),
    }
}

fn get_cargo_search_tool() -> Tool {
    Tool {
        name: "search".to_string(),
        description: "Search packages in crates.io".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo search in"
                },
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "limit": {
                    "type": "integer",
                    "description": "Limit the number of results (default: 10, max: 100)"
                },
                "registry": {
                    "type": "string",
                    "description": "Registry to use"
                }
            },
            "required": ["query"]
        }),
    }
}

fn get_cargo_remove_tool() -> Tool {
    Tool {
        name: "remove".to_string(),
        description: "Remove dependencies from a Cargo.toml manifest file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo remove in"
                },
                "dependency": {
                    "type": "string",
                    "description": "Dependency to remove"
                },
                "package": {
                    "type": "string",
                    "description": "Package to modify"
                },
                "dev": {
                    "type": "boolean",
                    "description": "Remove as development dependency"
                },
                "build": {
                    "type": "boolean",
                    "description": "Remove as build dependency"
                }
            },
            "required": ["dependency"]
        }),
    }
}

fn get_cargo_new_tool() -> Tool {
    Tool {
        name: "new".to_string(),
        description: "Create a new cargo package at <path>".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo new in"
                },
                "path": {
                    "type": "string",
                    "description": "Path where the new package will be created"
                },
                "bin_template": {
                    "type": "boolean",
                    "description": "Use a binary (application) template [default]"
                },
                "lib_template": {
                    "type": "boolean",
                    "description": "Use a library template"
                },
                "name": {
                    "type": "string",
                    "description": "Set the resulting package name, defaults to the directory name"
                },
                "edition": {
                    "type": "string",
                    "description": "Edition to set for the crate generated"
                },
                "registry": {
                    "type": "string",
                    "description": "Registry to use"
                }
            },
            "required": ["path"]
        }),
    }
}

fn get_cargo_init_tool() -> Tool {
    Tool {
        name: "init".to_string(),
        description: "Create a new cargo package in an existing directory".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo init in"
                },
                "path": {
                    "type": "string",
                    "description": "Path to initialize (defaults to current directory)"
                },
                "bin_template": {
                    "type": "boolean",
                    "description": "Use a binary (application) template [default]"
                },
                "lib_template": {
                    "type": "boolean",
                    "description": "Use a library template"
                },
                "name": {
                    "type": "string",
                    "description": "Set the resulting package name, defaults to the directory name"
                },
                "edition": {
                    "type": "string",
                    "description": "Edition to set for the crate generated"
                },
                "registry": {
                    "type": "string",
                    "description": "Registry to use"
                }
            }
        }),
    }
}

fn get_cargo_metadata_tool() -> Tool {
    Tool {
        name: "metadata".to_string(),
        description: "Output the resolved dependencies of a package in machine-readable format"
            .to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo metadata in"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "no_deps": {
                    "type": "boolean",
                    "description": "Output information only about the workspace members and don't fetch dependencies"
                },
                "format_version": {
                    "type": "integer",
                    "description": "Format version"
                }
            }
        }),
    }
}

fn get_cargo_info_tool() -> Tool {
    Tool {
        name: "info".to_string(),
        description: "Display information about a package in the registry".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo info in"
                },
                "query": {
                    "type": "string",
                    "description": "Package name to get information about"
                },
                "registry": {
                    "type": "string",
                    "description": "Registry to use"
                }
            },
            "required": ["query"]
        }),
    }
}

fn get_cargo_version_tool() -> Tool {
    Tool {
        name: "version".to_string(),
        description: "Show version information for cargo and rust".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo version in"
                }
            }
        }),
    }
}

fn get_cargo_install_tool() -> Tool {
    Tool {
        name: "install".to_string(),
        description: "Install a Rust binary".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo install in"
                },
                "query": {
                    "type": "string",
                    "description": "Package name to install"
                },
                "version": {
                    "type": "string",
                    "description": "Specify a version to install"
                },
                "git_url": {
                    "type": "string",
                    "description": "Git URL to install from"
                },
                "branch_install": {
                    "type": "string",
                    "description": "Branch to use when installing from git"
                },
                "tag_install": {
                    "type": "string",
                    "description": "Tag to use when installing from git"
                },
                "rev_install": {
                    "type": "string",
                    "description": "Specific commit to use when installing from git"
                },
                "path_install": {
                    "type": "string",
                    "description": "Filesystem path to local crate to install"
                },
                "list": {
                    "type": "boolean",
                    "description": "List all installed packages and their versions"
                },
                "force": {
                    "type": "boolean",
                    "description": "Force overwriting existing crates or binaries"
                },
                "no_track": {
                    "type": "boolean",
                    "description": "Do not save tracking information"
                },
                "bin_install": {
                    "type": "string",
                    "description": "Install only the specified binary"
                },
                "bins_install": {
                    "type": "boolean",
                    "description": "Install all binaries"
                },
                "example_install": {
                    "type": "string",
                    "description": "Install only the specified example"
                },
                "examples_install": {
                    "type": "boolean",
                    "description": "Install all examples"
                },
                "root": {
                    "type": "string",
                    "description": "Directory to install packages into"
                },
                "registry": {
                    "type": "string",
                    "description": "Registry to use"
                },
                "index": {
                    "type": "string",
                    "description": "Registry index to use"
                },
                "features": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Space or comma separated list of features to activate"
                },
                "all_features": {
                    "type": "boolean",
                    "description": "Activate all available features"
                },
                "no_default_features": {
                    "type": "boolean",
                    "description": "Do not activate the default feature"
                },
                "target": {
                    "type": "string",
                    "description": "Install for the target triple"
                },
                "locked": {
                    "type": "boolean",
                    "description": "Assert that Cargo.lock will remain unchanged"
                }
            }
        }),
    }
}

fn get_cargo_uninstall_tool() -> Tool {
    Tool {
        name: "uninstall".to_string(),
        description: "Remove a Rust binary".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": {
                    "type": "string",
                    "description": "Working directory to run cargo uninstall in"
                },
                "query": {
                    "type": "string",
                    "description": "Package name to uninstall"
                },
                "bin_install": {
                    "type": "string",
                    "description": "Only uninstall the binary NAME"
                },
                "root": {
                    "type": "string",
                    "description": "Directory to uninstall packages from"
                }
            },
            "required": ["query"]
        }),
    }
}
