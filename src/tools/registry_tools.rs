use super::schemas::CommonSchemas;
use crate::types::Tool;
use serde_json::json;

pub fn get_registry_tools() -> Vec<Tool> {
    vec![
        get_cargo_search_tool(),
        get_cargo_info_tool(),
        get_cargo_install_tool(),
        get_cargo_uninstall_tool(),
    ]
}

fn get_cargo_search_tool() -> Tool {
    Tool {
        name: "search".to_string(),
        description: "Search packages in crates.io".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "limit": {
                    "type": "integer",
                    "description": "Limit the number of results (default: 10, max: 100)"
                },
                "registry": CommonSchemas::registry()
            },
            "required": ["query"]
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
                "working_directory": CommonSchemas::working_directory(),
                "query": {
                    "type": "string",
                    "description": "Package name to get information about"
                },
                "registry": CommonSchemas::registry()
            },
            "required": ["query"]
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
                "working_directory": CommonSchemas::working_directory(),
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
                "registry": CommonSchemas::registry(),
                "index": {
                    "type": "string",
                    "description": "Registry index to use"
                },
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
                "target": CommonSchemas::target(),
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
                "working_directory": CommonSchemas::working_directory(),
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
