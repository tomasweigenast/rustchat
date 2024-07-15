use server::server::Server;
use types::Result;

mod coding;
mod networking;
mod server;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new("127.0.0.1:7878").await?;
    server.run().await?;

    Ok(())
}
