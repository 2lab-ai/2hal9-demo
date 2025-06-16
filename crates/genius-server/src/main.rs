use genius_server::SimpleGameServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("genius_server=info,tower_http=info")
        .init();
    
    // Use simple server for now (full server has complex dependencies)
    let server = SimpleGameServer::new();
    server.run("0.0.0.0:8080").await?;
    
    Ok(())
}