import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

// --- MOCK DATA ---

interface LogEntry {
    time: string;
    source: string;
    message: string;
}

interface Escalation {
    id: string;
    title: string;
    severity: 'CRITICAL' | 'HIGH' | 'MEDIUM';
    source: string; // Agent DID
    status: 'OPEN' | 'VERIFYING' | 'RESOLVED';
    timestamp: string;
    description: string;
    logs: LogEntry[];
    resolution?: string;
}

const ESCALATIONS: Escalation[] = [
    {
        id: 'esc-01',
        title: 'Unauthorized Network Egress',
        severity: 'CRITICAL',
        source: 'did:aeon:dev:0x71a2...',
        status: 'OPEN',
        timestamp: '10:42 AM',
        description: 'Agent attempted to connect to unknown host (192.168.1.55) on port 22. Blocked by Policy Engine.',
        logs: [
            { time: '10:42:01', source: 'PolicyEngine', message: 'Intercepted socket.connect()' },
            { time: '10:42:02', source: 'Cortex-G', message: 'Violation confirmed: SEC-001 Zero Trust' },
            { time: '10:42:03', source: 'System', message: 'Agent Process Suspended' }
        ]
    },
    {
        id: 'esc-02',
        title: 'Identity Spoofing Attempt',
        severity: 'HIGH',
        source: 'did:aeon:rented:0x99b1...',
        status: 'VERIFYING',
        timestamp: '09:15 AM',
        description: 'Agent attempted to sign Ledger transaction using expired key.',
        logs: [
            { time: '09:15:22', source: 'Ledger', message: 'Signature validation failed' },
            { time: '09:15:23', source: 'AuthService', message: 'Key rotation triggered' }
        ]
    },
    {
        id: 'esc-03',
        title: 'Budget Cap Exceeded',
        severity: 'MEDIUM',
        source: 'did:aeon:soc:0x55c3...',
        status: 'RESOLVED',
        timestamp: 'Yesterday',
        description: 'Daily API spend reached $55.00 (Limit: $50.00).',
        logs: [
            { time: '23:55:00', source: 'Billing', message: 'Soft limit reached' },
            { time: '23:59:00', source: 'Billing', message: 'Hard limit hit. Throttling.' }
        ],
        resolution: 'Limit increased to $100.00 by Admin.'
    }
];

