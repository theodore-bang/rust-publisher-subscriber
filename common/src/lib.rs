use std::net::TcpStream;
use serde::{Deserialize, Serialize};

pub const ADDR: &str = "127.0.0.1:5550";

pub type Pid = u64;
pub type Sid = u64;
pub type Messages = Vec<String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Stub {
    pub id: u64,
    pub procedure: Procedures,
    pub args: Vec<String>,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub id: u32,
}
*/

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

pub fn try_connect() -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(ADDR)
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
