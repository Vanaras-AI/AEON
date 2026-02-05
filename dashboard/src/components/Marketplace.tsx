import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

interface AgentProduct {
    id: string;
    name: string;
    description: string;
    developer: string;
    price: string;
    verified: boolean;
    downloads: string;
    tags: string[];
    mandate: {
        network: string[];
        filesystem: string[];
    };
}

const PRODUCTS: AgentProduct[] = [
    {
        id: 'ag-01',
        name: 'DeepResearch v4',
        description: 'Recursive searching agent that summarizes 100+ sources.',
        developer: 'did:aeon:dev:0x71...',
        price: '0.01 AEON/op',
        verified: true,
        downloads: '12.4k',
        tags: ['Research', 'WASM', 'Python'],
        mandate: {
            network: ["api.openai.com", "google.com", "arxiv.org"],
            filesystem: ["READ *.pdf", "WRITE ./output/*"]
        }
    },
    {
        id: 'ag-02',
        name: 'Contract Auditor',
        description: 'Legal compliance checker for Solidity smart contracts.',
        developer: 'did:aeon:legal:0x99...',
        price: '0.50 AEON/op',
        verified: true,
        downloads: '3.1k',
        tags: ['Security', 'Audit', 'Finance'],
        mandate: {
            network: ["etherscan.io", "api.github.com"],
            filesystem: ["READ *.sol"]
        }
    },
    // ... rest of products simplified for brevity in prototype
];

const MandateModal = ({ agent, onConfirm, onCancel }: { agent: AgentProduct, onConfirm: () => void, onCancel: () => void }) => {
    return (
        <div className="absolute inset-0 z-[60] flex items-center justify-center bg-black/90 backdrop-blur-md p-12">
            <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="w-full max-w-2xl bg-[#050505] border border-[#00ff41]/30 shadow-[0_0_50px_rgba(0,255,65,0.1)] flex flex-col p-8"
            >
                <div className="flex items-center gap-4 mb-8">
                    <div className="w-12 h-12 bg-[#111] flex items-center justify-center border border-[#333] text-2xl">🛡️</div>
                    <div>
                        <h2 className="text-xl font-bold uppercase tracking-tight text-white">Review Agent Mandate</h2>
                        <p className="text-[10px] font-mono text-[#00ff41]">DNA VERIFICATION: {agent.id} // DID: {agent.developer.slice(0, 20)}...</p>
                    </div>
                </div>

                <div className="space-y-6 flex-1 overflow-y-auto pr-2">
                    <section>
                        <h3 className="text-[10px] uppercase font-mono text-gray-500 mb-2 border-b border-[#222] pb-1">Network Intent</h3>
                        <div className="space-y-1">
                            {agent.mandate.network.map(d => (
                                <div key={d} className="flex items-center gap-2 text-xs font-mono">
                                    <span className="text-[#00ff41]">→</span>
                                    <span className="text-gray-300">ALLOW CONNECT TO</span>
                                    <span className="text-white border-b border-dotted border-gray-600">{d}</span>
                                </div>
                            ))}
                        </div>
                    </section>

                    <section>
                        <h3 className="text-[10px] uppercase font-mono text-gray-500 mb-2 border-b border-[#222] pb-1">File System Intent</h3>
                        <div className="space-y-1">
                            {agent.mandate.filesystem.map(f => (
                                <div key={f} className="flex items-center gap-2 text-xs font-mono">
                                    <span className="text-[#00ff41]">→</span>
                                    <span className="text-gray-300">{f}</span>
                                </div>
                            ))}
                        </div>
                    </section>

                    <div className="p-4 bg-yellow-900/10 border border-yellow-500/30 rounded">
                        <p className="text-[10px] text-yellow-500 font-bold uppercase mb-1">⚖️ Sovereign Check</p>
                        <p className="text-[10px] text-yellow-500/80 leading-relaxed font-mono">
                            AEON has compared this mandate against your Sovereign Constitution.
                            <span className="text-white"> No conflicts detected.</span> All intents are within your global safety parameters.
                        </p>
                    </div>
                </div>

                <div className="flex justify-end gap-3 mt-8 pt-4 border-t border-[#222]">
                    <button onClick={onCancel} className="px-6 py-2 border border-[#333] text-xs uppercase text-gray-500 hover:text-white transition-colors">Abort</button>
                    <button onClick={onConfirm} className="px-6 py-2 bg-[#00ff41] text-black text-xs font-bold uppercase tracking-widest hover:bg-[#00cc33] transition-colors">Confirm Governance</button>
                </div>
            </motion.div>
        </div>
    );
};

