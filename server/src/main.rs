use common::{ADDR, Procedures, Message, Stub};
use common::{Sid, Pid};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde_json;

fn main() -> Result<(), anyhow::Error> {

    println!("Starting server...");

    let mut _sid_generator: Sid = 0;
    let mut _pid_generator: Pid = 0;

    let mut _subscribers_list: Vec<Sid> = Vec::new();
    let mut _publishers_list: Vec<Pid> = Vec::new();

    let listener = TcpListener::bind(ADDR)
                                .expect("Could not start server!");

    println!("Server started!");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept a connection: {}", e);
            }

        }
    }

    println!("Shutting down server");

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    println!("Made connection");
    let buf_reader = BufReader::new(&mut stream);

    println!("Deserializing message");
    match serde_json::from_reader::<BufReader<&mut TcpStream>, Message>(buf_reader) {
        Ok(msg) => {
            println!("Received message: {:?}", msg);
            
            // Prepare response
            let response = Message { content: "Message received".to_string(), id: msg.id };
            let response_json = serde_json::to_string(&response).unwrap();
            
            // Send the response back to the client
            let buf_writer = BufWriter::new(&mut stream);
            let _ = serde_json::to_writer(buf_writer, &response_json);
        }
        Err(e) => {
            eprintln!("Failed to deserialize message: {}", e);
        }
    }

    /*
    // Read the incoming message
    println!("Reading incoming request...");
    match stream.read_to_string(&mut buffer) {
        Ok(_) => {
            // Convert buffer to string
            let received_data = buffer.clone();

            // Deserialize the message
            println!("Deserializing message");
            match serde_json::from_reader::<Message>(&received_data) {
                Ok(msg) => {
                    println!("Received message: {:?}", msg);
                    
                    // Prepare response
                    let response = Message { content: "Message received".to_string(), id: msg.id };
                    let response_json = serde_json::to_string(&response).unwrap();
                    
                    // Send the response back to the client
                    stream.write(response_json.as_bytes()).unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to deserialize message: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
        }
    }
    */
}