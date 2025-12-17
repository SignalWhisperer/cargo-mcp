use super::workflow_tools::get_workflow_tools;
use crate::types::Tool;

pub fn get_available_tools() -> Vec<Tool> {
    get_workflow_tools()
}
