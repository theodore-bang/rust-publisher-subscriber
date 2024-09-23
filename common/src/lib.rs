// COMMON DATA STRUCTURES //
/* 
// This library consists of data stuctures, types, and function 
// used by crates `pub_api`, `sub_api`, and `server`.
*/

use std::net::TcpStream;
use serde::{Deserialize, Serialize};

// Server's Address, hardcoded into programs //
// If you change this, rebuild the entire Rust Workspace, because
// this address is hardcoded into the client API.
pub const ADDR: &str = "127.0.0.1:5550";

// Type Aliases //
pub type Pid = u64;
pub type Sid = u64;
pub type Messages = Vec<String>;

// Function for Clients to connect with Server //
pub fn try_connect() -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(ADDR)
}

// Requests to Server //
#[derive(Debug, Deserialize, Serialize)]
pub struct Stub {
    pub id: u64,
    pub procedure: Procedures,
    pub args: Vec<String>,
}

// Possible RPCs to Server // 
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Procedures {
    RegisterSubscriber,
    Subscribe,
    Pull,
    RegisterPublisher,
    CreateTopic,
    DeleteTopic,
    Send
}

// Responses to Clients //
/* 
// Todo: didn't implement this as part of system yet.
// Server can just directly send back reponses as a 
// specific data type with Serde, for now.
*/
/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: u32,
    pub content: String,
}
*/
