use crate::tools::context::Context;
use crate::utils::schema;
use jsonrpsee::server::RpcModule;
use jsonrpsee::types::error::ErrorObjectOwned;
use serde_json::Value;
use std::sync::Arc;

pub async fn initialize(module: &mut RpcModule<Arc<Context>>) {
    module
        .register_async_method::<Result<Value, ErrorObjectOwned>, _, _>(
            "initialize",
            |_params, _ctx, _| async move {
                Ok(schema::initialize())
            },
        )
        .unwrap();
}

pub async fn tools_list(module: &mut RpcModule<Arc<Context>>) {
    module
        .register_async_method::<Result<Value, ErrorObjectOwned>, _, _>(
            "tools/list",
            |_params, ctx, _| async move {
                let tools = ctx.tools_meta.lock().unwrap().clone();
                Ok(schema::tools_list(tools))
            },
        )
        .unwrap();
}
