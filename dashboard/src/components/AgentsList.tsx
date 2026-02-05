import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

// --- MOCK DATA ---

interface Comment {
    id: string;
    author: string;
    role: string;
    text: string;
    date: string;
}

interface AgentProfile {
    id: string;
    name: string;
    did: string;
    created: string;
    calls: string;
    lazarusEvents: number;
    isSafe: boolean;
    tags: string[];
    comments: Comment[];
    description: string;
}

const AGENTS: AgentProfile[] = [
    {
        id: 'ag-01',
        name: 'DeepResearch v4',
        did: 'did:aeon:dev:0x71a2...',
        created: '2025-11-12',
        calls: '1.2M',
        lazarusEvents: 2,
        isSafe: true,
        tags: ['Research', 'WASM', 'High-Avail'],
        description: 'Recursive searching agent that summarizes 100+ sources. Optimized for financial due diligence.',
        comments: [
            { id: 'c1', author: 'Sarah C.', role: 'Product Manager', text: 'Excellent latency improvents in v4. Ready for bank deployment.', date: '2 days ago' },
            { id: 'c2', author: 'System', role: 'Automated', text: 'Lazarus Recovery triggered on 2026-01-15 due to OOM.', date: '2 weeks ago' }
        ]
    },
    {
        id: 'ag-02',
        name: 'Contract Auditor',
        did: 'did:aeon:legal:0x99b1...',
        created: '2025-12-05',
        calls: '450k',
        lazarusEvents: 0,
        isSafe: true,
        tags: ['Security', 'Legal', 'Audit'],
        description: 'Static analysis for Solidity and Rust smart contracts. Checks for reentrancy and overflow.',
        comments: [
            { id: 'c3', author: 'Mike R.', role: 'CISO', text: 'Approved for Level 4 sensitive contracts.', date: '1 month ago' }
        ]
    },
    {
        id: 'ag-03',
        name: 'Twitter Sentiment',
        did: 'did:aeon:soc:0x55c3...',
        created: '2026-01-20',
        calls: '8.9M',
        lazarusEvents: 14,
        isSafe: false,
        tags: ['Social', 'High-Vol', 'Beta'],
        description: 'Real-time brand monitoring. NOTE: High memory usage during viral events.',
        comments: [
            { id: 'c4', author: 'Sarah C.', role: 'Product Manager', text: 'Too unstable for enterprise clients. Keeps crashing.', date: 'Yesterday' }
        ]
    }
];

