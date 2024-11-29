use futures_util::{SinkExt, StreamExt};
use tokio::io::{self, AsyncBufReadExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Servidor WebSocket corriendo en ws://127.0.0.1:8080");

    // Esperar a que el cliente se conecte
    let (stream, _) = listener.accept().await.unwrap();
    let ws_stream = accept_async(stream).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    // Crear tarea para leer entrada desde stdin
    let stdin_task = tokio::spawn(async move {
        let stdin = io::BufReader::new(io::stdin());
        let mut lines = stdin.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if write.send(line.into()).await.is_err() {
                println!("El cliente se desconectó. Finalizando...");
                break;
            }
        }
    });

    // Leer mensajes del cliente en el hilo principal
    while let Some(Ok(msg)) = read.next().await {
        println!("Mensaje recibido del cliente: {}", msg);
    }

    // Esperar a que la tarea de stdin termine antes de finalizar
    stdin_task.await.unwrap();
}
