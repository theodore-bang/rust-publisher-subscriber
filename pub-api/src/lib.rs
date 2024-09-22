use std::io::{BufReader, Write};

use common::{try_connect, Pid};
use serde_json;

pub fn register_publisher() -> Option<Pid> {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: None,
        procedure: common::Procedures::RegisterPublisher,
        args: None,
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let Ok(_) = stream.write_all(message.as_bytes()) else {return None};

    let reader = BufReader::new(&stream);
    let response: Pid = serde_json::from_reader(reader).unwrap();

    Some(response)
}

pub fn create_topic(pid: Pid, topic: String) {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: Some(pid),
        procedure: common::Procedures::CreateTopic,
        args: Some(vec![topic.clone()]),
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
    /*
    let reader = BufReader::new(&stream);
    let response: Pid = serde_json::from_reader(reader).unwrap();

    Some(response)
    */
}

pub fn delete_topic(pid: Pid, topic: String) {
    todo!()
}

pub fn send(pid: Pid, topic: String, message: String) {
    todo!()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
