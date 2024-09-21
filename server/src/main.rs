use common::{ADDR, Procedures, Message, Stub};
use common::{Sid, Pid};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde_json;

fn handle_client(server_data: &mut ServerData, mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    
    // Read data from the client
    let bytes_read = stream.read(&mut buffer)?;
    
    // Convert bytes to a string
    let received = serde_json::from_slice::<Stub>(&buffer[..bytes_read]).unwrap();
    println!("Received: {:?}", received);

    let response = server_data.register_sid();

    // Prepare and send a response back to the client
    let response = serde_json::to_string(&response).unwrap();
    stream.write_all(response.as_bytes())?;
    
    Ok(())
}

struct ServerData {
    sid_generator: Sid,
    sub_list: Vec<Sid>,
}

impl ServerData {
    pub fn new() -> Self {
        Self {
            sid_generator: 0,
            sub_list: Vec::new(),
        }
    }

    pub fn check_sid(&self, sid: Sid) -> bool {
        self.sub_list.contains(&sid)
    }

    pub fn register_sid(&mut self) -> Sid {
        self.sid_generator += 1;
        self.sub_list.push(self.sid_generator);
        self.sid_generator.clone()
    }



}

fn main() -> io::Result<()> {
    // Bind the server to a local address and port
    let listener = TcpListener::bind(ADDR)?;
    println!("Server listening on  {}", ADDR);

    let mut server_data = ServerData::new();

    // Accept connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                handle_client(&mut server_data, stream)?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
