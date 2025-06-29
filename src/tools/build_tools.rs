use super::schemas::{CommonSchemas, merge_properties};
use crate::types::Tool;
use serde_json::json;

pub fn get_build_tools() -> Vec<Tool> {
    vec![
        get_cargo_check_tool(),
        get_cargo_build_tool(),
        get_cargo_clippy_tool(),
        get_cargo_fmt_tool(),
    ]
}

fn get_cargo_check_tool() -> Tool {
    let base_properties = CommonSchemas::build_common_properties();
    let target_properties = CommonSchemas::target_selection_properties();
    let additional_properties = json!({
        "profile": CommonSchemas::profile(),
        "message_format": CommonSchemas::message_format(),
        "workspace": CommonSchemas::workspace(),
        "exclude": CommonSchemas::exclude()
    });

    let properties = merge_properties(
        merge_properties(base_properties, target_properties),
        additional_properties,
    );

    Tool {
        name: "check".to_string(),
        description: "Run cargo check to analyze code without producing executables".to_string(),
        input_schema: json!({
            "type": "object",
            "readOnly": true,
            "properties": properties
        }),
    }
}

fn get_cargo_build_tool() -> Tool {
    let base_properties = CommonSchemas::build_common_properties();
    let target_properties = CommonSchemas::target_selection_properties();
    let additional_properties = json!({
        "profile": CommonSchemas::profile(),
        "message_format": CommonSchemas::message_format(),
        "workspace": CommonSchemas::workspace(),
        "exclude": CommonSchemas::exclude()
    });

    let properties = merge_properties(
        merge_properties(base_properties, target_properties),
        additional_properties,
    );

    Tool {
        name: "build".to_string(),
        description: "Compile the current package".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": properties
        }),
    }
}

fn get_cargo_clippy_tool() -> Tool {
    let base_properties = CommonSchemas::build_common_properties();
    let target_properties = CommonSchemas::target_selection_properties();
    let clippy_properties = json!({
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
        "profile": CommonSchemas::profile(),
        "message_format": CommonSchemas::message_format(),
        "workspace": CommonSchemas::workspace(),
        "exclude": CommonSchemas::exclude()
    });

    let properties = merge_properties(
        merge_properties(base_properties, target_properties),
        clippy_properties,
    );

    Tool {
        name: "clippy".to_string(),
        description: "Run Clippy lints on the current package".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": properties
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
                "working_directory": CommonSchemas::working_directory(),
                "package": CommonSchemas::package()
            }
        }),
    }
}
