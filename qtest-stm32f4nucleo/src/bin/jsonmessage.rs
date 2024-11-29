use qtest_stm32f4nucleo::Peripheral;
use serde_json::{Value, json};
use std::collections::HashMap;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio::net::TcpListener;
use futures_util::{SinkExt, StreamExt}; // Para trabajar con WebSocket
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex; // Usamos Mutex para manejar la concurrencia

#[derive(Serialize, Deserialize, Debug)]
struct FieldsMessage {
    field_type: String,
    fields: HashMap<String, Value>, // Usamos un HashMap para almacenar los campos dinámicos
}

// Función que maneja los "fields" recibidos desde el cliente
async fn handle_received_fields(
    message: String,
    global_fields: Arc<Mutex<HashMap<String, Value>>>,
) -> Result<FieldsMessage, serde_json::Error> {
    let fields_message: FieldsMessage = serde_json::from_str(&message)?;
    // Imprimir los campos recibidos (para depuración)
    println!("Recibido campos: {:?}", fields_message.fields);

    // Almacenamos los campos de forma global
    let mut global_fields_lock = global_fields.lock().await; // Bloqueamos el Mutex para acceder a los datos
    global_fields_lock.extend(fields_message.fields.clone()); // Extendemos los campos a la variable global

    Ok(fields_message)
}

// Función que envía los campos al cliente cuando los recibe una interrupción
async fn update_fields(mut json_data: Value, irq: &Irq, peripheral: Peripheral) -> Result<Value, InvalidGpioName> {
        // Iterar sobre cada peripheral en el array del JSON
        if let Some(peripherals) = json_data.as_array_mut() {
            for peripheral_obj in peripherals {
                if let Some(peripheral_name) = peripheral_obj.get("peripheral").and_then(|p| p.as_str()) {
                    // Convertir el nombre del peripheral a la línea GPIO
                    let gpio_line = gpio_to_number(peripheral_name)?;
    
                    // Comprobar si la IRQ coincide
                    if irq.line.to_string() == gpio_line {
                        if let Some(pins) = peripheral_obj.get_mut("pins").and_then(|pins| pins.as_array_mut()) {
                            for pin in pins {
                                if let (Some(pin_id), Some(valor)) = (pin.get("pin"), pin.get_mut("valor")) {
                                    if let (Some(pin_id_str), Some(valor_bool)) = (pin_id.as_str(), valor.as_bool_mut()) {
                                        // Realizamos la comprobación dinámica
                                        let new_value = peripheral.get(peripheral_name).idr.ishigh(pin_id_str);
                                        *valor_bool = new_value; // Actualizamos el valor
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(json_data.clone())
}



// Función que maneja la conexión WebSocket con el cliente
async fn handle_connection(
    mut ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    mut ws_rx: watch::Receiver<String>,
    global_fields: Arc<Mutex<HashMap<String, Value>>>,
) {
    // Escuchar mensajes del canal `watch` y reenviar al cliente
    while ws_rx.changed().await.is_ok() {
        let message = ws_rx.borrow().clone();
        let mensaje = format!("Mensaje : {}", message);
        if ws_stream.send(Message::Text(mensaje)).await.is_err() {
            eprintln!("[WebSocket] Error al enviar mensaje. Cerrando conexión.");
            break;
        }
    }

    // Escuchar los mensajes recibidos desde el cliente
    while let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(text) => {
                println!("Mensaje recibido de cliente: {}", text);
                let fields_message = handle_received_fields(text, global_fields.clone()).await.unwrap();
                // Aquí, ya hemos actualizado la variable global `global_fields` con los campos recibidos
            },
            Message::Close(_) => {
                println!("Cliente desconectado");
                break;
            },
            _ => {}
        }
    }
}


tokio::spawn(async move {
    loop {
        let irq = rx_irq.recv().await.unwrap();
        info!("[Parser] Received IRQ: {:?}", irq);
        handle_irq_update(fieldsglobal,irq, mut ws_tx);
    }
});

async fn handle_irq_update(json_str:value, irq: Irq, ws_tx: watch::Sender<String>) {
    ws_tx.send (update_fields(value, &irq, peripheral))
}


// Función que resuelve el GPIO basado en su nombre
fn gpio_to_number(nombre_gpio: &str) -> Result<&'static str, InvalidGpioName> {
    match nombre_gpio {
        "gpio_a" => Ok("0"),
        "gpio_b" => Ok("1"),
        "gpio_c" => Ok("2"),
        _ => Err(InvalidGpioName),
    }
}



let json_str = r#"
[
    {
        "peripheral": "gpio_a",
        "pins": [
            {"pin": "5", "valor": false},
            {"pin": "3", "valor": false},
            {"pin": "4", "valor": false}
        ]
    },
    {
        "peripheral": "gpio_b",
        "pins": [
            {"pin": "5", "valor": false},
            {"pin": "3", "valor": false},
            {"pin": "4", "valor": false}
        ]
    }
]"#;

// Parsear el JSON
let mut json_data: Value = serde_json::from_str(json_str).expect("JSON malformado");
