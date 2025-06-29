use super::schemas::CommonSchemas;
use crate::types::Tool;
use serde_json::json;

pub fn get_dependency_tools() -> Vec<Tool> {
    vec![
        get_cargo_add_tool(),
        get_cargo_remove_tool(),
        get_cargo_update_tool(),
        get_cargo_tree_tool(),
    ]
}

fn get_cargo_add_tool() -> Tool {
    Tool {
        name: "add".to_string(),
        description: "Add dependencies to a Cargo.toml manifest file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "dependency": {
                    "type": "string",
                    "description": "Dependency to add"
                },
                "package": CommonSchemas::package(),
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
                "no_default_features": CommonSchemas::no_default_features(),
                "default_features": {
                    "type": "boolean",
                    "description": "Re-enable the default features"
                },
                "features": CommonSchemas::features(),
                "rename": {
                    "type": "string",
                    "description": "Rename the dependency"
                },
                "registry": CommonSchemas::registry(),
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

fn get_cargo_remove_tool() -> Tool {
    Tool {
        name: "remove".to_string(),
        description: "Remove dependencies from a Cargo.toml manifest file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "dependency": {
                    "type": "string",
                    "description": "Dependency to remove"
                },
                "package": CommonSchemas::package(),
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

fn get_cargo_update_tool() -> Tool {
    Tool {
        name: "update".to_string(),
        description: "Update dependencies as recorded in the local lock file".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
                "aggressive": {
                    "type": "boolean",
                    "description": "Force updating all dependencies of SPEC as well"
                },
                "precise": {
                    "type": "string",
                    "description": "Update a single dependency to exactly PRECISE"
                },
                "workspace": CommonSchemas::workspace(),
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
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
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
