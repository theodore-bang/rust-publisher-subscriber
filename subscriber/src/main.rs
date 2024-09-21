use common;
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};
use sub_api;

// use sub_api;
use serde_json::Result;

fn main() -> io::Result<()> {

    let response = sub_api::register_subscriber().unwrap();
    println!("Received from server: {}", response);

    Ok(())
}

