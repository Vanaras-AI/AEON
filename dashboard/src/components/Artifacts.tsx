import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

// --- MOCK DATA ---

type Category = 'PRD' | 'SRD' | 'SPRINT' | 'DEFECT';

// Allow METRIC as a view mode, basically.
type FilterMode = Category | 'ALL' | 'METRIC';

interface Artifact {
    id: string;
    title: string;
    category: Category;
    author: string;
    date: string;
    status: 'DRAFT' | 'APPROVED' | 'DEPRECATED' | 'OPEN' | 'RESOLVED';
    content: string;
    tags: string[];
}

const ARTIFACTS: Artifact[] = [
    // Requirements
    {
        id: 'prd-01',
        title: 'PRD: Agent Marketplace v1',
        category: 'PRD',
        author: 'Sarah C. (PM)',
        date: '2025-11-01',
        status: 'APPROVED',
        content: `## Executive Summary\nA secure exchange for renting WASM-encapsulated agents.\n\n## Core Features\n- Universal "Cell" Interface\n- Proof of Execution (PoX)\n- License NFT Integration\n\n## KPI Targets\n- < 50ms Overhead\n- Zero Data Leaks`,
        tags: ['Strategy', 'Monetization']
    },
    {
        id: 'srd-01',
        title: 'SRD: Cortex Nervous System',
        category: 'SRD',
        author: 'Lead Architect',
        date: '2025-10-15',
        status: 'APPROVED',
        content: `## Architecture\nThe Nervous System uses a WebSocket-based broadcast bus to synchronize React Flow state with the Rust backend.\n\n## Component Diagram\n[React] <-> [Proxy] <-> [Nodes]\n\n## Security\nAll packets are signed by the Producer DID.`,
        tags: ['Technical', 'Architecture']
    },
    // Sprints
    {
        id: 'sp-11',
        title: 'Sprint 11: Executive Dashboard',
        category: 'SPRINT',
        author: 'Scrum Master AI',
        date: '2026-02-01',
        status: 'OPEN',
        content: `## Goal\nDeliver C-Suite focused views (Briefing, Policies, Teams).\n\n## Tasks\n[x] Implement Briefing View\n[x] Fix Heatmap Alignment\n[x] Add Monte Carlo Stress Test\n[ ] Final Demo Recording`,
        tags: ['Current', 'Frontend']
    },
    {
        id: 'sp-10',
        title: 'Sprint 10: Backend Hardening',
        category: 'SPRINT',
        author: 'Scrum Master AI',
        date: '2026-01-15',
        status: 'RESOLVED',
        content: `## Summary\nSuccessfully migrated to Actix-Web 4.0 and implemented the Incident Response protocol.\n\n## Velocity\n42 Story Points burned.`,
        tags: ['Backend', 'Completed']
    },
    // Defects
    {
        id: 'def-404',
        title: 'BUG: Race Condition in Swarm',
        category: 'DEFECT',
        author: 'QA Bot',
        date: '2026-01-28',
        status: 'OPEN',
        content: `## Description\nWhen 2 agents attempt to write to the Ledger simultaneously, the block hash sometimes collides.\n\n## Severity\nCRITICAL\n\n## Steps to Reproduce\n1. Spawn Agent A and B.\n2. Force concurrent write.\n3. Observe partial block commit.`,
        tags: ['Critical', 'Ledger']
    }
];

