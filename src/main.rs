//! A simple WebSocket server.
//!
//! Can come in 2 flavors: an echo server or a broadcast server.

pub mod echo;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    echo::run().await
}
