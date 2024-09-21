use common::{self, Procedures, Stub};
use common::{Sid, Pid};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let port_listener = TcpListener::bind(common::ADDR).await?;

    let mut _sid_generator: Sid = 0;
    let mut _pid_generator: Pid = 0;

    let mut _subscribers_list: Vec<Sid> = Vec::new();
    let mut _publishers_list: Vec<Pid> = Vec::new();

    println!("Server started!");

    loop {
        let (mut socket, addr) = port_listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("Connecting with {:?}", addr);
            // let (mut reader, mut writer) = socket.split();
            // println!("Connected with {:?}", addr);

            let mut buffer = Vec::new();
            println!("Reading...");
            match socket.read_to_end(&mut buffer).await {
                Ok(_) => {
                    // Deserialize the data into a Message struct
                    if let Ok(received_msg) = serde_json::from_slice::<Stub>(&buffer) {
                        println!("Received message: {:?}", received_msg);

                        // Send a response back to the client
                        let response = serde_json::to_string(&1337).unwrap();
                        socket.write_all(response.as_bytes()).await.unwrap();
                    } else {
                        println!("Failed to deserialize message");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from socket; error = {:?}", e);
                }
            }

        }).await?;
    }

    // println!("Shutting down server");

    // Ok(())
}

/*
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket_stream, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket_stream.split();
            tokio::io::copy(&mut reader, &mut writer).await;
        }).await?;
    }

}
*/