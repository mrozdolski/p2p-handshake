use tokio::net::TcpListener;

pub async fn start_listener(listener: TcpListener) -> std::io::Result<()> {
    println!("Listening for incoming connections...");

    loop {
        let (stream, _addr) = listener.accept().await?;
        if let Err(e) = crate::handshake::process::process_incoming_connection(stream).await {
            eprintln!("Handshake failed, try again. {}", e);
        } else {
            println!("Node handshake successful 🤝");
            break;
        }
    }

    Ok(())
}
