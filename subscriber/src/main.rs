use common;
use std::{io::{Read, Write}, net::TcpStream};

// use sub_api;

fn main() -> Result<(), anyhow::Error> {
    let mut stream = TcpStream::connect(common::ADDR)?;
    
    // Create a message
    let message = common::Message {
        content: "Hello from client".to_string(),
        id: 1,
    };

    // Serialize the message to JSON
    let message_json = serde_json::to_string(&message).unwrap();
    
    println!("Sending request...");
    // Send the message
    stream.write_all(message_json.as_bytes()).unwrap();

    // Wait for the server's response
    println!("Waiting for response...");
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Deserialize the response from the server
    let response: common::Message = serde_json::from_str(&String::from_utf8_lossy(&buffer)).unwrap();
    println!("Received from server: {:?}", response);

    Ok(())
}
