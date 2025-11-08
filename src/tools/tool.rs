use std::sync::Arc;
use crate::tools::context::Context;
use jsonrpsee::server::RpcModule;
use serde::{Deserialize, Serialize};
use serde_json::{self, Deserializer, Map, Serializer, Value, json};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolField {
    pub name: &'static str,
    pub description: &'static str,
    pub type_: &'static str,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub fields: Vec<ToolField>,
}

impl ToolMeta {
    pub fn get_schema(&self) -> serde_json::Value {
        let required_values = self
            .fields
            .iter()
            .filter(|f| f.required)
            .map(|f| f.name)
            .collect::<Vec<&str>>();

        let properties_map: Map<String, Value> = self.fields.iter().map(|f| {
            (f.name.to_owned(), json!({
                "type": f.type_,
                "description": f.description,
            }))
        }).collect();

        let json_data = json!({
            "name": self.name,
            "description": self.description,
            "inputSchema": {
                "type": "object",
                "properties": properties_map,
                "required": required_values,
            },
        });
        return json_data;
    }
}

pub trait Tool {
    async fn register_tool(self, module: &mut RpcModule<Arc<Context>>);
    async fn meta(&self) -> ToolMeta;
}
