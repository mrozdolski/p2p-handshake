mod config;
mod network;
mod handshake;
mod utils;

use tokio::signal;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (listener, remote_port) = match network::ports::bind_ports().await {
        Ok((listener, port)) => (listener, port),
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    tokio::select! {
        result = network::listener::start_listener(listener) => {
            if let Err(e) = result {
                eprintln!("{}", e);
            }
        },
        _ = network::connector::initiate_connection(remote_port) => {
            println!("Node handshake initiated and completed 🤝");
        },
        _ = signal::ctrl_c() => {
            eprintln!("Ctrl+C exiting");
        }
    }

    Ok(())
}
