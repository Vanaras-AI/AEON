import { useState, useEffect, useRef } from 'react';

export type TelemetryEvent = {
    type: 'METABOLISM' | 'SIGNAL' | 'TRAUMA' | 'HANDSHAKE' | 'HEARTBEAT' | 'PULSE' | 'PULSE_FAILED' | 'COALESCENCE_START' | 'GOVERNANCE_HALT';
    timestamp: number;
    payload: any;
};

export function useNervousSystem(url: string = 'ws://127.0.0.1:3000/telemetry') {
    const [connected, setConnected] = useState(false);
    const [events, setEvents] = useState<TelemetryEvent[]>([]);
    const ws = useRef<WebSocket | null>(null);

    useEffect(() => {
        ws.current = new WebSocket(url);

        ws.current.onopen = () => {
            console.log('🔌 [NERVOUS SYSTEM] Connected to Cortex');
            setConnected(true);
        };

        ws.current.onclose = () => {
            console.log('❌ [NERVOUS SYSTEM] Disconnected from Cortex');
            setConnected(false);
        };

        ws.current.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                setEvents((prev) => [...prev.slice(-99), data]); // Keep last 100 events
            } catch (e) {
                console.error('Failed to parse telemetry:', e);
            }
        };

        return () => {
            ws.current?.close();
        };
    }, [url]);

    const sendCommand = (type: string, payload: any = {}) => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            ws.current.send(JSON.stringify({ type, payload, timestamp: Date.now() }));
            console.log(`📤 [NERVOUS SYSTEM] Sent Command: ${type}`);
        } else {
            console.warn('⚠️ [NERVOUS SYSTEM] Cannot send command: Disconnected');
        }
    };

    return { connected, events, sendCommand };
}
