use common::{ADDR, Procedures, Stub};
use common::{Sid, Pid};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use serde_json;

mod broker;
use crate::broker::Broker;

fn handle_client(server_data: Arc<RwLock<Broker>>, mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    
    // Read data from the client
    let bytes_read = stream.read(&mut buffer)?;
    
    // Convert bytes to a string
    let received = serde_json::from_slice::<Stub>(&buffer[..bytes_read]).unwrap();
    println!("Received: {:?}", received);

    match received.procedure {
        Procedures::RegisterSubscriber => {
            let response = server_data.write().unwrap().register_sub();

            // Prepare and send a response back to the client
            let response = serde_json::to_string(&response).unwrap();
            stream.write_all(response.as_bytes())?;
        },
        Procedures::RegisterPublisher => {
            let response = server_data.write().unwrap().register_pub();

            // Prepare and send a response back to the client
            let response = serde_json::to_string(&response).unwrap();
            stream.write_all(response.as_bytes())?;
        }
        _ => (),
    }
    
    Ok(())
}


fn main() -> io::Result<()> {
    // Bind the server to a local address and port
    let listener = TcpListener::bind(ADDR)?;
    println!("Server listening on  {}", ADDR);

    let server_data = Arc::new(RwLock::new(Broker::new()));

    // Accept connections in a loop
    for stream in listener.incoming() {
        let ref_to_server = Arc::clone(&server_data);
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                thread::spawn(move || handle_client(ref_to_server, stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
