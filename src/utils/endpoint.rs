use crate::tools::context::Context;
use crate::tools::tool::ToolMeta;
use crate::utils::schema;
use jsonrpsee::server::RpcModule;
use jsonrpsee::types::error::ErrorObjectOwned;
use serde_json::{self, Map, Value, json};
use std::sync::Arc;

pub async fn initialize(module: &mut RpcModule<Arc<Context>>) {
    module
        .register_async_method::<Result<Value, ErrorObjectOwned>, _, _>(
            "initialize",
            |_params, _ctx, _| async move { Ok(schema::initialize()) },
        )
        .unwrap();
}

pub async fn tools_list(module: &mut RpcModule<Arc<Context>>) {
    module
        .register_async_method::<Result<Value, ErrorObjectOwned>, _, _>(
            "tools/list",
            |_params, ctx, _| async move {
                tracing::info!("tools/list endpoint called");
                let tool_metas: Vec<ToolMeta> = ctx
                    .tools
                    .iter()
                    .filter_map(|(_k, v)| Some(v.meta()))
                    .collect::<Vec<_>>();
                tracing::info!("Found {} tools", tool_metas.len());
                Ok(schema::tools_list(tool_metas))
            },
        )
        .unwrap();
}

pub async fn tools_call(module: &mut RpcModule<Arc<Context>>) {
    module
        .register_async_method::<Result<Value, ErrorObjectOwned>, _, _>(
            "tools/call",
            |params, ctx, _| async move {
                if !params.is_object() {
                    Ok(json!({"error": "Invalid params"}))
                } else {
                    let p = params.parse::<Map<String, Value>>().unwrap();
                    let name = p.get("name").unwrap().as_str().unwrap();
                    tracing::info!("Calling tool {}", name);
                    let tool = ctx.tools.get(name).unwrap();
                    tool.execute(params)
                }
            },
        )
        .unwrap();
}
