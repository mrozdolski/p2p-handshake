use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

pub async fn read_line(stream: &mut BufReader<TcpStream>) -> std::io::Result<()> {
    let mut buf = String::new();
    stream.read_line(&mut buf).await?;
    print!("{}", buf);
    Ok(())
}
