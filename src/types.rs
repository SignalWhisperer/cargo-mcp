use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::McpError;

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoToolParams {
    #[serde(default)]
    pub working_directory: Option<String>,
    #[serde(default)]
    pub package: Option<String>,
    #[serde(default)]
    pub features: Option<Vec<String>>,
    #[serde(default)]
    pub all_features: Option<bool>,
    #[serde(default)]
    pub no_default_features: Option<bool>,
    #[serde(default)]
    pub release: Option<bool>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub bin: Option<String>,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub test: Option<String>,
    #[serde(default)]
    pub bench: Option<String>,
    #[serde(default)]
    pub lib: Option<bool>,
    #[serde(default)]
    pub bins: Option<bool>,
    #[serde(default)]
    pub examples: Option<bool>,
    #[serde(default)]
    pub tests: Option<bool>,
    #[serde(default)]
    pub benches: Option<bool>,
    #[serde(default)]
    pub all_targets: Option<bool>,
    #[serde(default)]
    pub fix: Option<bool>,
    #[serde(default)]
    pub allow_dirty: Option<bool>,
    #[serde(default)]
    pub allow_staged: Option<bool>,
    #[serde(default)]
    pub dependency: Option<String>,
    #[serde(default)]
    pub dev: Option<bool>,
    #[serde(default)]
    pub build: Option<bool>,
    #[serde(default)]
    pub optional: Option<bool>,
    #[serde(default)]
    pub no_default_features_dep: Option<bool>,
    #[serde(default)]
    pub default_features: Option<bool>,
    #[serde(default)]
    pub features_dep: Option<Vec<String>>,
    #[serde(default)]
    pub rename: Option<String>,
    #[serde(default)]
    pub registry: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub git: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub rev: Option<String>,
    // New parameters for additional commands
    #[serde(default)]
    pub nocapture: Option<bool>,
    #[serde(default)]
    pub ignored: Option<bool>,
    #[serde(default)]
    pub include_ignored: Option<bool>,
    #[serde(default)]
    pub exact: Option<bool>,
    #[serde(default)]
    pub jobs: Option<u32>,
    #[serde(default)]
    pub test_threads: Option<u32>,
    #[serde(default)]
    pub open: Option<bool>,
    #[serde(default)]
    pub no_deps: Option<bool>,
    #[serde(default)]
    pub document_private_items: Option<bool>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub registry_search: Option<String>,
    #[serde(default)]
    pub duplicates: Option<bool>,
    #[serde(default)]
    pub edges: Option<String>,
    #[serde(default)]
    pub invert: Option<Vec<String>>,
    #[serde(default)]
    pub prune: Option<Vec<String>>,
    #[serde(default)]
    pub depth: Option<u32>,
    #[serde(default)]
    pub prefix: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub charset: Option<String>,
    #[serde(default)]
    pub precise: Option<String>,
    #[serde(default)]
    pub aggressive: Option<bool>,
    #[serde(default)]
    pub workspace: Option<bool>,
    #[serde(default)]
    pub dry_run: Option<bool>,
    #[serde(default)]
    pub bin_template: Option<bool>,
    #[serde(default)]
    pub lib_template: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub edition: Option<String>,
    #[serde(default)]
    pub format_version: Option<u32>,
    // Parameters for install/uninstall
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub git_url: Option<String>,
    #[serde(default)]
    pub branch_install: Option<String>,
    #[serde(default)]
    pub tag_install: Option<String>,
    #[serde(default)]
    pub rev_install: Option<String>,
    #[serde(default)]
    pub path_install: Option<String>,
    #[serde(default)]
    pub list: Option<bool>,
    #[serde(default)]
    pub force: Option<bool>,
    #[serde(default)]
    pub no_track: Option<bool>,
    #[serde(default)]
    pub bin_install: Option<String>,
    #[serde(default)]
    pub bins_install: Option<bool>,
    #[serde(default)]
    pub example_install: Option<String>,
    #[serde(default)]
    pub examples_install: Option<bool>,
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub index: Option<String>,
    #[serde(default)]
    pub locked: Option<bool>,
    // Agent-friendly options
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub message_format: Option<String>,
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}
