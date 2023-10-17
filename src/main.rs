use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

mod coding;
mod networking;
mod server;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => {
                eprintln!("Error accepting client connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    stream.set_nodelay(true).expect("set_nodelay call failed");

    let addr = stream.peer_addr().unwrap();
    println!("Connection established with host: {}", addr);

    let mut buffer = [0; 50];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                let data = String::from_utf8_lossy(&buffer[0..n]).trim().to_string();

                println!("Bytes from socket: {:?}. Read: {}", buffer, n);
                println!("Buffer as string: {}", data);

                stream
                    .write_all(format!("received: {}", data).as_bytes())
                    .expect("write_all call failed");
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                stream
                    .shutdown(Shutdown::Both)
                    .expect("shutdown call failed");
                break;
            }
        }
    }
}
