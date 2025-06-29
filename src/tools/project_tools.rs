use super::schemas::CommonSchemas;
use crate::types::Tool;
use serde_json::json;

pub fn get_project_tools() -> Vec<Tool> {
    vec![
        get_cargo_new_tool(),
        get_cargo_init_tool(),
        get_cargo_clean_tool(),
        get_cargo_doc_tool(),
    ]
}

fn get_cargo_new_tool() -> Tool {
    Tool {
        name: "new".to_string(),
        description: "Create a new cargo package at <path>".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
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
                "registry": CommonSchemas::registry()
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
                "working_directory": CommonSchemas::working_directory(),
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
                "registry": CommonSchemas::registry()
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
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
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

fn get_cargo_doc_tool() -> Tool {
    Tool {
        name: "doc".to_string(),
        description: "Build this package's and its dependencies' documentation".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package(),
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
                "release": CommonSchemas::release(),
                "target": CommonSchemas::target(),
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
                "jobs": CommonSchemas::jobs()
            }
        }),
    }
}
