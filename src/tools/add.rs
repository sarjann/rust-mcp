use crate::tools::context::Context;
use crate::tools::tool::{Tool, ToolField, ToolMeta};
use jsonrpsee::server::RpcModule;
use jsonrpsee::types::error::ErrorObjectOwned;
use std::sync::Arc;

pub struct AddTool {}

impl Tool for AddTool {
    async fn register_tool(self, module: &mut RpcModule<Arc<Context>>) {
        let metadata = self.meta().await;

        module
            .register_async_method::<Result<String, ErrorObjectOwned>, _, _>(
                metadata.name,
                |params, _, _| async move {
                    let params: Vec<i32> = params.parse()?;
                    let list_params = params.into_iter().collect::<Vec<i32>>();
                    let sum = list_params[0] + list_params[1];
                    Ok(sum.to_string())
                },
            )
            .unwrap();
    }
    async fn meta(&self) -> ToolMeta {
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
