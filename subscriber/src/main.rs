use common;
use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream};
use sub_api;

// use sub_api;
use serde_json::Result;

// fn main() -> io::Result<()> {
fn main() {

    let my_sid = sub_api::register_subscriber().unwrap();

}

