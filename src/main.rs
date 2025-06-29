use anyhow::Result;
use cargo_mcp::server::CargoMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    CargoMcpServer::run().await
}
