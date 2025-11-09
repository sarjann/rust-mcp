use crate::tools::tool::{Tool, ToolField, ToolMeta};
use jsonrpsee::types::error::ErrorObjectOwned;
use jsonrpsee::types::params::Params;
use serde_json::{Value, json};
use serde_json::map::Map;

pub struct AddTool {}

impl Tool for AddTool {
    fn execute(&self, params: Params<'static>) -> Result<Value, ErrorObjectOwned> {
        // let params: Vec<i32> = params.parse()?;
        let params = params.parse::<Map<String, Value>>().unwrap();
        let arguments = params.get("arguments").unwrap();
        let a = arguments.get("a").unwrap().as_i64().unwrap();
        let b = arguments.get("b").unwrap().as_i64().unwrap();
        let sum = a + b;
        Ok(json!({ "sum": sum }))
    }

    fn meta(&self) -> ToolMeta {
        ToolMeta {
            name: "add",
            description: "Add two numbers",
            fields: vec![
                ToolField {
                    name: "a",
                    description: "First number",
                    type_: "number",
                    required: true,
                },
                ToolField {
                    name: "b",
                    description: "Second number",
                    type_: "number",
                    required: true,
                },
            ],
        }
    }
}
