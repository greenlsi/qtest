use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{Error, ErrorKind, Result},
    net::{TcpListener, TcpStream},
    sync::watch::Receiver,
    task::JoinSet,
};
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};

pub async fn ws_handler(ws_url: String, ws_rx: Receiver<String>) -> Result<()> {
    tracing::debug!("Initializing TCP listener for WebSocket server...");
    let listener = TcpListener::bind(ws_url).await?;
    let local_addr = listener.local_addr()?;
    tracing::info!("WebSocket server listening on {local_addr}");

    let mut join_set = JoinSet::new();
    loop {
        tokio::select! {
            accept_res = listener.accept() => {
                let (stream, _) = accept_res?;
                let ws_stream = tokio_tungstenite::accept_async(stream).await.map_err(|_| Error::new(ErrorKind::ConnectionRefused, "Error accepting WebSocket connection"))?;
                let peer = ws_stream.get_ref().peer_addr()?;
                tracing::info!("WebSocket server established new connection: {peer}");

                join_set.spawn(handle_connection(ws_stream, ws_rx.clone()));
            }

            // When a spawned connection task finishes, this branch runs.
            Some(res) = join_set.join_next() => {
                match res {
                    Ok(()) => tracing::debug!("WebSocket connection task completed normally"),
                    Err(join_err) => tracing::error!("WebSocket connection task failed: {join_err:?}"),
                }
            }
        }
    }
}

/// Handler of a single WebSocket connection.
async fn handle_connection(ws_stream: WebSocketStream<TcpStream>, mut ws_rx: Receiver<String>) {
    let (mut write, mut read) = ws_stream.split();

    loop {
        tokio::select! {
            // Messages received from the WebSocket client
            Some(message) = read.next() => {
                match message {
                    Err(e) => {
                        tracing::error!("Error receiving WebSocket message: {e}");
                        break;
                    },
                    other => {
                        tracing::warn!("WebSocket received unexpected message: {other:?}")
                    }
                }
            }
            // Changes produced by QTest IRQs
            _ = ws_rx.changed() => {
                let peripheral_id = ws_rx.borrow().clone();
                let message = serde_json::json!({ "peripheral_irq": peripheral_id });
                let message = match serde_json::to_string(&message) {
                    Ok(json_str) => json_str,
                    Err(e) => {
                        tracing::error!("Error serializing message from mpsc: {e}");
                        continue;
                    }
                };
                if let Err(e) = write.send(Message::Text(message.into())).await {
                    tracing::error!("Error sending IRQ message through WebSocket: {e}");
                    break;
                }
            }
        }
    }
}