export default function Marketplace({ onImport }: { onImport: (agent: AgentProduct) => void }) {
    const [filter, setFilter] = useState('ALL');
    const [selectedAgent, setSelectedAgent] = useState<AgentProduct | null>(null);

    return (
        <div className="flex-1 bg-black text-white p-8 overflow-y-auto font-sans h-full relative">
            <AnimatePresence>
                {selectedAgent && (
                    <MandateModal
                        agent={selectedAgent}
                        onCancel={() => setSelectedAgent(null)}
                        onConfirm={() => {
                            onImport(selectedAgent);
                            setSelectedAgent(null);
                        }}
                    />
                )}
            </AnimatePresence>

            {/* Background Grid */}
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-10 pointer-events-none"></div>

            {/* Header */}
            <div className="flex justify-between items-end mb-8 relative z-10">
                <div>
                    <h1 className="text-3xl font-bold tracking-tight uppercase">Agent Exchange <span className="text-[#00ff41]">A2X</span></h1>
                    <p className="text-gray-500 font-mono text-xs mt-2">SECURE WASM CONTAINER REGISTRY // VERIFIED BY CORTEX-G</p>
                </div>
                <div className="flex gap-4">
                    <div className="bg-[#111] border border-[#333] px-3 py-1 flex items-center gap-2 rounded">
                        <span className="text-gray-500 text-xs">BALANCE</span>
                        <span className="font-mono font-bold text-[#00ff41]">4,200.00 AEON</span>
                    </div>
                </div>
            </div>

            {/* Filter Bar */}
            <div className="flex gap-4 mb-8 border-b border-[#333] pb-4 sticky top-0 bg-black/80 backdrop-blur z-20 pt-4">
                {['ALL', 'VERIFIED', 'FINANCE', 'RESEARCH', 'DEV TOOLS'].map(tag => (
                    <button
                        key={tag}
                        onClick={() => setFilter(tag)}
                        className={clsx(
                            "text-[10px] font-bold tracking-wider px-4 py-2 rounded-full border transition-all",
                            filter === tag
                                ? "bg-white text-black border-white"
                                : "bg-black text-gray-500 border-[#333] hover:border-gray-500"
                        )}
                    >
                        {tag}
                    </button>
                ))}
            </div>

            {/* Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 relative z-10">
                {PRODUCTS.map((p) => (
                    <motion.div
                        key={p.id}
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        whileHover={{ y: -5 }}
                        className="bg-[#0a0a0a] border border-[#333] p-5 rounded-sm group hover:border-[#00ff41] transition-colors relative overflow-hidden"
                    >
                        {/* Verified Badge */}
                        {p.verified && (
                            <div className="absolute top-0 right-0 bg-[#00ff41]/10 text-[#00ff41] px-2 py-1 text-[8px] font-bold uppercase tracking-wider border-l border-b border-[#00ff41]/50">
                                Verified Safe
                            </div>
                        )}

                        <div className="h-12 w-12 bg-[#111] rounded flex items-center justify-center mb-4 text-2xl border border-[#222]">
                            🤖
                        </div>

                        <h3 className="font-bold text-lg leading-tight mb-1">{p.name}</h3>
                        <div className="text-[10px] text-gray-500 font-mono mb-4 truncate">{p.developer}</div>

                        <p className="text-sm text-gray-400 mb-6 h-10 leading-snug">{p.description}</p>

                        <div className="flex items-center gap-2 mb-6">
                            {p.tags.map(t => (
                                <span key={t} className="text-[9px] bg-[#111] text-gray-400 px-2 py-0.5 rounded border border-[#222]">{t}</span>
                            ))}
                        </div>

                        <div className="flex items-center justify-between mt-auto pt-4 border-t border-[#222]">
                            <div className="flex flex-col">
                                <span className="text-[10px] text-gray-500">PRICE</span>
                                <span className="font-mono text-sm font-bold text-white">{p.price}</span>
                            </div>
                            <button
                                onClick={() => setSelectedAgent(p)}
                                className="bg-white text-black text-[10px] font-bold px-4 py-2 rounded hover:bg-[#00ff41] hover:text-black transition-colors uppercase tracking-wider"
                            >
                                Import
                            </button>
                        </div>
                    </motion.div>
                ))}
            </div>
        </div>
    );
}
