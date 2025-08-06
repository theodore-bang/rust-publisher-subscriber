// PUBLISHER APIs //

use common::{Pid, Procedures, Stub, try_connect};
use std::io::{BufReader, Write};

pub fn register_publisher() -> Option<Pid> {
    // Try connecting to Server //
    /* if connection fails at any point, just return None as PID */
    let Ok(mut stream) = try_connect()
    else {return None};

    // The Request //
    let rpc = Stub {
        id: 0,
        procedure: Procedures::RegisterPublisher,
        args: vec![],
    };

    // Prepare the message to send //
    /* note: this "should" always succeed based on how Serde works */
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send request to Server //
    let Ok(_) = stream.write_all(message.as_bytes()) else {return None};

    // Read response from Server //
    let reader = BufReader::new(&stream);
    let Ok(new_pid) = serde_json::from_reader(reader)
        else { return None; };

    // Return new PID //
    Some(new_pid)
}

pub fn create_topic(pid: Pid, topic: &str) {
    // Try connecting to Server //
    /* if it fails, do nothing */
    let Ok(mut stream) = try_connect()
    else {return };

    // The Request //
    let rpc = Stub {
        id: pid,
        procedure: Procedures::CreateTopic,
        args: vec![topic.to_string()],
    };

    // Send request to Server //
    let message = serde_json::to_string(&rpc).unwrap();
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
}

pub fn delete_topic(pid: Pid, topic: &str) {
    // Try connecting to Server //
    /* if it fails, do nothing */
    let Ok(mut stream) = try_connect()
    else {return };

    // The Request //
    let rpc = Stub {
        id: pid,
        procedure: Procedures::DeleteTopic,
        args: vec![topic.to_string()],
    };

    // Send request to Server //
    let message = serde_json::to_string(&rpc).unwrap();
    let _ = stream.write_all(message.as_bytes());

    // We don't exepct a response //
}

pub fn send(pid: Pid, topic: &str, message: &str) {
    // Try connecting to Server //
    /* if it fails, do nothing */
    let Ok(mut stream) = try_connect()
    else {return };

    // The Request //
    let rpc = Stub {
        id: pid,
        procedure: Procedures::Send,
        args: vec![topic.to_string(), message.to_string()],
    };

    // Send request to Server //
    let message = serde_json::to_string(&rpc).unwrap();
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
}
