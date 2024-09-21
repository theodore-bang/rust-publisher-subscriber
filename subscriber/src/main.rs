use common::Message;
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};

// use sub_api;
use serde_json::Result;

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    
    // Prepare the message to send
    let message = "Hello from client!";
    
    // Send the message
    stream.write_all(message.as_bytes())?;

    // Read the response from the server
    let reader = BufReader::new(&stream);
    let response: String = reader.lines().next().unwrap().unwrap();
    
    println!("Received from server: {}", response);

    Ok(())
}

