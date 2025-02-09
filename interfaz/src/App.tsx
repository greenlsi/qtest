import { useEffect, useState } from 'react';
import WebSocketComponent from './WebSocketComponent.tsx';
import './App.css';
import Board from './board.tsx';
import importedMapData from './Map.json';
import initialFieldsData from './Fields.json';
import SelectedPins from './selectedPins.tsx';
//import 'bootstrap/dist/css/bootstrap.min.css';

interface Field {
    peripheral: string;
    pin: string;
    data?: boolean;
    [key: string]: any; // Permite atributos adicionales
}

interface FieldsData {
    fields: Record<string, Field>;
}

interface PinData {
    peripheral: string;
    pin: string;
}

interface MapData {
    [key: string]: PinData;
}


function App() {
    const [isPressed, setIsPressed] = useState<boolean>(false);
    const [ledState, setLedState] = useState<boolean>(false);
    const [fieldsData, setFieldsData] = useState<FieldsData>(initialFieldsData);
    const [resultado, setResultado] = useState<typeof fieldsData.fields>(initialFieldsData.fields);
    const [mapData] = useState<MapData>(importedMapData);

    const changeButtonState = () => {
        setIsPressed((prevValue) => !prevValue);
    };

    const modifyFields = (id: string) => {
        if (!mapData[id]) {
            console.error(`ID ${id} no encontrado en mapData.`);
            console.log("fieldsData:", fieldsData);
            console.log("mapData:", mapData);
            return fieldsData;
        }

        const newFieldsData = { ...fieldsData };

        if (!newFieldsData.fields[id]) {
            newFieldsData.fields[id] = { ...mapData[id] };
            console.log("fieldsData actualizado (agregado):", newFieldsData);
        } else {
            delete newFieldsData.fields[id];
            console.log("fieldsData actualizado (eliminado):", newFieldsData);
        }

        setFieldsData(newFieldsData);
    };

    useEffect(() => {
        const fetchData = async () => {
            try {
                const response = await fetch(`http://localhost:8080/gpio_c/pulsar_boton/13/${isPressed ? 0 : 1}`, {
                    method: 'GET',
                });
                if (!response.ok) throw new Error('Failed to set pin state');
                const data = await response.json();
                console.log('Data tras pulsar botón:', data);
            } catch (error) {
                console.error('Error:', error);
            }
        };
        fetchData();
    }, [isPressed]);

    const handleWebSocketMessage = (webSocketMessage: string | object) => {
        try {
            const data: { fields: Field } = typeof webSocketMessage === "string" ? JSON.parse(webSocketMessage) : webSocketMessage;
            setResultado(data.fields);

            if (data.fields["led2"]) {
                setLedState(data.fields["led2"].data || false);
            }
        } catch (error) {
            console.error("Error procesando el mensaje del WebSocket:", error);
        }
    };

    return (
        <div className="container">
            <div className="board-container">
                <Board ledState={ledState} changeButtonState={changeButtonState} modifyFields={modifyFields} />
            </div>
            <div className="resultado">
                <div className="websocket-container">
                    <div className="websocket-view">
                        <WebSocketComponent onMessage={handleWebSocketMessage} fieldsData={fieldsData} />
                    </div>
                </div>
                {resultado && <SelectedPins pins={resultado} />}
            </div>
        </div>
    );
}

export default App;

