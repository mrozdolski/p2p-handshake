use tokio::io::{AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub async fn process_incoming_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut stream = BufReader::new(stream);

    crate::utils::io_helpers::read_line(&mut stream).await?;
    stream.write_all(b"2. Node handshake response sent. Awaiting verification...\n").await?;
    crate::utils::io_helpers::read_line(&mut stream).await?;
    stream.write_all(b"4. Verification complete. Node connection established.\n").await?;
    crate::utils::io_helpers::read_line(&mut stream).await?;
    stream.write_all(b"6. Node connection verified.\n").await
}
