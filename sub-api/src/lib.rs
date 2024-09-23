// SUBSCRIBER APIs //

use common::{Messages, Procedures, Sid, Stub, try_connect};
use std::io::{BufReader, Write};
use serde_json;
// use std::net::TcpStream;


pub fn register_subscriber() -> Option<Sid> {
    // Try connecting to Server //
    /* if connection fails at any point, return None as SID */
    let Ok(mut stream) = try_connect()
        else {return None};

    // The Request //
    let rpc = Stub {
        id: 0,
        procedure: Procedures::RegisterSubscriber,
        args: vec![],
    };

    // Prepare the message to send //
    /* note: this "should" always succeed based on how Serde works */
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let Ok(_) = stream.write_all(message.as_bytes()) 
        else { return None };

    // Read Response from Server //
    let reader = BufReader::new(&stream);
    let Ok(new_sid) = serde_json::from_reader(reader) 
        else { return None; };

    // Return new SID //
    Some(new_sid)
}

pub fn subscribe(sid: Sid, topic: &str) {
    // Try connecting to Server //
    /* if connection fails at any point, do nothing */
    let Ok(mut stream) = try_connect()
        else {return ()};

    // The Request //
    let rpc = Stub {
        id: sid,
        procedure: Procedures::Subscribe,
        args: vec![topic.to_string()]
    };

    // Send request to Server //
    let message = serde_json::to_string(&rpc).unwrap();
    let _ = stream.write_all(message.as_bytes());

    // We don't expect a response //
}

pub fn pull(sid: Sid, topic: &str) -> Messages {
    // Try connecting to Server //
    /* if connection fails at any point, return an empty list */
    let Ok(mut stream) = try_connect()
        else {return vec![]};

    // The Request //
    let rpc = Stub {
        id: sid,
        procedure: Procedures::Pull,
        args: vec![topic.to_string()],
    };

    // Send request to Server //
    let message = serde_json::to_string(&rpc).unwrap();
    let Ok(_) = stream.write_all(message.as_bytes()) 
        else { return vec![]; };

    // Read response from Server //
    let reader = BufReader::new(&stream);
    let Ok(messages) = serde_json::from_reader(reader)
        else { return vec![]; };

    messages
}