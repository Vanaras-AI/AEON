import { memo, useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

interface ForensicIntent {
    id: string;
    timestamp: string;
    type: 'NETWORK' | 'FILESYSTEM' | 'RESOURCE';
    intent: string;
    target: string;
    status: 'ALLOWED' | 'BLOCKED';
    reason?: string;
}

const IntentRow = ({ intent }: { intent: ForensicIntent }) => (
    <motion.div
        initial={{ opacity: 0, x: -10 }}
        animate={{ opacity: 1, x: 0 }}
        className="grid grid-cols-[80px_100px_1fr_100px] border-b border-[#222] text-[10px] font-mono group hover:bg-[#0a0a0a]"
    >
        <div className="p-3 text-gray-600 border-r border-[#222]">
            {intent.timestamp}
        </div>
        <div className="p-3 text-gray-400 border-r border-[#222] font-bold">
            {intent.type}
        </div>
        <div className="p-3 border-r border-[#222] text-white flex flex-col gap-1">
            <span className="text-gray-400 uppercase text-[9px]">{intent.intent}</span>
            <span className="truncate">{intent.target}</span>
            {intent.reason && <span className="text-[9px] text-red-500/80 italic">{intent.reason}</span>}
        </div>
        <div className={clsx(
            "p-3 flex items-center justify-center font-bold",
            intent.status === 'ALLOWED' ? "text-[#00ff41] bg-[#00ff41]/5" : "text-red-500 bg-red-900/10"
        )}>
            {intent.status}
        </div>
    </motion.div>
);

export default memo(({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
    const [intents, setIntents] = useState<ForensicIntent[]>([]);

    useEffect(() => {
        if (!isOpen) return;

        // Simulator for prototype
        const types: ForensicIntent['type'][] = ['NETWORK', 'FILESYSTEM', 'RESOURCE'];
        const interval = setInterval(() => {
            const newIntent: ForensicIntent = {
                id: Math.random().toString(),
                timestamp: new Date().toLocaleTimeString().split(' ')[0],
                type: types[Math.floor(Math.random() * types.length)],
                intent: 'ACTION_OBSERVED',
                target: 'api.openai.com',
                status: Math.random() > 0.3 ? 'ALLOWED' : 'BLOCKED',
                reason: Math.random() > 0.3 ? undefined : 'Prohibited by User Constitution'
            };

            if (newIntent.type === 'FILESYSTEM') {
                newIntent.intent = 'FILE_DELETE';
                newIntent.target = 'top.secret';
            } else if (newIntent.type === 'RESOURCE') {
                newIntent.intent = 'MEMORY_ALLOC';
                newIntent.target = '1.2GB Request';
            }

            setIntents(prev => [newIntent, ...prev].slice(0, 50));
        }, 1500);

        return () => clearInterval(interval);
    }, [isOpen]);

    if (!isOpen) return null;

    return (
        <div className="absolute inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-12">
            <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.95 }}
                className="w-full max-w-5xl bg-black border border-[#333] shadow-2xl flex flex-col h-[700px] overflow-hidden"
            >
                {/* Header */}
                <div className="h-12 border-b border-[#333] bg-[#050505] flex justify-between items-center px-4">
                    <div className="flex items-center gap-3">
                        <div className="w-6 h-6 bg-[#111] flex items-center justify-center border border-[#333]">
                            <span className="text-xs">⚖️</span>
                        </div>
                        <span className="text-sm font-bold text-white uppercase tracking-wider">
                            A2I Forensic Ledger <span className="text-gray-600">/</span> Real-Time Monitor
                        </span>
                    </div>
                    <button onClick={onClose} className="text-gray-500 hover:text-white transition-colors">
                        ✕ ESC
                    </button>
                </div>

                {/* Status Bar */}
                <div className="h-10 border-b border-[#333] flex items-center px-4 gap-6 bg-black text-[10px] uppercase font-mono tracking-widest">
                    <div className="flex items-center gap-2">
                        <span className="w-2 h-2 rounded-full bg-[#00ff41] animate-pulse"></span>
                        <span className="text-gray-400">Ledger Active</span>
                    </div>
                    <div className="flex items-center gap-2">
                        <span className="text-white">{intents.length} Events Captured</span>
                    </div>
                </div>

                {/* Table Header */}
                <div className="grid grid-cols-[80px_100px_1fr_100px] border-b border-[#222] bg-[#0a0a0a] text-[9px] uppercase font-mono text-gray-500 sticky top-0 z-10">
                    <div className="p-2 border-r border-[#222]">Time</div>
                    <div className="p-2 border-r border-[#222]">Category</div>
                    <div className="p-2 border-r border-[#222]">Intent / Target</div>
                    <div className="p-2 text-center">Verdict</div>
                </div>

                {/* Ledger Stream */}
                <div className="flex-1 overflow-y-auto bg-[#020202]">
                    <AnimatePresence initial={false}>
                        {intents.map((intent) => (
                            <IntentRow key={intent.id} intent={intent} />
                        ))}
                    </AnimatePresence>

                    {intents.length === 0 && (
                        <div className="h-full flex items-center justify-center text-gray-600 font-mono text-xs uppercase tracking-widest">
                            Waiting for Agent Intent...
                        </div>
                    )}
                </div>

                {/* Footer */}
                <div className="p-4 border-t border-[#333] bg-[#050505] flex justify-between items-center text-[10px] font-mono">
                    <div className="text-gray-500">
                        HASH: <span className="text-white">AEON_LEDGER_0x882c...</span>
                    </div>
                    <div className="flex gap-3">
                        <button className="text-gray-500 hover:text-white uppercase">Export CSV</button>
                        <button className="text-gray-500 hover:text-white uppercase">Verify Proofs</button>
                    </div>
                </div>
            </motion.div>
        </div>
    );
});