export default function Escalations() {
    const [selectedId, setSelectedId] = useState<string>(ESCALATIONS[0].id);
    const [incidents, setIncidents] = useState(ESCALATIONS);

    const selectedInc = incidents.find(i => i.id === selectedId) || incidents[0];

    const resolveIncident = (id: string) => {
        setIncidents(prev => prev.map(i => i.id === id ? { ...i, status: 'RESOLVED', resolution: 'Manual override applied by Admin.' } : i));
    };

    const verifyIncident = (id: string) => {
        setIncidents(prev => prev.map(i => i.id === id ? { ...i, status: 'VERIFYING' } : i));
    };

    return (
        <div className="flex-1 bg-black text-white font-sans h-full overflow-hidden flex relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* LEFT COLUMN: LIST */}
            <div className="w-80 border-r border-[#333] flex flex-col z-10 bg-black">
                <div className="p-6 border-b border-[#333] flex justify-between items-center bg-[#1a0505]">
                    <div>
                        <h2 className="text-xl font-bold uppercase tracking-tight text-red-500">Alerts</h2>
                        <p className="text-[10px] text-red-400 font-mono mt-1">{incidents.filter(i => i.status !== 'RESOLVED').length} ACTIVE INCIDENTS</p>
                    </div>
                    <div className="w-2 h-2 rounded-full bg-red-500 animate-pulse"></div>
                </div>
                <div className="flex-1 overflow-y-auto">
                    {incidents.map(inc => (
                        <div
                            key={inc.id}
                            onClick={() => setSelectedId(inc.id)}
                            className={clsx(
                                "p-4 border-b border-[#222] cursor-pointer hover:bg-[#111] transition-colors group relative",
                                selectedId === inc.id ? "bg-[#111]" : ""
                            )}
                        >
                            {selectedId === inc.id && <div className="absolute left-0 top-0 bottom-0 w-1 bg-red-500"></div>}

                            <div className="flex justify-between items-center mb-1">
                                <span className={clsx("font-bold text-xs",
                                    inc.severity === 'CRITICAL' ? "text-red-500" :
                                        inc.severity === 'HIGH' ? "text-orange-500" : "text-yellow-500"
                                )}>{inc.severity}</span>
                                <span className="text-[9px] font-mono text-gray-600">{inc.timestamp}</span>
                            </div>
                            <div className={clsx("font-bold text-sm mb-1", selectedId === inc.id ? "text-white" : "text-gray-400")}>{inc.title}</div>
                            <div className="flex justify-between items-center">
                                <span className="text-[10px] text-gray-600 font-mono truncate max-w-[120px]">{inc.source}</span>
                                <span className={clsx("text-[8px] px-1.5 py-0.5 rounded font-bold uppercase",
                                    inc.status === 'OPEN' ? "bg-red-900/50 text-red-500 border border-red-900" :
                                        inc.status === 'VERIFYING' ? "bg-yellow-900/50 text-yellow-500 border border-yellow-900" :
                                            "bg-green-900/50 text-green-500 border border-green-900"
                                )}>{inc.status}</span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* RIGHT COLUMN: DETAIL */}
            <div className="flex-1 p-8 overflow-y-auto z-10 relative bg-[#050000]">
                <AnimatePresence mode='wait'>
                    <motion.div
                        key={selectedId}
                        initial={{ opacity: 0, scale: 0.98 }}
                        animate={{ opacity: 1, scale: 1 }}
                        exit={{ opacity: 0, scale: 0.98 }}
                        className="max-w-4xl"
                    >
                        {/* Header */}
                        <div className="flex justify-between items-start mb-8 pb-6 border-b border-[#333]">
                            <div>
                                <h1 className="text-3xl font-bold mb-2 text-white">{selectedInc.title}</h1>
                                <div className="flex items-center gap-2">
                                    <span className="text-xs font-mono text-red-500 border border-red-900 bg-red-900/10 px-2 py-1 rounded">ID: {selectedInc.id.toUpperCase()}</span>
                                    <span className="text-xs font-mono text-gray-500">SOURCE: {selectedInc.source}</span>
                                </div>
                            </div>
                            <div className="flex gap-3">
                                {selectedInc.status === 'OPEN' && (
                                    <>
                                        <button
                                            onClick={() => verifyIncident(selectedInc.id)}
                                            className="bg-[#111] text-white border border-[#333] hover:border-white font-bold uppercase tracking-wider px-6 py-2 rounded transition-colors text-xs"
                                        >
                                            Investigate
                                        </button>
                                        <button
                                            onClick={() => resolveIncident(selectedInc.id)}
                                            className="bg-red-600 text-black font-bold uppercase tracking-wider px-6 py-2 rounded hover:bg-red-500 transition-colors text-xs"
                                        >
                                            Force Close
                                        </button>
                                    </>
                                )}
                                {selectedInc.status === 'VERIFYING' && (
                                    <button
                                        onClick={() => resolveIncident(selectedInc.id)}
                                        className="bg-[#00ff41] text-black font-bold uppercase tracking-wider px-6 py-2 rounded hover:bg-[#00cc33] transition-colors text-xs"
                                    >
                                        Mark Resolved
                                    </button>
                                )}
                                {selectedInc.status === 'RESOLVED' && (
                                    <div className="text-green-500 font-mono text-xs uppercase border border-green-900 bg-green-900/10 px-4 py-2 rounded flex items-center gap-2">
                                        <span>✔ Incident Closed</span>
                                    </div>
                                )}
                            </div>
                        </div>

                        {/* Description */}
                        <div className="mb-8">
                            <h3 className="text-xs font-bold uppercase tracking-widest text-gray-500 mb-2">Context</h3>
                            <p className="text-sm text-gray-300 leading-relaxed font-mono bg-[#111] p-4 rounded border border-[#222]">
                                {selectedInc.description}
                            </p>
                        </div>

                        {/* Logs Terminal */}
                        <div className="mb-8">
                            <h3 className="text-xs font-bold uppercase tracking-widest text-gray-500 mb-2">Black Box Logs</h3>
                            <div className="bg-black border border-[#333] rounded p-4 font-mono text-xs">
                                {selectedInc.logs.map((log, i) => (
                                    <div key={i} className="mb-1 flex gap-4">
                                        <span className="text-gray-600 select-none">{log.time}</span>
                                        <span className="text-blue-500 w-24">{log.source}</span>
                                        <span className="text-gray-300">
                                            {log.message}
                                        </span>
                                    </div>
                                ))}
                                <div className="mt-2 text-[#00ff41] animate-pulse">_</div>
                            </div>
                        </div>

                        {/* Resolution Note */}
                        {selectedInc.resolution && (
                            <div className="bg-green-900/10 border border-green-900/30 p-4 rounded">
                                <h3 className="text-xs font-bold uppercase tracking-widest text-green-500 mb-1">Resolution</h3>
                                <p className="text-sm text-green-100 font-mono">{selectedInc.resolution}</p>
                            </div>
                        )}

                    </motion.div>
                </AnimatePresence>
            </div>
        </div>
    );
}
