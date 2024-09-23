
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};
use common::{Sid, Messages, Stub, Procedures};
use common::{try_connect};
use tokio::stream;


pub fn register_subscriber() -> Result<Sid, String> {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: 0,
        procedure: common::Procedures::RegisterSubscriber,
        args: vec![],
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let Ok(_) = stream.write_all(message.as_bytes()) 
    else { return Err("Failed to connect".to_string()); };

    let reader = BufReader::new(&stream);
    let new_sid: Sid = serde_json::from_reader(reader).unwrap();

    Ok(new_sid)
}

pub fn subscribe(sid: Sid, topic: String) {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: sid,
        procedure: common::Procedures::Subscribe,
        args: vec![topic.clone()]
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
}

pub fn pull(sid: Sid, topic: String) -> Messages {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: sid,
        procedure: common::Procedures::Pull,
        args: vec![topic],
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let Ok(_) = stream.write_all(message.as_bytes()) 
    else { return vec![]; };

    let reader = BufReader::new(&stream);
    let messages: Vec<String> = serde_json::from_reader(reader).unwrap();

    messages
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
