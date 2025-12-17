use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::types::CargoToolParams;

fn handle_clean(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("clean");

    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo clean")?;

    let result = if output.status.success() {
        "Clean completed successfully".to_string()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("Clean failed: {}", stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_search_crates(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("search");

    if let Some(query) = &params.query {
        cmd.arg(query);
    }

    if let Some(limit) = params.limit {
        cmd.arg("--limit").arg(limit.to_string());
    }

    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo search")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        if stdout.is_empty() {
            "No crates found".to_string()
        } else {
            stdout.to_string()
        }
    } else {
        format!("Search failed: {}", stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_crate_info(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("info");

    if let Some(crate_name) = &params.crate_name {
        cmd.arg(crate_name);
    }

    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo info")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        stdout.to_string()
    } else {
        format!("Info failed: {}", stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_add_crate(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("add");

    if let Some(crate_name) = &params.crate_name {
        cmd.arg(crate_name);
    }

    if let Some(package) = &params.package {
        cmd.arg("-p").arg(package);
    }

    if let Some(features) = &params.features {
        if !features.is_empty() {
            cmd.arg("--features").arg(features.join(","));
        }
    }

    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo add")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        format!("Added crate successfully\n{}{}", stdout, stderr)
    } else {
        format!("Add crate failed: {}{}", stdout, stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_remove_crate(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("remove");

    if let Some(crate_name) = &params.crate_name {
        cmd.arg(crate_name);
    }

    if let Some(package) = &params.package {
        cmd.arg("-p").arg(package);
    }

    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo remove")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        format!("Removed crate successfully\n{}{}", stdout, stderr)
    } else {
        format!("Remove crate failed: {}{}", stdout, stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_test(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("test");

    if let Some(package) = &params.package {
        cmd.arg("-p").arg(package);
    }

    if let Some(test_name) = &params.test_name {
        cmd.arg(test_name);
    }

    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo test")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        format!("{}{}", stdout, stderr)
    } else {
        format!("Tests failed:\n{}{}", stdout, stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_lint(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("clippy");

    // Always include tests unless explicitly disabled
    if !params.no_tests.unwrap_or(false) {
        cmd.arg("--tests");
    }

    // Add package if specified
    if let Some(package) = &params.package {
        cmd.arg("-p").arg(package);
    }

    // Set working directory if specified
    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    // Build clippy arguments
    cmd.arg("--");

    let level = if params.warn_only.unwrap_or(false) {
        "-W"
    } else {
        "-D"
    };
    cmd.arg(format!("{}clippy::pedantic", level));

    if params.ignore_docs.unwrap_or(false) {
        cmd.arg("-Aclippy::missing_docs");
    }

    let output = cmd.output().context("Failed to execute cargo clippy")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        if stdout.is_empty() && stderr.is_empty() {
            "Linting completed successfully".to_string()
        } else {
            format!("{}{}", stdout, stderr)
        }
    } else {
        format!("Linting failed:\n{}{}", stdout, stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_build(params: &CargoToolParams) -> Result<Value> {
    // Run cargo fmt first
    let mut fmt_cmd = Command::new("cargo");
    fmt_cmd.arg("fmt");

    if let Some(package) = &params.package {
        fmt_cmd.arg("-p").arg(package);
    }

    if let Some(working_dir) = &params.working_directory {
        fmt_cmd.current_dir(working_dir);
    }

    let fmt_output = fmt_cmd.output().context("Failed to execute cargo fmt")?;

    if !fmt_output.status.success() {
        let fmt_stderr = String::from_utf8_lossy(&fmt_output.stderr);
        return Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Formatting failed: {}", fmt_stderr)
            }]
        }));
    }

    // Now run cargo build
    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    // Add release flag if specified
    if params.release.unwrap_or(false) {
        cmd.arg("--release");
    }

    // Add package if specified
    if let Some(package) = &params.package {
        cmd.arg("-p").arg(package);
    }

    // Set working directory if specified
    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo build")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let result = if output.status.success() {
        let build_result = if stdout.is_empty() && stderr.is_empty() {
            "Build completed successfully".to_string()
        } else {
            format!("{}{}", stdout, stderr)
        };
        format!("Code formatted successfully\n{}", build_result)
    } else {
        format!("Build failed:\n{}{}", stdout, stderr)
    };

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

fn handle_pre_build(params: &CargoToolParams) -> Result<Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg("check");
    cmd.env("CARGO_BUILD_WARNINGS", "allow");

    // Always include tests unless explicitly disabled
    if !params.no_tests.unwrap_or(false) {
        cmd.arg("--tests");
    }

    // Add package if specified
    if let Some(package) = &params.package {
        cmd.arg("-p").arg(package);
    }

    // Set working directory if specified
    if let Some(working_dir) = &params.working_directory {
        cmd.current_dir(working_dir);
    }

    let output = cmd.output().context("Failed to execute cargo check")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut result = if output.status.success() {
        if stdout.is_empty() && stderr.is_empty() {
            "Pre-build check completed successfully".to_string()
        } else {
            format!("{}{}", stdout, stderr)
        }
    } else {
        return Ok(json!({
            "content": [{
                "type": "text",
                "text": format!("Pre-build check failed:\n{}{}", stdout, stderr)
            }]
        }));
    };

    // Run cargo fmt after successful check
    let mut fmt_cmd = Command::new("cargo");
    fmt_cmd.arg("fmt");

    if let Some(package) = &params.package {
        fmt_cmd.arg("-p").arg(package);
    }

    if let Some(working_dir) = &params.working_directory {
        fmt_cmd.current_dir(working_dir);
    }

    let fmt_output = fmt_cmd.output().context("Failed to execute cargo fmt")?;

    if fmt_output.status.success() {
        result.push_str("\nCode formatted successfully");
    } else {
        let fmt_stderr = String::from_utf8_lossy(&fmt_output.stderr);
        result.push_str(&format!("\nFormatting failed: {}", fmt_stderr));
    }

    Ok(json!({
        "content": [{
            "type": "text",
            "text": result
        }]
    }))
}

pub fn execute_cargo_command(subcommand: &str, params: &CargoToolParams) -> Result<String> {
    let mut cmd = Command::new("cargo");
    cmd.arg(subcommand);

    if let Some(ref working_dir) = params.working_directory {
        cmd.current_dir(working_dir);
    }

    match subcommand {
        "check" | "build" | "clippy" | "test" | "bench" | "doc" => {
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if let Some(ref features) = params.features {
                cmd.args(["--features", &features.join(",")]);
            }
            if params.all_features.unwrap_or(false) {
                cmd.arg("--all-features");
            }
            if params.no_default_features.unwrap_or(false) {
                cmd.arg("--no-default-features");
            }
            if params.release.unwrap_or(false) {
                cmd.arg("--release");
            }
            if let Some(ref target) = params.target {
                cmd.args(["--target", target]);
            }
            if params.lib.unwrap_or(false) {
                cmd.arg("--lib");
            }
            if let Some(ref bin) = params.bin {
                cmd.args(["--bin", bin]);
            }
            if params.bins.unwrap_or(false) {
                cmd.arg("--bins");
            }
            if let Some(ref example) = params.example {
                cmd.args(["--example", example]);
            }
            if params.examples.unwrap_or(false) {
                cmd.arg("--examples");
            }
            if let Some(ref test) = params.test {
                cmd.args(["--test", test]);
            }
            if params.tests.unwrap_or(false) {
                cmd.arg("--tests");
            }
            if let Some(ref bench) = params.bench {
                cmd.args(["--bench", bench]);
            }
            if params.benches.unwrap_or(false) {
                cmd.arg("--benches");
            }
            if params.all_targets.unwrap_or(false) {
                cmd.arg("--all-targets");
            }
            if let Some(jobs) = params.jobs {
                cmd.args(["--jobs", &jobs.to_string()]);
            }

            // Agent-friendly defaults and options
            if let Some(ref profile) = params.profile {
                cmd.args(["--profile", profile]);
            }

            // Default to short message format for better agent parsing, but allow override
            let message_format = params.message_format.as_deref().unwrap_or("short");
            if subcommand == "check" || subcommand == "build" || subcommand == "clippy" {
                cmd.args(["--message-format", message_format]);
            }

            if params.workspace.unwrap_or(false) {
                cmd.arg("--workspace");
            }

            if let Some(ref exclude) = params.exclude {
                for pkg in exclude {
                    cmd.args(["--exclude", pkg]);
                }
            }

            // Command-specific options
            if subcommand == "clippy" {
                if params.fix.unwrap_or(false) {
                    cmd.arg("--fix");
                }
                if params.allow_dirty.unwrap_or(false) {
                    cmd.arg("--allow-dirty");
                }
                if params.allow_staged.unwrap_or(false) {
                    cmd.arg("--allow-staged");
                }
            }

            if subcommand == "test" {
                if params.nocapture.unwrap_or(false) {
                    cmd.arg("--nocapture");
                }
                if params.ignored.unwrap_or(false) {
                    cmd.arg("--ignored");
                }
                if params.include_ignored.unwrap_or(false) {
                    cmd.arg("--include-ignored");
                }
                if params.exact.unwrap_or(false) {
                    cmd.arg("--exact");
                }
                if let Some(test_threads) = params.test_threads {
                    cmd.args(["--test-threads", &test_threads.to_string()]);
                }
            }

            if subcommand == "doc" {
                if params.open.unwrap_or(false) {
                    cmd.arg("--open");
                }
                if params.no_deps.unwrap_or(false) {
                    cmd.arg("--no-deps");
                }
                if params.document_private_items.unwrap_or(false) {
                    cmd.arg("--document-private-items");
                }
            }
        }
        "run" => {
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if let Some(ref bin) = params.bin {
                cmd.args(["--bin", bin]);
            }
            if let Some(ref example) = params.example {
                cmd.args(["--example", example]);
            }
            if let Some(ref features) = params.features {
                cmd.args(["--features", &features.join(",")]);
            }
            if params.all_features.unwrap_or(false) {
                cmd.arg("--all-features");
            }
            if params.no_default_features.unwrap_or(false) {
                cmd.arg("--no-default-features");
            }
            if params.release.unwrap_or(false) {
                cmd.arg("--release");
            }
            if let Some(ref target) = params.target {
                cmd.args(["--target", target]);
            }
        }
        "fmt" => {
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
        }
        "add" => {
            if let Some(ref dependency) = params.dependency {
                cmd.arg(dependency);
            }
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if params.dev.unwrap_or(false) {
                cmd.arg("--dev");
            }
            if params.build.unwrap_or(false) {
                cmd.arg("--build");
            }
            if params.optional.unwrap_or(false) {
                cmd.arg("--optional");
            }
            if params.no_default_features_dep.unwrap_or(false) {
                cmd.arg("--no-default-features");
            }
            if params.default_features.unwrap_or(false) {
                cmd.arg("--default-features");
            }
            if let Some(ref features) = params.features_dep {
                cmd.args(["--features", &features.join(",")]);
            }
            if let Some(ref rename) = params.rename {
                cmd.args(["--rename", rename]);
            }
            if let Some(ref registry) = params.registry {
                cmd.args(["--registry", registry]);
            }
            if let Some(ref path) = params.path {
                cmd.args(["--path", path]);
            }
            if let Some(ref git) = params.git {
                cmd.args(["--git", git]);
            }
            if let Some(ref branch) = params.branch {
                cmd.args(["--branch", branch]);
            }
            if let Some(ref tag) = params.tag {
                cmd.args(["--tag", tag]);
            }
            if let Some(ref rev) = params.rev {
                cmd.args(["--rev", rev]);
            }
        }
        "remove" => {
            if let Some(ref dependency) = params.dependency {
                cmd.arg(dependency);
            }
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if params.dev.unwrap_or(false) {
                cmd.arg("--dev");
            }
            if params.build.unwrap_or(false) {
                cmd.arg("--build");
            }
        }
        "clean" => {
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if params.release.unwrap_or(false) {
                cmd.arg("--release");
            }
            if let Some(ref target) = params.target {
                cmd.args(["--target", target]);
            }
        }
        "update" => {
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if params.aggressive.unwrap_or(false) {
                cmd.arg("--aggressive");
            }
            if let Some(ref precise) = params.precise {
                cmd.args(["--precise", precise]);
            }
            if params.workspace.unwrap_or(false) {
                cmd.arg("--workspace");
            }
            if params.dry_run.unwrap_or(false) {
                cmd.arg("--dry-run");
            }
        }
        "tree" => {
            if let Some(ref package) = params.package {
                cmd.args(["--package", package]);
            }
            if let Some(ref features) = params.features {
                cmd.args(["--features", &features.join(",")]);
            }
            if params.all_features.unwrap_or(false) {
                cmd.arg("--all-features");
            }
            if params.no_default_features.unwrap_or(false) {
                cmd.arg("--no-default-features");
            }
            if let Some(ref target) = params.target {
                cmd.args(["--target", target]);
            }
            if let Some(ref edges) = params.edges {
                cmd.args(["--edges", edges]);
            }
            if let Some(ref invert) = params.invert {
                for inv in invert {
                    cmd.args(["--invert", inv]);
                }
            }
            if let Some(ref prune) = params.prune {
                for pr in prune {
                    cmd.args(["--prune", pr]);
                }
            }
            if let Some(depth) = params.depth {
                cmd.args(["--depth", &depth.to_string()]);
            }
            if let Some(ref prefix) = params.prefix {
                cmd.args(["--prefix", prefix]);
            }
            if params.duplicates.unwrap_or(false) {
                cmd.arg("--duplicates");
            }
            if let Some(ref charset) = params.charset {
                cmd.args(["--charset", charset]);
            }
            if let Some(ref format) = params.format {
                cmd.args(["--format", format]);
            }
        }
        "search" => {
            if let Some(ref query) = params.query {
                cmd.arg(query);
            }
            if let Some(limit) = params.limit {
                cmd.args(["--limit", &limit.to_string()]);
            }
            if let Some(ref registry) = params.registry_search {
                cmd.args(["--registry", registry]);
            }
        }
        "new" => {
            if let Some(ref path) = params.path {
                cmd.arg(path);
            }
            if params.bin_template.unwrap_or(false) {
                cmd.arg("--bin");
            }
            if params.lib_template.unwrap_or(false) {
                cmd.arg("--lib");
            }
            if let Some(ref name) = params.name {
                cmd.args(["--name", name]);
            }
            if let Some(ref edition) = params.edition {
                cmd.args(["--edition", edition]);
            }
            if let Some(ref registry) = params.registry {
                cmd.args(["--registry", registry]);
            }
        }
        "init" => {
            if let Some(ref path) = params.path {
                cmd.arg(path);
            }
            if params.bin_template.unwrap_or(false) {
                cmd.arg("--bin");
            }
            if params.lib_template.unwrap_or(false) {
                cmd.arg("--lib");
            }
            if let Some(ref name) = params.name {
                cmd.args(["--name", name]);
            }
            if let Some(ref edition) = params.edition {
                cmd.args(["--edition", edition]);
            }
            if let Some(ref registry) = params.registry {
                cmd.args(["--registry", registry]);
            }
        }
        "metadata" => {
            if let Some(ref features) = params.features {
                cmd.args(["--features", &features.join(",")]);
            }
            if params.all_features.unwrap_or(false) {
                cmd.arg("--all-features");
            }
            if params.no_default_features.unwrap_or(false) {
                cmd.arg("--no-default-features");
            }
            if params.no_deps.unwrap_or(false) {
                cmd.arg("--no-deps");
            }
            if let Some(format_version) = params.format_version {
                cmd.args(["--format-version", &format_version.to_string()]);
            }
        }
        "info" => {
            if let Some(ref query) = params.query {
                cmd.arg(query);
            }
            if let Some(ref registry) = params.registry {
                cmd.args(["--registry", registry]);
            }
        }
        "version" => {
            // cargo version doesn't take additional arguments
        }
        "install" => {
            if params.list.unwrap_or(false) {
                cmd.arg("--list");
            } else if let Some(ref query) = params.query {
                cmd.arg(query);
            }
            if let Some(ref version) = params.version {
                cmd.args(["--version", version]);
            }
            if let Some(ref git_url) = params.git_url {
                cmd.args(["--git", git_url]);
            }
            if let Some(ref branch) = params.branch_install {
                cmd.args(["--branch", branch]);
            }
            if let Some(ref tag) = params.tag_install {
                cmd.args(["--tag", tag]);
            }
            if let Some(ref rev) = params.rev_install {
                cmd.args(["--rev", rev]);
            }
            if let Some(ref path) = params.path_install {
                cmd.args(["--path", path]);
            }
            if params.force.unwrap_or(false) {
                cmd.arg("--force");
            }
            if params.no_track.unwrap_or(false) {
                cmd.arg("--no-track");
            }
            if let Some(ref bin) = params.bin_install {
                cmd.args(["--bin", bin]);
            }
            if params.bins_install.unwrap_or(false) {
                cmd.arg("--bins");
            }
            if let Some(ref example) = params.example_install {
                cmd.args(["--example", example]);
            }
            if params.examples_install.unwrap_or(false) {
                cmd.arg("--examples");
            }
            if let Some(ref root) = params.root {
                cmd.args(["--root", root]);
            }
            if let Some(ref registry) = params.registry {
                cmd.args(["--registry", registry]);
            }
            if let Some(ref index) = params.index {
                cmd.args(["--index", index]);
            }
            if let Some(ref features) = params.features {
                cmd.args(["--features", &features.join(",")]);
            }
            if params.all_features.unwrap_or(false) {
                cmd.arg("--all-features");
            }
            if params.no_default_features.unwrap_or(false) {
                cmd.arg("--no-default-features");
            }
            if let Some(ref target) = params.target {
                cmd.args(["--target", target]);
            }
            if params.locked.unwrap_or(false) {
                cmd.arg("--locked");
            }
        }
        "uninstall" => {
            if let Some(ref query) = params.query {
                cmd.arg(query);
            }
            if let Some(ref bin) = params.bin_install {
                cmd.args(["--bin", bin]);
            }
            if let Some(ref root) = params.root {
                cmd.args(["--root", root]);
            }
        }
        _ => {}
    }

    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute cargo command")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined_output = if stderr.is_empty() {
        stdout.to_string()
    } else if stdout.is_empty() {
        stderr.to_string()
    } else {
        format!("STDOUT:\n{stdout}\n\nSTDERR:\n{stderr}")
    };

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Cargo command failed with exit code {}: {}",
            output.status.code().unwrap_or(-1),
            combined_output
        ));
    }

    Ok(combined_output)
}

pub fn handle_tool_call(tool_name: &str, params: Value) -> Result<Value> {
    let start = Instant::now();

    let cargo_params: CargoToolParams =
        serde_json::from_value(params).context("Failed to parse tool parameters")?;

    let mut result = match tool_name {
        "pre_build" => handle_pre_build(&cargo_params),
        "build" => handle_build(&cargo_params),
        "lint" => handle_lint(&cargo_params),
        "test" => handle_test(&cargo_params),
        "clean" => handle_clean(&cargo_params),
        "search_crates" => handle_search_crates(&cargo_params),
        "crate_info" => handle_crate_info(&cargo_params),
        "add_crate" => handle_add_crate(&cargo_params),
        "remove_crate" => handle_remove_crate(&cargo_params),
        _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
    }?;

    // Update the execution time in the result
    if let Some(content) = result.get_mut("content") {
        if let Some(content_array) = content.as_array_mut() {
            if let Some(first_item) = content_array.get_mut(0) {
                if let Some(obj) = first_item.as_object_mut() {
                    obj.insert(
                        "execution_time_ms".to_string(),
                        json!(start.elapsed().as_millis()),
                    );
                }
            }
        }
    }

    Ok(result)
}
