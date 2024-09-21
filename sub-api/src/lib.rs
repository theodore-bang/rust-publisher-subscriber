
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};
use common::{Sid, Messages, Stub, Procedures};
use common::{try_connect};


pub fn register_subscriber() -> Option<Sid> {
    let mut stream = try_connect().unwrap();

    let rpc = common::Stub {
        id: None,
        procedure: common::Procedures::RegisterSubscriber,
        args: None,
    };

    // Prepare the message to send //
    let message = serde_json::to_string(&rpc).unwrap();
    
    // Send the message //
    let Ok(_) = stream.write_all(message.as_bytes()) else {return None};

    let reader = BufReader::new(&stream);
    let response: Sid = serde_json::from_reader(reader).unwrap();

    Some(response)
}

pub fn subscribe(sid: Sid, topic: String) {
    todo!()
}

pub fn pull(sid: Sid, topic: String) -> Messages {
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
