use serde_json::{self, Value, json};
use crate::tools::tool::ToolMeta;

pub fn tools_list(tools: Vec<ToolMeta>) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": 1,
        "tools": tools.iter().map(|t| t.get_schema()).collect::<Vec<Value>>(),
        })
}
pub fn initialize() -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": 1,
            "protocolVersion": "2025-06-18",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "rust-mcp",
                "version": "0.0.1"
            }
    })
}
