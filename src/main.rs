use server::server::Server;

mod coding;
mod networking;
mod server;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new("127.0.0.1:7878").await?;
    let _ = server.run().await;

    Ok(())
}
