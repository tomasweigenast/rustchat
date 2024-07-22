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

// async fn handle_client(mut stream: TcpStream, addr: SocketAddr) -> Result<(), Error> {
//     stream.set_nodelay(true)?;

//     println!("Connection established with host: {}", addr);

//     let mut buffer = [0; 50];
//     loop {
//         match stream.read(&mut buffer).await {
//             Ok(n) => {
//                 let data = String::from_utf8_lossy(&buffer[0..n]).trim().to_string();

//                 println!("Bytes from socket: {:?}. Read: {}", buffer, n);
//                 println!("Buffer as string: {}", data);

//                 streamk
//                     .write_all(format!("received: {}", data).as_bytes())
//                     .await
//                     .expect("write_all call failed");
//             }
//             Err(e) => {
//                 eprintln!("Error reading from socket: {}", e);
//                 stream.shutdown().await.expect("shutdown call failed");
//             }
//         }
//     }
// }
