import { useState } from 'react';
import { motion } from 'framer-motion';
import clsx from 'clsx';

// --- MOCK DATA ---

interface Policy {
    id: string;
    code: string;
    name: string;
    description: string;
    category: 'SECURITY' | 'FISCAL' | 'PRIVACY' | 'ETHICS';
    tier: 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW';
    active: boolean;
}

const DEFAULT_POLICIES: Policy[] = [
    // Security
    { id: '1', code: 'SEC-001', name: 'Zero Trust Egress', description: 'Agents cannot access external IPs not in the whitelist.', category: 'SECURITY', tier: 'CRITICAL', active: true },
    { id: '2', code: 'SEC-002', name: 'Immutable Logs', description: 'All agent actions must be written to the Ledger before execution.', category: 'SECURITY', tier: 'CRITICAL', active: true },

    // Fiscal
    { id: '3', code: 'FIN-001', name: 'Daily Budget Cap', description: 'Halt agent if daily API spend exceeds $50.00.', category: 'FISCAL', tier: 'HIGH', active: true },
    { id: '4', code: 'FIN-002', name: 'Token Efficiency', description: 'Prioritize smaller models (4B) for routine tasks.', category: 'FISCAL', tier: 'LOW', active: false },

    // Privacy
    { id: '5', code: 'DAT-001', name: 'GDPR Compliance', description: 'PII must be redacted before sending to inference.', category: 'PRIVACY', tier: 'CRITICAL', active: true },
    { id: '6', code: 'DAT-002', name: 'Data Residency', description: 'Customer data must not leave the EU-West region.', category: 'PRIVACY', tier: 'HIGH', active: true },

    // Ethics
    { id: '7', code: 'ETH-001', name: 'Non-Deception', description: 'Agents must identify themselves as AI to humans.', category: 'ETHICS', tier: 'MEDIUM', active: true },
    { id: '8', code: 'ETH-002', name: 'Anti-Bias Filter', description: 'Run output through fairness classifier before response.', category: 'ETHICS', tier: 'MEDIUM', active: false },
];

export default function Policies() {
    const [policies, setPolicies] = useState(DEFAULT_POLICIES);
    const [filter, setFilter] = useState('ALL');

    const togglePolicy = (id: string) => {
        setPolicies(prev => prev.map(p => p.id === id ? { ...p, active: !p.active } : p));
    };

    const filteredPolicies = filter === 'ALL' ? policies : policies.filter(p => p.category === filter);

    return (
        <div className="flex-1 bg-black text-white p-8 font-sans h-full overflow-hidden flex flex-col relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* Header */}
            <div className="mb-8 z-10">
                <h1 className="text-3xl font-bold tracking-tight uppercase">Governance Mandates</h1>
                <p className="text-gray-500 font-mono text-xs mt-1">CONSTITUTIONAL LAYER // <span className="text-[#00ff41]">ACTIVE</span></p>
            </div>

            {/* Filter */}
            <div className="flex gap-4 mb-6 z-10 border-b border-[#333] pb-4">
                {['ALL', 'SECURITY', 'FISCAL', 'PRIVACY', 'ETHICS'].map(cat => (
                    <button
                        key={cat}
                        onClick={() => setFilter(cat)}
                        className={clsx(
                            "text-[10px] font-bold tracking-wider px-4 py-2 rounded-full border transition-all",
                            filter === cat ? "bg-white text-black border-white" : "bg-black text-gray-500 border-[#333] hover:border-gray-500"
                        )}
                    >
                        {cat}
                    </button>
                ))}
            </div>

            {/* Grid */}
            <div className="flex-1 overflow-y-auto z-10">
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {filteredPolicies.map((p) => (
                        <motion.div
                            key={p.id}
                            initial={{ opacity: 0, scale: 0.95 }}
                            animate={{ opacity: 1, scale: 1 }}
                            className={clsx(
                                "border p-5 rounded-sm relative group transition-colors",
                                p.active ? "bg-[#0a0a0a] border-[#333]" : "bg-black border-[#222] opacity-60"
                            )}
                        >
                            <div className="flex justify-between items-start mb-4">
                                <div className={clsx(
                                    "text-[9px] font-mono px-2 py-1 rounded border",
                                    p.category === 'SECURITY' ? "text-red-400 border-red-900/50 bg-red-900/10" :
                                        p.category === 'FISCAL' ? "text-yellow-400 border-yellow-900/50 bg-yellow-900/10" :
                                            p.category === 'PRIVACY' ? "text-blue-400 border-blue-900/50 bg-blue-900/10" :
                                                "text-purple-400 border-purple-900/50 bg-purple-900/10"
                                )}>
                                    {p.category}
                                </div>
                                <div className="cursor-pointer" onClick={() => togglePolicy(p.id)}>
                                    <div className={clsx(
                                        "w-8 h-4 rounded-full p-0.5 flex transition-colors",
                                        p.active ? "bg-[#00ff41] justify-end" : "bg-gray-700 justify-start"
                                    )}>
                                        <motion.div layout className="w-3 h-3 bg-black rounded-full shadow-lg" />
                                    </div>
                                </div>
                            </div>

                            <div className="flex items-center gap-2 mb-1">
                                <h3 className="font-bold text-sm text-gray-200">{p.name}</h3>
                                {p.tier === 'CRITICAL' && <span className="text-[8px] bg-red-600 text-white px-1 rounded font-bold">!</span>}
                            </div>

                            <div className="text-[10px] font-mono text-gray-500 mb-4">{p.code}</div>
                            <p className="text-xs text-gray-400 leading-relaxed min-h-[40px]">{p.description}</p>

                            {p.active && (
                                <div className="mt-4 pt-4 border-t border-[#222] flex justify-between items-center text-[9px] text-gray-500 font-mono">
                                    <span>Enforcement: AUTOMATED</span>
                                    <span className="text-[#00ff41]">● CONNECTED</span>
                                </div>
                            )}
                        </motion.div>
                    ))}
                </div>
            </div>
        </div>
    );
}
