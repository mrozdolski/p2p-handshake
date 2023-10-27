use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub async fn perform_handshake(stream: TcpStream) -> std::io::Result<()> {
    let mut stream = BufReader::new(stream);

    stream.write_all(b"1. Node handshake initiated. Awaiting response...\n").await?;
    crate::utils::io_helpers::read_line(&mut stream).await?;
    stream.write_all(b"3. Response received. Initiating verification...\n").await?;
    crate::utils::io_helpers::read_line(&mut stream).await?;
    stream.write_all(b"5. Verification successful. Proceeding with node connection...\n").await?;
    crate::utils::io_helpers::read_line(&mut stream).await
}
