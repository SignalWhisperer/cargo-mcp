use super::{
    build_tools::get_build_tools, dependency_tools::get_dependency_tools,
    execution_tools::get_execution_tools, project_tools::get_project_tools,
    registry_tools::get_registry_tools, utility_tools::get_utility_tools,
};
use crate::types::Tool;

pub fn get_available_tools() -> Vec<Tool> {
    let mut tools = Vec::new();

    tools.extend(get_build_tools());
    tools.extend(get_execution_tools());
    tools.extend(get_dependency_tools());
    tools.extend(get_project_tools());
    tools.extend(get_registry_tools());
    tools.extend(get_utility_tools());

    tools
}
