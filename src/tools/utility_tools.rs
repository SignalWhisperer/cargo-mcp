use super::schemas::CommonSchemas;
use crate::types::Tool;
use serde_json::json;

pub fn get_utility_tools() -> Vec<Tool> {
    vec![get_cargo_metadata_tool(), get_cargo_version_tool()]
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
                "working_directory": CommonSchemas::working_directory(),
                "features": CommonSchemas::features(),
                "all_features": CommonSchemas::all_features(),
                "no_default_features": CommonSchemas::no_default_features(),
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

fn get_cargo_version_tool() -> Tool {
    Tool {
        name: "version".to_string(),
        description: "Show version information for cargo and rust".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": {
                "working_directory": CommonSchemas::working_directory()
            }
        }),
    }
}
