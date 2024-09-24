// SERVER //
/*
// The server listens for TCP connections at address "common::ADDR", 
// defined in the "common" library crate. If you want to change this address,
// change it in the library crate `common` and then rebuild the Rust Workspace.
// When a new connection is made, the server spawns a new thread to handle
// the connection, which is done with function `handle_client()`.
// 
// Server data, i.e. the message bus or message broker, is held in the object
// `server_data`, which is an instance of `Broker` that holds the Topics and
// Messages of the server. There is only one instance of this `server_data`.
// Access to this `server_data` is shared between threads via an Arc Pointer,
// and guarded by a Read-Write Mutex (`RwLock`).
// 
// A spawned thread reads the client's request and then calls the appropriate
// method for `server_data`.
*/

use common::{ADDR, Procedures, Stub};
use common::{Sid, Pid};
use std::io::{self, Read, Write};
// use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use serde_json;

use std::time::{Duration, Instant};
use prometheus::{Encoder, TextEncoder, IntCounter, IntGauge, Registry};
use lazy_static::lazy_static;

mod broker;
use crate::broker::Broker;


// Metric: Counters for Prometheus //
lazy_static! {
    static ref REQUEST_COUNTER: IntCounter = IntCounter::new("tcp_requests_total", "Total TCP requests").unwrap();
    static ref REQUESTS_PER_SECOND: IntGauge = IntGauge::new("tcp_requests_per_second", "Requests per second").unwrap();
}

// Server Entry Point //
/*
// Server starts by setting up the metric collection system. These metrics will be sent
// to metric analyzer Prometheus, at address 127.0.0.1:9898 - if any part of this system 
// fails, the server should continue functioning, but metrics won't be collected.
// Server then begins by creating a Message Broker, and then listening at a hardcoded
// address ("common::ADDR").
*/
fn main() -> io::Result<()> {
    // Metrics: Create counters for Prometheus //
    let registry = Registry::new();
    registry.register(Box::new(REQUEST_COUNTER.clone())).unwrap();
    registry.register(Box::new(REQUESTS_PER_SECOND.clone())).unwrap();

    // Metrics: thread that counts "requests per second" //
    thread::spawn(move || {
        loop {
            let _start = Instant::now();
            let start_count = REQUEST_COUNTER.get();

            thread::sleep(Duration::from_secs(1));

            // Calculate requests per second (rps)
            let end_count = REQUEST_COUNTER.get();
            let rps = end_count - start_count;
            REQUESTS_PER_SECOND.set(rps as i64);
        }
    });

    // Metrics: 
    thread::spawn(move || {
        let addr = "127.0.0.1:9898";
        let listener = TcpListener::bind(addr).unwrap();
        // println!("Metrics server running at {}", addr);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            handle_metrics(stream, &registry);
        }
    });

    // Wrap server_data object in RwLock Mutex and Arc Pointer //
    let server_data = Arc::new(RwLock::new(Broker::new()));

    // Bind the server to a local address and port //
    let listener = TcpListener::bind(ADDR)?;
    println!("Server: listening at {}", ADDR);

    // Accept connections in a loop //
    for stream in listener.incoming() {
        // Clone a reference to server_data to give to new thread //
        let ref_to_server = Arc::clone(&server_data);

        // Spawn Thread for client if connection is successful //
        match stream {
            Ok(stream) => {
                // println!("New client connection!");
                thread::spawn(move || handle_client(ref_to_server, stream));
            }
            Err(e) => {
                eprintln!("Server: connection failed: {}", e);
            }
        }
    }

    /*
    // The above should loop endlessly, but if the TCP port closes
    // for some reason, the following message will play before
    // shutting down the server.
    */
    println!("Server: unexpectedly stopped listening");
    Ok(())
}

// Handle individual Client connections //
/*
// This function will be run by spawned threads for every client request.
// Todo: I haven't decided how to handle server errors yet. The problem is that
// if the client expects a response, the server might not be able to provide one.
// The client should probably implement some timeout error.
*/
fn handle_client(server_data: Arc<RwLock<Broker>>, mut stream: TcpStream) {
    // Buffer to hold bytes from request //
    // Todo: change this to a BufReader...
    // ...though I had problems with that before.
    let mut buffer: [u8; 512] = [0; 512];
    
    // Read data from the client as bytes //
    let Ok(bytes_read) = stream.read(&mut buffer)
        else { return () };
    
    // Deserialize the bytes back into a Stub //
    let Ok(received) = serde_json::from_slice::<Stub>(&buffer[..bytes_read])
    else { return () };
    println!("Server: received request \"{:?}\"", received);

    // Handle client's request //
    /*
    // Depending on procedure listed in the server stub, call the appropriate 
    // Broker method.
     */
    match received.procedure {
        Procedures::RegisterSubscriber => {
            let new_sid: Sid = server_data.write().unwrap().register_sub();

            // Send SID back to the client //
            let response = serde_json::to_string(&new_sid).unwrap();
            let _ = stream.write_all(response.as_bytes());
        },
        Procedures::RegisterPublisher => {
            let new_pid: Pid = server_data.write().unwrap().register_pub();

            // Send PID back to the client //
            let response = serde_json::to_string(&new_pid).unwrap();
            let _ = stream.write_all(response.as_bytes());
        },
        Procedures::CreateTopic => {
            let pid: Pid = received.id;
            let topic_name = received.args[0].clone();
            server_data.write().unwrap().create_topic(pid, topic_name);
            // No response neede by client //
        },
        Procedures::DeleteTopic => {
            let pid: Pid = received.id;
            let topic_name = received.args[0].clone();
            server_data.write().unwrap().delete_topic(pid, topic_name);
            // No response neede by client //
        },
        Procedures::Send => {
            let pid: Pid = received.id;
            let topic_name = received.args[0].clone();
            let message_content = received.args[1].clone();
            server_data.write().unwrap().add_message(pid, topic_name, message_content);
            // No response neede by client //
        },
        Procedures::Subscribe => {
            let sid: Sid = received.id;
            let topic_name = received.args[0].clone();
            server_data.write().unwrap().subscribe(sid, topic_name);
            // No response neede by client //
        },
        Procedures::Pull => {
            let sid: Sid = received.id;
            let topic_name = received.args[0].clone();
            let all_msgs = server_data.write().unwrap().pull(sid, topic_name);
            let response = serde_json::to_string(&all_msgs).unwrap();

            // Send messages back to Client //
            /* Todo: as mentioned, need to do better error handling. */
            let _ = stream.write_all(response.as_bytes());
        }
    }

    // Metric: Count a request as finished //
    REQUEST_COUNTER.inc();
    
    // Ok(())
}

fn handle_metrics(mut stream: TcpStream, registry: &Registry) {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    stream.write_all(&buffer).unwrap();
}