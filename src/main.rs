mod tools;
mod utils;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use jsonrpsee::server::{RpcModule, Server, ServerHandle};

use tracing_subscriber::util::SubscriberInitExt;
use tools::tool::{Tool, ToolMeta};
use tools::context::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .finish()
        .try_init()?;

    tracing::info!("Starting server");

    let (server_addr, handle) = run_server().await?;

    tracing::info!("Server running on {}", server_addr);

    handle.stopped().await;

    Ok(())
}

async fn run_server() -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let server = Server::builder()
        .build("127.0.0.1:1338".parse::<SocketAddr>()?)
        .await?;

    let context = Arc::new(Context { tools_meta: Mutex::new(vec![]) });
    let mut module = RpcModule::new(context.clone());

    register_tools(&mut module, &context).await;
    register_endpoints(&mut module).await;

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

async fn register_endpoints(module: &mut RpcModule<Arc<Context>>) {
    utils::endpoint::initialize(module).await;
    utils::endpoint::tools_list(module).await;
}

async fn register_tools(module: &mut RpcModule<Arc<Context>>, context: &Arc<Context>) {
    let tools = vec![
        tools::add::AddTool {},
    ];

    let mut tool_metas = vec![];

    for tool in &tools {
        let meta: ToolMeta = tool.meta().await;
        tool_metas.push(meta.clone());
    }

    for tool in tools {
        tool.register_tool(module).await;
    }

    *context.tools_meta.lock().unwrap() = tool_metas;
}


