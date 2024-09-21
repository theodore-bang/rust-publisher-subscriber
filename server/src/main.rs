use common::{ADDR, Procedures, Message, Stub};
use common::{Sid, Pid};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use serde_json;

fn handle_client(server_data: Arc<RwLock<ServerData>>, mut stream: TcpStream) -> io::Result<()> {
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

struct ServerData {
    sid_generator: Sid,
    pid_generator: Pid,
    sub_list: Vec<Sid>,
    pub_list: Vec<Sid>,
}

impl ServerData {
    pub fn new() -> Self {
        Self {
            sid_generator: 0,
            pid_generator: 0,
            sub_list: Vec::new(),
            pub_list: Vec::new(),
        }
    }

    pub fn check_sid(&self, sid: Sid) -> bool {
        self.sub_list.contains(&sid)
    }
    pub fn check_pid(&self, pid: Pid) -> bool {
        self.sub_list.contains(&pid)
    }

    pub fn register_sub(&mut self) -> Sid {
        self.sid_generator += 1;
        self.sub_list.push(self.sid_generator);
        println!("Registering SID: {}", self.sid_generator);
        self.sid_generator.clone()
    }
    pub fn register_pub(&mut self) -> Pid {
        self.pid_generator += 1;
        self.pub_list.push(self.pid_generator);
        println!("Registering PID: {}", self.pid_generator);
        self.pid_generator.clone()
    }



}

fn main() -> io::Result<()> {
    // Bind the server to a local address and port
    let listener = TcpListener::bind(ADDR)?;
    println!("Server listening on  {}", ADDR);

    let mut server_data = Arc::new(RwLock::new(ServerData::new()));

    // Accept connections in a loop
    for stream in listener.incoming() {
        let ref_to_server = Arc::clone(&server_data);
        match stream {
            Ok(stream) => {
                println!("New client connected!");
                handle_client(ref_to_server, stream)?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
