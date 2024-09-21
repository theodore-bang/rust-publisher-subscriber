use common::{Sid, Messages, Stub, Procedures};
use tokio::net::{TcpStream, TcpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn register_subscriber() -> Option<Sid> {
    if let Ok(mut stream) = TcpStream::connect(common::ADDR).await {
        let (mut reader, mut writer) = stream.split();

        // Parameter Marshalling //
        let register_rpc = Stub {
            id: None,
            procedure: Procedures::RegisterSubscriber,
            args: None,
        };

        // Serialize to JSON and send to Server //
        let json_msg = serde_json::to_string(&register_rpc).unwrap();
        writer.write_all(json_msg.as_bytes()).await.unwrap();
        println!("Request sent...");

        // Read response from Server //
        println!("Reading response...");
        let mut buf = String::new();
        let _n = reader.read_to_string(&mut buf).await.unwrap();

        let new_sid = serde_json::from_str::<u64>(&buf).unwrap();

        println!("Received: {}", new_sid);
        Some(123)
    } else {
        None
    }
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