export default function AgentsList() {
    const [selectedId, setSelectedId] = useState<string>(AGENTS[0].id);
    const [isSimulating, setIsSimulating] = useState(false);
    const [simProgress, setSimProgress] = useState(0);

    const selectedAgent = AGENTS.find(a => a.id === selectedId) || AGENTS[0];

    const runMonteCarlo = () => {
        if (isSimulating) return;
        setIsSimulating(true);
        setSimProgress(0);

        // Simulation Loop
        let p = 0;
        const interval = setInterval(() => {
            p += Math.random() * 5;
            if (p >= 100) {
                p = 100;
                clearInterval(interval);
                setIsSimulating(false);
            }
            setSimProgress(p);
        }, 100);
    };

    return (
        <div className="flex-1 bg-black text-white font-sans h-full overflow-hidden flex relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* LEFT COLUMN: LIST */}
            <div className="w-80 border-r border-[#333] flex flex-col z-10 bg-black">
                <div className="p-6 border-b border-[#333]">
                    <h2 className="text-xl font-bold uppercase tracking-tight">Active Fleet</h2>
                    <p className="text-[10px] text-gray-500 font-mono mt-1">{AGENTS.length} AGENTS DEPLOYED</p>
                </div>
                <div className="flex-1 overflow-y-auto">
                    {AGENTS.map(agent => (
                        <div
                            key={agent.id}
                            onClick={() => setSelectedId(agent.id)}
                            className={clsx(
                                "p-4 border-b border-[#222] cursor-pointer hover:bg-[#111] transition-colors group",
                                selectedId === agent.id ? "bg-[#111] border-l-2 border-l-[#00ff41]" : "border-l-2 border-l-transparent"
                            )}
                        >
                            <div className="flex justify-between items-center mb-1">
                                <span className={clsx("font-bold text-sm", selectedId === agent.id ? "text-white" : "text-gray-400 group-hover:text-white")}>{agent.name}</span>
                                {agent.isSafe ? (
                                    <span className="text-[8px] bg-[#00ff41]/10 text-[#00ff41] px-1.5 py-0.5 rounded font-bold uppercase">Safe</span>
                                ) : (
                                    <span className="text-[8px] bg-red-500/10 text-red-500 px-1.5 py-0.5 rounded font-bold uppercase">Risk</span>
                                )}
                            </div>
                            <div className="text-[10px] text-gray-600 font-mono truncate">{agent.did}</div>
                        </div>
                    ))}
                </div>
            </div>

            {/* RIGHT COLUMN: DETAIL */}
            <div className="flex-1 p-8 overflow-y-auto z-10 relative">
                <AnimatePresence mode='wait'>
                    <motion.div
                        key={selectedId}
                        initial={{ opacity: 0, x: 20 }}
                        animate={{ opacity: 1, x: 0 }}
                        exit={{ opacity: 0, x: -20 }}
                        className="max-w-4xl"
                    >
                        {/* Header */}
                        <div className="flex justify-between items-start mb-8">
                            <div>
                                <h1 className="text-4xl font-bold mb-2">{selectedAgent.name}</h1>
                                <div className="flex gap-2">
                                    {selectedAgent.tags.map(t => (
                                        <span key={t} className="text-[10px] font-mono border border-[#333] px-2 py-0.5 rounded text-gray-400">{t}</span>
                                    ))}
                                </div>
                            </div>
                            <div className="text-right">
                                <div className="text-[10px] text-gray-500 font-mono uppercase mb-1">Created By</div>
                                <div className="text-xs font-mono text-[#00ff41] bg-[#00ff41]/5 px-2 py-1 rounded border border-[#00ff41]/20">
                                    {selectedAgent.did}
                                </div>
                            </div>
                        </div>

                        {/* Stats Grid */}
                        <div className="grid grid-cols-4 gap-4 mb-8">
                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">Calls (Lifetime)</div>
                                <div className="text-2xl font-bold font-mono">{selectedAgent.calls}</div>
                            </div>
                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">Deployed</div>
                                <div className="text-xl font-bold">{selectedAgent.created}</div>
                            </div>
                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">Lazarus Resurrections</div>
                                <div className={clsx("text-2xl font-bold font-mono", selectedAgent.lazarusEvents > 5 ? "text-red-500" : "text-gray-200")}>
                                    {selectedAgent.lazarusEvents}
                                </div>
                            </div>
                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">Status</div>
                                <div className="flex items-center gap-2">
                                    {selectedAgent.isSafe ? (
                                        <><span className="w-2 h-2 rounded-full bg-[#00ff41]"></span> <span className="font-bold text-[#00ff41]">PRODUCTION SAFE</span></>
                                    ) : (
                                        <><span className="w-2 h-2 rounded-full bg-red-500 animate-pulse"></span> <span className="font-bold text-red-500">UNSTABLE</span></>
                                    )}
                                </div>
                            </div>
                        </div>

                        {/* Monte Carlo Section */}
                        <div className="mb-8 p-6 bg-black border border-[#333] rounded-lg relative overflow-hidden group">
                            <div className="absolute top-0 right-0 p-4 opacity-10 font-bold text-6xl text-gray-500 select-none group-hover:opacity-20 transition-opacity">
                                🎲
                            </div>
                            <h3 className="text-lg font-bold mb-2">Monte Carlo Stress Test</h3>
                            <p className="text-sm text-gray-400 mb-6 max-w-xl">
                                Simulate 10,000 concurrent execution paths to verify thread safety and memory leaks.
                                This process runs in a sandboxed hypervisor and will not affect production.
                            </p>

                            {isSimulating ? (
                                <div className="w-full">
                                    <div className="flex justify-between text-xs font-mono mb-2">
                                        <span className="text-[#00ff41] animate-pulse">&gt;&gt; SPINNING UP UNIVERSES...</span>
                                        <span>{simProgress.toFixed(0)}%</span>
                                    </div>
                                    <div className="h-2 bg-[#222] rounded-full overflow-hidden">
                                        <div className="h-full bg-gradient-to-r from-blue-500 to-[#00ff41]" style={{ width: `${simProgress}%` }}></div>
                                    </div>
                                    <div className="grid grid-cols-4 gap-2 mt-4 text-[9px] font-mono text-gray-500">
                                        <div>P99 LATENCY: 42ms</div>
                                        <div>MEM LEAK: 0.00%</div>
                                        <div>RACE CONDS: 0</div>
                                        <div>MAX THREADS: 1024</div>
                                    </div>
                                </div>
                            ) : (
                                <button
                                    onClick={runMonteCarlo}
                                    className="bg-white text-black font-bold uppercase tracking-wider px-6 py-3 rounded hover:bg-[#00ff41] hover:text-black transition-colors"
                                >
                                    Start Simulation
                                </button>
                            )}
                        </div>

                        {/* Comments */}
                        <div>
                            <h3 className="text-xs font-bold uppercase tracking-widest text-gray-500 mb-4">Production Notes</h3>
                            <div className="space-y-4">
                                {selectedAgent.comments.map(c => (
                                    <div key={c.id} className="flex gap-4">
                                        <div className="w-8 h-8 rounded bg-[#222] flex items-center justify-center text-[10px] font-bold text-gray-400">
                                            {c.author.substring(0, 2).toUpperCase()}
                                        </div>
                                        <div className="flex-1 bg-[#111] p-3 rounded border border-[#222]">
                                            <div className="flex justify-between items-baseline mb-1">
                                                <span className="text-xs font-bold text-gray-200">{c.author} <span className="text-[10px] font-normal text-gray-500">({c.role})</span></span>
                                                <span className="text-[10px] text-gray-600 font-mono">{c.date}</span>
                                            </div>
                                            <p className="text-xs text-gray-400">{c.text}</p>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        </div>

                    </motion.div>
                </AnimatePresence>
            </div>

        </div>
    );
}
