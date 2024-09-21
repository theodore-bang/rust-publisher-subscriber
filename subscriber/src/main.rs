use common;
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};

// use sub_api;
use serde_json::Result;

fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    
    let rpc = common::Stub {
        id: None,
        procedure: common::Procedures::RegisterSubscriber,
        args: None,
    };

    // Prepare the message to send
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message
    stream.write_all(message.as_bytes())?;

    // Read the response from the server
    let reader = BufReader::new(&stream);
    let response: String = reader.lines().next().unwrap().unwrap();
    
    println!("Received from server: {}", response);

    Ok(())
}

