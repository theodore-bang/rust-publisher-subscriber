use std::io::{BufReader, Write};

use common::{try_connect, Pid};
use serde_json;

pub fn register_publisher() -> Option<Pid> {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: 0,
        procedure: common::Procedures::RegisterPublisher,
        args: vec![],
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let Ok(_) = stream.write_all(message.as_bytes()) else {return None};

    let reader = BufReader::new(&stream);
    let response: Pid = serde_json::from_reader(reader).unwrap();

    Some(response)
}

pub fn create_topic(pid: Pid, topic: &str) {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: pid,
        procedure: common::Procedures::CreateTopic,
        args: vec![topic.to_string()],
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
}

pub fn delete_topic(pid: Pid, topic: &str) {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: pid,
        procedure: common::Procedures::DeleteTopic,
        args: vec![topic.to_string()],
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let _ = stream.write_all(message.as_bytes());

    // We don't exepct a response //
}

pub fn send(pid: Pid, topic: &str, message: &str) {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: pid,
        procedure: common::Procedures::Send,
        args: vec![topic.to_string(), message.to_string()],
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
}
