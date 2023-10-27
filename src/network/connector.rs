use crate::config::constants::{LOCALHOST, WAIT_INTERVAL};
use tokio::net::TcpStream;
use tokio::time::sleep;

pub async fn initiate_connection(remote_port: u16) {
    loop {
        if let Ok(stream) = TcpStream::connect((LOCALHOST, remote_port)).await {
            if let Err(e) = crate::handshake::initiate::perform_handshake(stream).await {
                eprintln!("{}", e);
            }
            break;
        } else {
            println!("💤 Idle waiting for a peer");
            sleep(WAIT_INTERVAL).await;
        }
    }
}
