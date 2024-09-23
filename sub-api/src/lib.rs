
use std::io::{BufReader, Write};
use std::net::TcpStream;
use common::{Sid, Messages, Stub, Procedures, try_connect};


pub fn register_subscriber() -> Result<Sid, String> {
    let mut stream: TcpStream = try_connect().unwrap();

    let rpc = Stub {
        id: 0,
        procedure: Procedures::RegisterSubscriber,
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

    let rpc = Stub {
        id: sid,
        procedure: Procedures::Subscribe,
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

    let rpc = Stub {
        id: sid,
        procedure: Procedures::Pull,
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