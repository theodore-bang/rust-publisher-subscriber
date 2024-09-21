use sub_api;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sid = sub_api::register_subscriber().await.unwrap();
    println!("{sid}");
    Ok(())    
}
/*
fn echo() {
    let mut stream = TcpStream::connect(common::ADDR).await?;

    let (mut reader, mut writer) = stream.split();

    writer.write_all(b"Hello Mr. Server").await?;

    let mut buf = [0; 1024];

    let n = reader.read(&mut buf).await?;

    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
*/