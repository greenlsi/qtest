import React from "react";
import "./SelectedPins.css"; // Archivo CSS para estilos

interface Field {
    peripheral: string;
    pin: string;
    data?: boolean;
    [key: string]: any; // Permite atributos adicionales
}


interface SelectedPinsProps {
    pins: Record<string, Field>;
}

const SelectedPins: React.FC<SelectedPinsProps> = ({ pins }) => {
    return (
        <div className="selected-pins-container">
            <h3 className="title">PINS SELECCIONADOS</h3>
            <div className="fields-grid-overflow-y-auto">
                {Object.keys(pins).map((fieldKey, fieldIndex) => {
                    const field = pins[fieldKey]; // Acceder al campo específico
                    return (
                        <div key={fieldIndex} className="field-card">
                            <div className="field-body">
                                <div className="pins-list">
                                    <div className="pin-item">
                                        <h5>{field.peripheral} - Pin {field.pin}</h5>
                                        <strong>Peripheral:</strong> {field.peripheral} <br />
                                        <strong>Pin:</strong> {field.pin} <br />
                                        <strong>Mode:</strong> {field.mode} <br />
                                        <strong>Estado:</strong> 
                                        <span className={`status ${field.data ? "high" : "low"}`}>
                                            {field.data ? "Alto" : "Bajo"}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default SelectedPins;
