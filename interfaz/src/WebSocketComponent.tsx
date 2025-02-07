import React, { useEffect } from "react";
import useWebSocket from "react-use-websocket";

interface WebSocketComponentProps {
    onMessage?: (message: any) => void;
    fieldsData: Record<string, any>;
}

const WebSocketComponent: React.FC<WebSocketComponentProps> = ({ onMessage, fieldsData }) => {
    const { sendMessage, readyState } = useWebSocket("ws://127.0.0.1:8081", {
        onOpen: () => {
            console.log("Conexión WebSocket abierta");
            sendFieldsMessage(); // Enviar mensaje inicial
        },
        onMessage: (event) => {
            console.log("Mensaje recibido:", event.data);
            try {
                const parsedMessage = JSON.parse(event.data);
                if (onMessage) {
                    onMessage(parsedMessage);
                }
            } catch (error) {
                console.error("Error al analizar el mensaje WebSocket:", error);
            }
        },
        onError: (event) => {
            console.error("Error en la conexión WebSocket:", event);
        },
        shouldReconnect: () => true,
    });

    // Enviar mensaje 
    const sendFieldsMessage = () => {
        const jsonString = JSON.stringify(fieldsData);
        console.log("Enviando fields.json al servidor:", jsonString);
        sendMessage(jsonString);
    };

    //Enviar mensaje cada vez que fieldsData cambie
    useEffect(() => {
        sendFieldsMessage();
    }, [fieldsData]);

    const connectionStatus: string = {
        0: "Conectando",
        1: "Abierto",
        2: "Cerrando",
        3: "Cerrado",
    }[readyState as 0 | 1 | 2 | 3] || "Desconocido";

    // Determinar la clase CSS para el spinner según el estado
    const spinnerClass: string = {
        0: "spinner connecting",
        1: "spinner open",
        2: "spinner closing",
        3: "spinner closed",
    }[readyState as 0 | 1 | 2 | 3] || "Desconocido";

    return (
        <div>
            <h4>WebSocket Cliente</h4>
            <div className={spinnerClass}></div>
            <p>Estado de la conexión: {connectionStatus}</p>
        </div>
    );
};

export default WebSocketComponent;