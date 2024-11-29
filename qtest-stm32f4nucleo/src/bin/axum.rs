use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MessageBody {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar el servidor WebSocket
    let websocket_addr = "127.0.0.1:8081"; // Puerto para WebSocket
    println!("Servidor WebSocket escuchando en {}", websocket_addr);

    // Manejar las conexiones WebSocket en un hilo separado
    tokio::spawn(async move {
        let listener = TcpListener::bind(websocket_addr).await.expect("Error al enlazar el listener");
        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error al aceptar conexión WebSocket");
            println!("Nuevo cliente conectado");

            // Gestionar la conexión WebSocket
            tokio::spawn(handle_connection(ws_stream));
        }
    });

    // Configurar las rutas de la API REST
    let echo_route = warp::path("api")
        .and(warp::path("echo"))
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: MessageBody| {
            warp::reply::json(&serde_json::json!({
                "echoed": format!("Mensaje recibido: {}", body.message),
            }))
        });

    // Configuración CORS
    let cors = warp::cors()
        .allow_any_origin() // Permitir cualquier origen
        .allow_methods(vec!["GET", "POST", "OPTIONS"]) // Permitir métodos específicos
        .allow_headers(vec!["Content-Type", "Authorization"]); // Permitir cabeceras

    // Aplicar CORS para permitir solicitudes de cualquier origen
    let routes = echo_route
        .with(cors) // Permitir cualquier origen
        .with(warp::log("api")); // Asegurarte de que esté aplicado para todas las rutas


    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    // Dejar que el servidor REST corra sin terminar el proceso principal
    println!("Servidor REST escuchando en http://127.0.0.1:8080");
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

// Manejar la conexión WebSocket
async fn handle_connection(mut ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    println!("Mensaje recibido de cliente: {}", text);
                    let response = format!("Mensaje recibido jaju: {}", text);
                    ws_stream.send(Message::Text(response)).await.unwrap();
                }
            },
            Err(e) => {
                eprintln!("Error en la conexión WebSocket: {:?}", e);
                break;
            }
        }
    }
    println!("Cliente desconectado");
}