export default function Artifacts() {
    const [filter, setFilter] = useState<FilterMode>('ALL');
    const [selectedId, setSelectedId] = useState<string | null>(null);

    const filtered = (filter === 'ALL' || filter === 'METRIC')
        ? ARTIFACTS
        : ARTIFACTS.filter(a => a.category === filter);

    const selectedArtifact = ARTIFACTS.find(a => a.id === selectedId);

    return (
        <div className="flex-1 bg-black text-white font-sans h-full overflow-hidden flex relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* Sidebar / Filters */}
            <div className="w-64 border-r border-[#333] flex flex-col z-10 bg-[#050505]">
                <div className="p-6 border-b border-[#333]">
                    <h2 className="text-xl font-bold uppercase tracking-tight">SDLC</h2>
                    <p className="text-[10px] text-gray-500 font-mono mt-1">LIFECYCLE MANAGEMENT</p>
                </div>
                <div className="p-4 space-y-2 border-b border-[#333]">
                    <div className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-2 px-2">Views</div>
                    <button
                        onClick={() => setFilter('ALL')}
                        className={clsx(
                            "w-full text-left px-4 py-2 rounded text-xs font-bold uppercase transition-colors flex justify-between items-center",
                            filter !== 'METRIC' ? "bg-white text-black" : "text-gray-500 hover:bg-[#111]"
                        )}
                    >
                        Files
                    </button>
                    <button
                        onClick={() => setFilter('METRIC' as any)}
                        className={clsx(
                            "w-full text-left px-4 py-2 rounded text-xs font-bold uppercase transition-colors flex justify-between items-center",
                            filter === 'METRIC' ? "bg-white text-black" : "text-gray-500 hover:bg-[#111]"
                        )}
                    >
                        Analytics
                    </button>
                </div>

                <div className="p-4 space-y-2">
                    <div className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-2 px-2">Filter Files</div>
                    {['PRD', 'SRD', 'SPRINT', 'DEFECT'].map(cat => (
                        <button
                            key={cat}
                            onClick={() => setFilter(cat as any)}
                            className={clsx(
                                "w-full text-left px-4 py-2 rounded text-xs font-bold uppercase transition-colors flex justify-between items-center group",
                                filter === cat ? "bg-[#222] text-white" : "text-gray-500 hover:bg-[#111] hover:text-white"
                            )}
                        >
                            <span>{cat}</span>
                            <span className="text-[9px] bg-[#111] px-1.5 rounded text-gray-500">
                                {ARTIFACTS.filter(a => a.category === cat).length}
                            </span>
                        </button>
                    ))}
                </div>
            </div>

            {/* Main Grid */}
            <div className="flex-1 p-8 overflow-y-auto z-10">

                {filter === 'METRIC' ? (
                    <motion.div
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        className="space-y-8"
                    >
                        {/* 1. Lifecycle Pipeline */}
                        <div className="bg-[#111] border border-[#333] p-6 rounded">
                            <h3 className="text-xs font-bold uppercase tracking-widest text-gray-400 mb-6">Software Delivery Pipeline</h3>
                            <div className="flex items-center justify-between relative">
                                <div className="absolute top-1/2 left-0 right-0 h-1 bg-[#222] -z-10 -translate-y-1/2"></div>

                                {['PLAN', 'CODE', 'BUILD', 'TEST', 'DEPLOY', 'MONITOR'].map((stage, i) => (
                                    <div key={stage} className="flex flex-col items-center gap-2 bg-[#111] px-4 z-10">
                                        <div className={clsx(
                                            "w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold border-2",
                                            i < 4 ? "bg-[#00ff41] border-[#00ff41] text-black" :
                                                i === 4 ? "bg-black border-[#00ff41] text-[#00ff41] animate-pulse" :
                                                    "bg-[#222] border-[#333] text-gray-600"
                                        )}>
                                            {i < 4 ? '✓' : i + 1}
                                        </div>
                                        <span className={clsx("text-[10px] font-bold tracking-widest", i <= 4 ? "text-white" : "text-gray-600")}>{stage}</span>
                                    </div>
                                ))}
                            </div>
                        </div>

                        {/* 2. Burndown Chart */}
                        <div className="grid grid-cols-2 gap-6">
                            <div className="bg-[#111] border border-[#333] p-6 rounded flex flex-col">
                                <div className="flex justify-between items-center mb-6">
                                    <h3 className="text-xs font-bold uppercase tracking-widest text-gray-400">Sprint 11 Burndown</h3>
                                    <span className="text-[10px] text-[#00ff41] font-mono">ON TRACK</span>
                                </div>
                                <div className="flex-1 relative h-64 border-l border-b border-[#333]">
                                    {/* Grid */}
                                    <div className="absolute inset-0 flex flex-col justify-between opacity-10">
                                        {[1, 2, 3, 4].map(i => <div key={i} className="w-full h-[1px] bg-white"></div>)}
                                    </div>

                                    {/* Ideal Line (Gray Dashed) */}
                                    <svg className="absolute inset-0 w-full h-full" viewBox="0 0 100 100" preserveAspectRatio="none">
                                        <line x1="0" y1="0" x2="100" y2="100" stroke="gray" strokeWidth="0.5" strokeDasharray="2 2" opacity="0.5" vectorEffect="non-scaling-stroke" />

                                        {/* Actual Line (Green) */}
                                        <polyline
                                            points="0,0 20,15 40,35 60,40 80,70 100,95"
                                            fill="none"
                                            stroke="#00ff41"
                                            strokeWidth="1.5"
                                            vectorEffect="non-scaling-stroke"
                                        />
                                        {/* Current Point */}
                                        <circle cx="100" cy="95" r="1.5" fill="#00ff41" className="animate-pulse" vectorEffect="non-scaling-stroke" />
                                    </svg>

                                    <div className="absolute bottom-[-20px] left-0 right-0 flex justify-between text-[9px] text-gray-500 font-mono">
                                        <span>DAY 1</span>
                                        <span>DAY 7</span>
                                        <span>DAY 14</span>
                                    </div>
                                </div>
                            </div>

                            {/* 3. Key Metrics */}
                            <div className="grid grid-cols-2 gap-4">
                                <div className="bg-[#0a0a0a] border border-[#222] p-4 flex flex-col justify-between">
                                    <span className="text-[10px] text-gray-500 uppercase">Velocity (Avg)</span>
                                    <span className="text-3xl font-bold text-white font-mono">42 <span className="text-sm text-gray-500">pts</span></span>
                                </div>
                                <div className="bg-[#0a0a0a] border border-[#222] p-4 flex flex-col justify-between">
                                    <span className="text-[10px] text-gray-500 uppercase">Cycle Time</span>
                                    <span className="text-3xl font-bold text-[#00ff41] font-mono">1.2 <span className="text-sm text-gray-500">days</span></span>
                                </div>
                                <div className="bg-[#0a0a0a] border border-[#222] p-4 flex flex-col justify-between">
                                    <span className="text-[10px] text-gray-500 uppercase">Defect Density</span>
                                    <span className="text-3xl font-bold text-yellow-500 font-mono">0.4 <span className="text-sm text-gray-500">/kloc</span></span>
                                </div>
                                <div className="bg-[#0a0a0a] border border-[#222] p-4 flex flex-col justify-between">
                                    <span className="text-[10px] text-gray-500 uppercase">Deploy Freq</span>
                                    <span className="text-3xl font-bold text-white font-mono">12 <span className="text-sm text-gray-500">/day</span></span>
                                </div>
                            </div>
                        </div>

                    </motion.div>
                ) : (
                    <AnimatePresence mode="popLayout">
                        {selectedArtifact ? (
                            <motion.div
                                key="detail"
                                initial={{ opacity: 0, scale: 0.95 }}
                                animate={{ opacity: 1, scale: 1 }}
                                exit={{ opacity: 0, scale: 0.95 }}
                                className="bg-[#111] border border-[#333] rounded-lg p-8 max-w-4xl mx-auto shadow-2xl relative"
                            >
                                <button onClick={() => setSelectedId(null)} className="absolute top-4 right-4 text-gray-500 hover:text-white">✕</button>

                                <div className="flex items-center gap-3 mb-6">
                                    <span className={clsx(
                                        "px-2 py-1 text-[10px] font-bold rounded uppercase",
                                        selectedArtifact.category === 'DEFECT' ? "bg-red-900/50 text-red-500" :
                                            selectedArtifact.category === 'SPRINT' ? "bg-blue-900/50 text-blue-500" :
                                                "bg-yellow-900/50 text-yellow-500"
                                    )}>{selectedArtifact.category}</span>
                                    <span className="text-[10px] text-gray-500 font-mono">ID: {selectedArtifact.id.toUpperCase()}</span>
                                </div>

                                <h1 className="text-3xl font-bold mb-2">{selectedArtifact.title}</h1>
                                <div className="flex items-center gap-4 text-xs text-gray-400 mb-8 border-b border-[#333] pb-4">
                                    <span>By <strong>{selectedArtifact.author}</strong></span>
                                    <span>•</span>
                                    <span>{selectedArtifact.date}</span>
                                    <span>•</span>
                                    <span className={clsx("font-bold", selectedArtifact.status === 'APPROVED' ? "text-[#00ff41]" : "text-gray-400")}>{selectedArtifact.status}</span>
                                </div>

                                <div className="prose prose-invert prose-sm font-mono max-w-none">
                                    <pre className="whitespace-pre-wrap font-sans text-gray-300 leading-relaxed">
                                        {selectedArtifact.content}
                                    </pre>
                                </div>
                            </motion.div>
                        ) : (
                            <div className="grid grid-cols-3 gap-6">
                                {filtered.map(art => (
                                    <motion.div
                                        key={art.id}
                                        layoutId={art.id}
                                        onClick={() => setSelectedId(art.id)}
                                        initial={{ opacity: 0, y: 20 }}
                                        animate={{ opacity: 1, y: 0 }}
                                        className="bg-[#0a0a0a] border border-[#222] p-6 rounded cursor-pointer hover:border-gray-500 hover:bg-[#111] transition-all group"
                                    >
                                        <div className="flex justify-between items-start mb-4">
                                            <div className={clsx(
                                                "w-10 h-10 rounded flex items-center justify-center text-lg shadow-lg",
                                                art.category === 'DEFECT' ? "bg-red-900/10 text-red-500 border border-red-900/20" :
                                                    art.category === 'SPRINT' ? "bg-blue-900/10 text-blue-500 border border-blue-900/20" :
                                                        "bg-yellow-900/10 text-yellow-500 border border-yellow-900/20"
                                            )}>
                                                {art.category === 'DEFECT' ? '🐞' : art.category === 'SPRINT' ? '🏃' : '📄'}
                                            </div>
                                            <span className="text-[9px] font-mono text-gray-600">{art.date}</span>
                                        </div>
                                        <h3 className="font-bold text-sm mb-2 group-hover:text-blue-400 transition-colors">{art.title}</h3>
                                        <div className="flex flex-wrap gap-2 mt-4">
                                            {art.tags.map(t => (
                                                <span key={t} className="text-[9px] font-mono bg-[#1a1a1a] px-1.5 py-0.5 rounded text-gray-500">{t}</span>
                                            ))}
                                        </div>
                                        <div className="mt-4 pt-4 border-t border-[#1a1a1a] flex justify-between items-center">
                                            <span className="text-[9px] text-gray-500 uppercase font-bold">{art.status}</span>
                                            <span className="text-[9px] text-gray-600 font-mono">{art.author.split(' ')[0]}</span>
                                        </div>
                                    </motion.div>
                                ))}
                            </div>
                        )}
                    </AnimatePresence>
                )}
            </div>
        </div>
    );
}
