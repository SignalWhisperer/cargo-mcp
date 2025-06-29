use serde_json::{Value, json};

/// Common schema properties used across multiple tools
pub struct CommonSchemas;

impl CommonSchemas {
    pub fn working_directory() -> Value {
        json!({
            "type": "string",
            "description": "Working directory to run cargo command in"
        })
    }

    pub fn package() -> Value {
        json!({
            "type": "string",
            "description": "Package to operate on"
        })
    }

    pub fn features() -> Value {
        json!({
            "type": "array",
            "items": {"type": "string"},
            "description": "Space or comma separated list of features to activate"
        })
    }

    pub fn all_features() -> Value {
        json!({
            "type": "boolean",
            "description": "Activate all available features"
        })
    }

    pub fn no_default_features() -> Value {
        json!({
            "type": "boolean",
            "description": "Do not activate the default feature"
        })
    }

    pub fn release() -> Value {
        json!({
            "type": "boolean",
            "description": "Build artifacts in release mode, with optimizations"
        })
    }

    pub fn target() -> Value {
        json!({
            "type": "string",
            "description": "Build for the target triple"
        })
    }

    pub fn target_selection_properties() -> Value {
        json!({
            "lib": {
                "type": "boolean",
                "description": "Only this package's library"
            },
            "bin": {
                "type": "string",
                "description": "Only the specified binary"
            },
            "bins": {
                "type": "boolean",
                "description": "All binaries"
            },
            "example": {
                "type": "string",
                "description": "Only the specified example"
            },
            "examples": {
                "type": "boolean",
                "description": "All examples"
            },
            "test": {
                "type": "string",
                "description": "Only the specified test target"
            },
            "tests": {
                "type": "boolean",
                "description": "All tests"
            },
            "bench": {
                "type": "string",
                "description": "Only the specified bench target"
            },
            "benches": {
                "type": "boolean",
                "description": "All benches"
            },
            "all_targets": {
                "type": "boolean",
                "description": "All targets"
            }
        })
    }

    pub fn build_common_properties() -> Value {
        json!({
            "working_directory": Self::working_directory(),
            "package": Self::package(),
            "features": Self::features(),
            "all_features": Self::all_features(),
            "no_default_features": Self::no_default_features(),
            "release": Self::release(),
            "target": Self::target()
        })
    }

    pub fn message_format() -> Value {
        json!({
            "type": "string",
            "description": "Error format (human, short, json, json-diagnostic-short, json-diagnostic-rendered-ansi, json-render-diagnostics). Defaults to 'short' for concise agent-friendly output"
        })
    }

    pub fn profile() -> Value {
        json!({
            "type": "string",
            "description": "Build artifacts with the specified profile"
        })
    }

    pub fn workspace() -> Value {
        json!({
            "type": "boolean",
            "description": "Build all packages in the workspace"
        })
    }

    pub fn exclude() -> Value {
        json!({
            "type": "array",
            "items": {"type": "string"},
            "description": "Exclude packages from the operation"
        })
    }

    pub fn jobs() -> Value {
        json!({
            "type": "integer",
            "description": "Number of parallel jobs, defaults to # of CPUs"
        })
    }

    pub fn registry() -> Value {
        json!({
            "type": "string",
            "description": "Registry to use"
        })
    }
}

/// Helper function to merge JSON objects
pub fn merge_properties(base: Value, additional: Value) -> Value {
    let mut result = base;
    if let (Some(base_obj), Some(add_obj)) = (result.as_object_mut(), additional.as_object()) {
        for (key, value) in add_obj {
            base_obj.insert(key.clone(), value.clone());
        }
    }
    result
}
