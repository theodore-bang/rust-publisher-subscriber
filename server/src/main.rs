use common::{ADDR, Procedures, Stub};
use common::{Sid, Pid};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use serde_json;

mod broker;
use crate::broker::Broker;

// Server Entry Point //
/*
// The server listens in on 
*/
fn main() -> io::Result<()> {
    // Bind the server to a local address and port //
    let listener = TcpListener::bind(ADDR)?;
    println!("Server listening at {}", ADDR);

    // Wrap Broker object in RwMutex and Arc Pointer //
    /* 
    // Arc Pointer allows the Broker to be accessed across multiple threads safely,
    // while the RwMutex ensures that only one thread can mutate the Broker at a
    // given time (solving data-races) but multiple threads can atomically read data.
    */
    let server_data = Arc::new(RwLock::new(Broker::new()));

    // Accept connections in a loop //
    for stream in listener.incoming() {
        // 
        let ref_to_server = Arc::clone(&server_data);
        // Spawn Thread for client if connection is successful //
        match stream {
            Ok(stream) => {
                // println!("New client connection!");
                thread::spawn(move || handle_client(ref_to_server, stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    /*
    // The above should loop endlessly, but if the TCP port closes
    // for some reason, the following message will play before
    // shutting down the server.
    */
    println!("Server unexpectedly stopped listening!");
    Ok(())
}

fn handle_client(server_data: Arc<RwLock<Broker>>, mut stream: TcpStream) -> io::Result<()> {
    // Buffer to hold bytes from request //
    // Todo: change this to a BufReader...
    // ...though I had problems with that before.
    let mut buffer = [0; 512];
    
    // Read data from the client as bytes //
    let bytes_read = stream.read(&mut buffer)?;
    
    // Deserialize the bytes back into a Stub //
    let received = serde_json::from_slice::<Stub>(&buffer[..bytes_read]).unwrap();
    println!("Received: {:?}", received);

    // Handle client's request //
    /*
    // Depending on procedure listed in the stub, call the appropriate 
    // Broker method.
     */
    match received.procedure {
        Procedures::RegisterSubscriber => {
            let response = server_data.write().unwrap().register_sub();

            // Prepare and send a response back to the client //
            let response = serde_json::to_string(&response).unwrap();
            stream.write_all(response.as_bytes())?;
        },
        Procedures::RegisterPublisher => {
            let response = server_data.write().unwrap().register_pub();

            // Prepare and send a response back to the client //
            let response = serde_json::to_string(&response).unwrap();
            stream.write_all(response.as_bytes())?;
        },
        Procedures::CreateTopic => {
            let pid = received.id;
            let topic_name = received.args[0].clone();
            server_data.write().unwrap().create_topic(pid, topic_name);
            // No response neede by client //
        },
        Procedures::DeleteTopic => {
            let pid = received.id;
            let topic_name = received.args[0].clone();
            server_data.write().unwrap().delete_topic(pid, topic_name);
        },
        Procedures::Send => {
            let pid = received.id;
            let topic_name = received.args[0].clone();
            let message_content = received.args[1].clone();
            server_data.write().unwrap().add_message(topic_name, message_content);
            // No response neede by client //
        },
        Procedures::Subscribe => {
            let sid = received.id;
            let topic_name = received.args[0].clone();
            server_data.write().unwrap().subscribe(sid, topic_name);
        },
        Procedures::Pull => {
            let sid = received.id;
            let topic_name = received.args[0].clone();
            let all_msgs = server_data.write().unwrap().pull(sid, topic_name);
            let response = serde_json::to_string(&all_msgs).unwrap();
            stream.write_all(response.as_bytes())?;
        }
        _ => (),
    }
    
    Ok(())
}