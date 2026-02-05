import { motion } from 'framer-motion';
import clsx from 'clsx';
import { useState } from 'react';

// --- MOCK DATA ---

interface Interaction {
    id: string;
    role: 'AGENT' | 'DEV' | 'SYSTEM';
    name: string;
    action: string;
    detail: string;
    timestamp: string;
    status: 'PENDING' | 'APPROVED' | 'EXECUTED' | 'REJECTED';
}

interface Mission {
    id: string;
    name: string;
    description: string;
    members: string[]; // DIDs or Names
    interactions: Interaction[];
    active: boolean;
}

const MISSIONS: Mission[] = [
    {
        id: 'm-01',
        name: 'Project: Hydra Scale',
        description: 'Horizontal scaling implementation for Q2 traffic.',
        members: ['Senior Dev (Alice)', 'Infrastructure Agent v4', 'Cost Optimizer Bot'],
        active: true,
        interactions: [
            { id: 'i-1', role: 'AGENT', name: 'Infrastructure Agent', action: 'PROPOSAL', detail: 'Detected 15% latency spike. Propose spinning up 3x t4g.large instances in eu-west-1.', timestamp: '10:45 AM', status: 'PENDING' },
            { id: 'i-2', role: 'DEV', name: 'Alice (Lead)', action: 'REVIEW', detail: 'Approved. Ensure budget cap is set to $200/day.', timestamp: '10:48 AM', status: 'APPROVED' },
            { id: 'i-3', role: 'AGENT', name: 'Cost Optimizer', action: 'CONSTRAINT', detail: 'Budget cap verified. Injected spot request with max price $0.04.', timestamp: '10:49 AM', status: 'EXECUTED' }
        ]
    },
    {
        id: 'm-02',
        name: 'Hotfix: Auth Memory Leak',
        description: 'Urgent patch for OOM in identity service.',
        members: ['Security Eng (Bob)', 'Chaos Agent', 'CodeRefactor Unit'],
        active: true,
        interactions: [
            { id: 'i-4', role: 'AGENT', name: 'Chaos Agent', action: 'REPORT', detail: 'Fuzzing revealed 500MB leak in JWT parser after 10k reqs.', timestamp: '09:12 AM', status: 'EXECUTED' },
            { id: 'i-5', role: 'AGENT', name: 'CodeRefactor Unit', action: 'PATCH', detail: 'Generated fix using Rust `Arc<Mutex>` pattern. PR #892 opened.', timestamp: '09:15 AM', status: 'PENDING' },
            { id: 'i-6', role: 'DEV', name: 'Bob (Security)', action: 'DENIED', detail: 'Fix introduces deadlock risk. Use message passing instead.', timestamp: '09:30 AM', status: 'REJECTED' }
        ]
    },
    {
        id: 'm-03',
        name: 'Feature: Dark Mode',
        description: 'UI Polish for dashboard.',
        members: ['Frontend Dev', 'PixelGen-3'],
        active: false,
        interactions: [
            { id: 'i-7', role: 'AGENT', name: 'PixelGen-3', action: 'DESIGN', detail: 'Generated 3 variants for color palette based on "Cyberpunk" prompt.', timestamp: 'Yesterday', status: 'EXECUTED' },
            { id: 'i-8', role: 'DEV', name: 'Frontend Dev', action: 'SELECT', detail: 'Variant B chosen. Implementing SCSS variables.', timestamp: 'Yesterday', status: 'EXECUTED' }
        ]
    }
];

