import { motion } from 'framer-motion';
import clsx from 'clsx';
import { useMemo } from 'react';

// --- MOCK DATA ---

// 1. Efficiency Scatter Data
const SCATTER_DATA = Array.from({ length: 30 }).map((_, i) => ({
    id: i,
    x: Math.random() * 100, // Ops/Sec (Normalized)
    y: Math.random() * 100, // Error Rate (Inverse)
    size: Math.random() * 10 + 5,
    cluster: Math.random() > 0.7 ? 'ANOMALY' : 'NOMINAL'
}));

// 2. Heatmap Data (Node vs Resource)
const NODES = [
    'DeepResearch (did:aeon:dev:0x71...)',
    'ContractAuditor (did:aeon:legal:0x99...)',
    'PixelGen-3 (did:aeon:art:0x22...)',
    'TwitterSent (did:aeon:soc:0x55...)',
    'DevUnit-1 (did:aeon:dev:0x8f...)'
];
const METRICS = ['CPU', 'RAM', 'NET', 'DISK', 'GPU'];
const HEATMAP_DATA = NODES.map(node => ({
    node,
    values: METRICS.map(metric => ({
        metric,
        value: Math.random()
    }))
}));

export default function Correlations() {
    return (
        <div className="flex-1 bg-black text-white p-8 font-sans h-full overflow-hidden flex flex-col relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* Header */}
            <div className="mb-8 z-10 flex justify-between items-end">
                <div>
                    <h1 className="text-3xl font-bold tracking-tight uppercase">System Correlations</h1>
                    <p className="text-gray-500 font-mono text-xs mt-1">MULTIVARIATE ANALYSIS // <span className="text-[#00ff41]">REAL-TIME</span></p>
                </div>
            </div>

            {/* Grid */}
            <div className="grid grid-cols-2 gap-8 h-full min-h-0 pb-8 z-10">

                {/* 1. EFFICIENCY MATRIX (Scatter) */}
                <div className="bg-[#0a0a0a] border border-[#333] p-6 flex flex-col">
                    <div className="flex justify-between items-center mb-6">
                        <h3 className="text-xs font-bold uppercase tracking-widest text-gray-400">Yield Efficiency (Ops/Error)</h3>
                        <div className="flex gap-4 text-[9px] font-mono">
                            <div className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-[#00ff41]"></span>NOMINAL</div>
                            <div className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-red-500"></span>ANOMALY</div>
                        </div>
                    </div>

                    <div className="flex-1 border-l border-b border-[#222] relative m-4">
                        {/* Grid Lines */}
                        <div className="absolute inset-0 flex flex-col justify-between pointer-events-none opacity-20">
                            {[1, 2, 3, 4].map(i => <div key={i} className="h-[1px] bg-gray-500 w-full"></div>)}
                        </div>
                        <div className="absolute inset-0 flex flex-row justify-between pointer-events-none opacity-20">
                            {[1, 2, 3, 4].map(i => <div key={i} className="w-[1px] bg-gray-500 h-full"></div>)}
                        </div>

                        {/* Labels */}
                        <span className="absolute -left-6 top-1/2 -rotate-90 text-[9px] text-gray-500 tracking-widest uppercase">Performance</span>
                        <span className="absolute bottom-[-24px] left-1/2 -translate-x-1/2 text-[9px] text-gray-500 tracking-widest uppercase">Throughput</span>

                        {/* Points */}
                        {SCATTER_DATA.map((pt) => (
                            <motion.div
                                key={pt.id}
                                initial={{ opacity: 0, scale: 0 }}
                                animate={{ opacity: 1, scale: 1 }}
                                transition={{ delay: pt.id * 0.02 }}
                                style={{ left: `${pt.x}%`, bottom: `${pt.y}%`, width: pt.size, height: pt.size }}
                                className={clsx(
                                    "absolute rounded-full opacity-60 border border-black cursor-pointer hover:opacity-100 hover:scale-150 transition-all",
                                    pt.cluster === 'ANOMALY' ? 'bg-red-500 shadow-[0_0_10px_rgba(239,68,68,0.5)]' : 'bg-[#00ff41] shadow-[0_0_5px_rgba(0,255,65,0.3)]'
                                )}
                            />
                        ))}
                    </div>
                </div>

                {/* 2. RESOURCE HEATMAP */}
                <div className="bg-[#0a0a0a] border border-[#333] p-6 flex flex-col">
                    <div className="flex justify-between items-center mb-6">
                        <h3 className="text-xs font-bold uppercase tracking-widest text-gray-400">Resource Saturation Heatmap</h3>
                    </div>

                    <div className="flex-1 flex flex-col justify-center gap-2">
                        {/* Header Row */}
                        <div className="flex">
                            <div className="w-56"></div>
                            {METRICS.map(m => (
                                <div key={m} className="flex-1 text-[9px] font-mono text-center text-gray-500">{m}</div>
                            ))}
                        </div>

                        {/* Rows */}
                        {HEATMAP_DATA.map((row, i) => (
                            <div key={row.node} className="flex items-center gap-1 group">
                                <div className="w-56 text-[10px] font-mono text-gray-400 text-right pr-4 flex flex-col justify-center h-full">
                                    <span className="font-bold text-white">{row.node.split(' (')[0]}</span>
                                    <span className="text-[8px] text-gray-600">({row.node.split(' (')[1]}</span>
                                </div>
                                {row.values.map((cell, j) => (
                                    <motion.div
                                        key={cell.metric}
                                        initial={{ opacity: 0 }}
                                        animate={{ opacity: 1 }}
                                        transition={{ delay: (i * 5 + j) * 0.05 }}
                                        className="flex-1 h-8 rounded-sm relative group-hover:opacity-80 transition-opacity"
                                        style={{ backgroundColor: `rgba(0, 255, 65, ${cell.value * 0.8})` }}
                                    >
                                        <div className="absolute inset-0 flex items-center justify-center opacity-0 hover:opacity-100 font-mono text-[9px] font-bold text-black pointer-events-none">
                                            {(cell.value * 100).toFixed(0)}%
                                        </div>
                                    </motion.div>
                                ))}
                            </div>
                        ))}
                    </div>

                    <div className="mt-6 flex justify-between items-center bg-[#111] p-3 rounded border border-[#222]">
                        <span className="text-[10px] text-gray-400 uppercase">System Coherence Score</span>
                        <div className="flex items-baseline gap-1">
                            <span className="text-2xl font-bold text-[#00ff41]">98.2%</span>
                            <span className="text-[9px] text-[#00ff41]">+0.4%</span>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    );
}
