use tokio::net::TcpListener;

pub async fn bind_ports() -> std::io::Result<(TcpListener, u16)> {
    match TcpListener::bind(("127.0.0.1", 5000)).await {
        Ok(listener) => Ok((listener, 5001)),
        Err(_) => TcpListener::bind(("127.0.0.1", 5001))
            .await
            .map(|listener| (listener, 5000)),
    }
}