export default function Teams() {
    const [selectedMissionId, setSelectedMissionId] = useState<string>(MISSIONS[0].id);

    const selectedMission = MISSIONS.find(m => m.id === selectedMissionId) || MISSIONS[0];

    return (
        <div className="flex-1 bg-black text-white font-sans h-full overflow-hidden flex relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* LEFT COLUMN: MISSION LIST */}
            <div className="w-80 border-r border-[#333] flex flex-col z-10 bg-black">
                <div className="p-6 border-b border-[#333]">
                    <h2 className="text-xl font-bold uppercase tracking-tight">Active Operations</h2>
                    <p className="text-[10px] text-gray-500 font-mono mt-1">HUMAN-AGENT COLLABORATION</p>
                </div>
                <div className="flex-1 overflow-y-auto">
                    {MISSIONS.map(m => (
                        <div
                            key={m.id}
                            onClick={() => setSelectedMissionId(m.id)}
                            className={clsx(
                                "p-4 border-b border-[#222] cursor-pointer hover:bg-[#111] transition-colors group",
                                selectedMissionId === m.id ? "bg-[#111] border-l-2 border-l-orange-500" : "border-l-2 border-l-transparent"
                            )}
                        >
                            <div className="flex justify-between items-center mb-1">
                                <span className={clsx("font-bold text-sm", selectedMissionId === m.id ? "text-white" : "text-gray-400 group-hover:text-white")}>{m.name}</span>
                                {m.active && <span className="w-1.5 h-1.5 rounded-full bg-orange-500 animate-pulse"></span>}
                            </div>
                            <div className="text-[10px] text-gray-600 truncate mb-1">{m.description}</div>
                            <div className="flex -space-x-1 mt-2">
                                {m.members.map((mem, i) => (
                                    <div key={i} className="w-4 h-4 rounded-full bg-[#222] border border-black flex items-center justify-center text-[6px] text-gray-400 font-bold" title={mem}>
                                        {mem[0]}
                                    </div>
                                ))}
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* RIGHT COLUMN: INTERACTION FEED */}
            <div className="flex-1 flex flex-col z-10 bg-[#050505]">
                {/* Header */}
                <div className="px-8 py-6 border-b border-[#333] flex justify-between items-center bg-black/50 backdrop-blur">
                    <div>
                        <h1 className="text-2xl font-bold">{selectedMission.name}</h1>
                        <p className="text-xs text-gray-500 font-mono mt-1">MISSION ID: {selectedMission.id.toUpperCase()}</p>
                    </div>
                    <div className="flex gap-2">
                        <button className="text-[10px] font-bold uppercase border border-[#333] bg-[#111] px-3 py-1.5 rounded text-gray-400 hover:text-white transition-colors">Archive</button>
                        <button className="text-[10px] font-bold uppercase border border-orange-500/50 bg-orange-500/10 text-orange-500 px-3 py-1.5 rounded hover:bg-orange-500/20 transition-colors">Assign New Agent</button>
                    </div>
                </div>

                {/* Feed */}
                <div className="flex-1 overflow-y-auto p-8 space-y-6">
                    {selectedMission.interactions.map((interaction, i) => (
                        <motion.div
                            key={interaction.id}
                            initial={{ opacity: 0, y: 10 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ delay: i * 0.1 }}
                            className="flex gap-4 group"
                        >
                            {/* Icon Column */}
                            <div className="flex flex-col items-center">
                                <div className={clsx(
                                    "w-8 h-8 rounded flex items-center justify-center border text-xs font-bold shadow-lg z-10",
                                    interaction.role === 'AGENT' ? "bg-[#111] border-[#333] text-blue-400" : "bg-gray-800 border-gray-600 text-white"
                                )}>
                                    {interaction.role === 'AGENT' ? '🤖' : '👨‍💻'}
                                </div>
                                {i !== selectedMission.interactions.length - 1 && (
                                    <div className="w-[1px] bg-[#222] flex-1 min-h-[20px] my-1"></div>
                                )}
                            </div>

                            {/* Content Card */}
                            <div className="flex-1 max-w-4xl">
                                <div className="flex items-baseline gap-2 mb-1">
                                    <span className="font-bold text-xs text-gray-300">{interaction.name}</span>
                                    <span className="text-[10px] font-mono text-gray-600">{interaction.timestamp}</span>
                                </div>

                                <div className={clsx(
                                    "p-4 rounded border relative",
                                    interaction.status === 'REJECTED' ? "bg-red-900/10 border-red-900/40" :
                                        interaction.role === 'AGENT' ? "bg-[#0a0a0a] border-[#222]" : "bg-black border-[#333]"
                                )}>
                                    <div className="flex justify-between items-start mb-2">
                                        <span className={clsx("text-[9px] font-mono uppercase px-1.5 py-0.5 rounded border",
                                            interaction.action === 'PROPOSAL' ? "text-blue-400 border-blue-900 bg-blue-900/20" :
                                                interaction.action === 'REVIEW' ? "text-[#00ff41] border-[#005515] bg-[#00ff41]/10" :
                                                    interaction.action === 'DENIED' ? "text-red-400 border-red-900 bg-red-900/20" :
                                                        "text-gray-400 border-gray-800 bg-gray-800/20"
                                        )}>
                                            {interaction.action}
                                        </span>
                                        {interaction.status === 'PENDING' && <span className="text-[8px] bg-yellow-500/20 text-yellow-500 px-1 rounded animate-pulse">Awaiting Approval</span>}
                                    </div>

                                    <p className="text-sm text-gray-300 leading-relaxed font-mono">{interaction.detail}</p>

                                    {/* Action Buttons (Mock) */}
                                    {interaction.role === 'AGENT' && interaction.status === 'PENDING' && (
                                        <div className="mt-4 pt-3 border-t border-[#222] flex gap-3">
                                            <button className="text-[10px] font-bold text-[#00ff41] hover:underline">[ APPROVE ]</button>
                                            <button className="text-[10px] font-bold text-red-500 hover:underline">[ DENY ]</button>
                                            <button className="text-[10px] font-bold text-gray-500 hover:underline">[ EDIT ]</button>
                                        </div>
                                    )}
                                </div>
                            </div>
                        </motion.div>
                    ))}
                </div>
            </div>
        </div>
    );
}
