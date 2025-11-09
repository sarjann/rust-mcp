mod tools;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use jsonrpsee::server::{RpcModule, Server, ServerHandle};

use std::collections::HashMap;
use tools::context::Context;
use tools::tool::Tool;
use tracing_subscriber::util::SubscriberInitExt;

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

    let mut tools: HashMap<String, Box<dyn Tool>> = HashMap::new();
    // Register tools
    tools.insert("add".to_owned(), Box::new(tools::add::AddTool {}));

    let context = Arc::new(Context { tools: tools });

    let mut module = RpcModule::new(context.clone());

    register_endpoints(&mut module).await;

    let addr = server.local_addr()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

async fn register_endpoints(module: &mut RpcModule<Arc<Context>>) {
    utils::endpoint::initialize(module).await;
    utils::endpoint::tools_list(module).await;
    utils::endpoint::tools_call(module).await;
}
