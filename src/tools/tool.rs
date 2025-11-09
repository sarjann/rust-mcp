use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value, json};
use jsonrpsee::types::params::Params;
use jsonrpsee::types::error::ErrorObjectOwned;

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

        let properties_map: Map<String, Value> = self
            .fields
            .iter()
            .map(|f| {
                (
                    f.name.to_owned(),
                    json!({
                        "type": f.type_,
                        "description": f.description,
                    }),
                )
            })
            .collect();

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

pub trait Tool: Send + Sync {
    fn execute(&self, params: Params<'static>) -> Result<Value, ErrorObjectOwned>;
    fn meta(&self) -> ToolMeta;
}
