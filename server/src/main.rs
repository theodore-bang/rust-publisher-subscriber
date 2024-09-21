use common::{ADDR, Procedures, Message, Stub};
use common::{Sid, Pid};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde_json;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    
    // Read data from the client
    let bytes_read = stream.read(&mut buffer)?;
    
    // Convert bytes to a string
    let received = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", received);

    // Prepare and send a response back to the client
    let response = "Message received";
    stream.write_all(response.as_bytes())?;
    
    Ok(())
}

fn main() -> io::Result<()> {
    // Bind the server to a local address and port
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    // Accept connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                handle_client(stream)?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
