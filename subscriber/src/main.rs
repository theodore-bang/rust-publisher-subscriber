use common;
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};
use sub_api;

// use sub_api;
use serde_json::Result;

fn main() -> io::Result<()> {

    if let Ok(response) = sub_api::Subscriber::register_subscriber() {
        println!("Received from server: {}", response.sid);
    }

    Ok(())
}

