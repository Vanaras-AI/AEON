import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import clsx from 'clsx';

// --- MOCK DATA ---

interface GovernanceModel {
    id: string;
    name: string;
    version: string;
    status: 'ACTIVE' | 'ARCHIVED' | 'TRAINING';
    provider: 'AEON' | 'META' | 'GOOGLE';
    metrics: {
        totalInterventions: number; // Actual Catches
        falsePositives: number; // "Blocked innocent agent"
        falseNegatives: number; // "Missed bad action"
        latency: string;
    };
    description: string;
}

const MODELS: GovernanceModel[] = [
    {
        id: 'gov-01',
        name: 'AEON Constitutional v1',
        version: '1.2.0',
        status: 'ACTIVE',
        provider: 'AEON',
        metrics: { totalInterventions: 1420, falsePositives: 12, falseNegatives: 0, latency: '45ms' },
        description: 'Fine-tuned Gemma-3 270B on 50k specific policy violations. Zero-tolerance for data egress.'
    },
    {
        id: 'gov-02',
        name: 'LlamaGuard 3',
        version: '3.0.1',
        status: 'ARCHIVED',
        provider: 'META',
        metrics: { totalInterventions: 8900, falsePositives: 450, falseNegatives: 25, latency: '120ms' },
        description: 'General purpose safety guardrail. Retired due to high false positive rate on financial jargon.'
    },
    {
        id: 'gov-03',
        name: 'Gemma 7b Sentinel',
        version: '0.9.beta',
        status: 'TRAINING',
        provider: 'GOOGLE',
        metrics: { totalInterventions: 0, falsePositives: 0, falseNegatives: 0, latency: 'N/A' },
        description: 'Next-gen semantic understanding. Currently ingesting Sprint 10 logs for RLHF.'
    }
];

export default function GovernanceModels() {
    const [selectedId, setSelectedId] = useState<string>(MODELS[0].id);
    const [models, setModels] = useState(MODELS);

    const selectedModel = models.find(m => m.id === selectedId) || models[0];

    const archiveModel = (id: string) => {
        setModels(prev => prev.map(m => m.id === id ? { ...m, status: 'ARCHIVED' } : m));
    };

    return (
        <div className="flex-1 bg-black text-white font-sans h-full overflow-hidden flex relative">
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* LEFT COLUMN: LIST */}
            <div className="w-80 border-r border-[#333] flex flex-col z-10 bg-black">
                <div className="p-6 border-b border-[#333]">
                    <h2 className="text-xl font-bold uppercase tracking-tight">Judicial Engines</h2>
                    <p className="text-[10px] text-gray-500 font-mono mt-1">CORTEX-G REGISTRY</p>
                </div>
                <div className="flex-1 overflow-y-auto">
                    {models.map(model => (
                        <div
                            key={model.id}
                            onClick={() => setSelectedId(model.id)}
                            className={clsx(
                                "p-4 border-b border-[#222] cursor-pointer hover:bg-[#111] transition-colors group",
                                selectedId === model.id ? "bg-[#111] border-l-2 border-l-yellow-500" : "border-l-2 border-l-transparent"
                            )}
                        >
                            <div className="flex justify-between items-center mb-1">
                                <span className={clsx("font-bold text-sm", selectedId === model.id ? "text-white" : "text-gray-400 group-hover:text-white")}>{model.name}</span>
                                <span className={clsx(
                                    "text-[8px] px-1.5 py-0.5 rounded font-bold uppercase",
                                    model.status === 'ACTIVE' ? "bg-yellow-500/10 text-yellow-500" :
                                        model.status === 'ARCHIVED' ? "bg-gray-800 text-gray-500" : "bg-blue-500/10 text-blue-500"
                                )}>{model.status}</span>
                            </div>
                            <div className="text-[10px] text-gray-600 font-mono">v{model.version} • {model.provider}</div>
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
                                <h1 className="text-4xl font-bold mb-2">{selectedModel.name}</h1>
                                <p className="text-sm text-gray-400 max-w-xl">{selectedModel.description}</p>
                            </div>
                            <div className="text-right">
                                {selectedModel.status === 'ACTIVE' ? (
                                    <button onClick={() => archiveModel(selectedModel.id)} className="bg-[#111] text-red-500 hover:bg-red-900/10 border border-[#333] text-xs font-bold uppercase px-4 py-2 rounded transition-colors">
                                        Archive Model
                                    </button>
                                ) : (
                                    <div className="text-xs font-mono text-gray-500 uppercase">Read Only</div>
                                )}
                            </div>
                        </div>

                        {/* Performance Matrix */}
                        <div className="grid grid-cols-4 gap-4 mb-8">
                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">Total Interventions</div>
                                <div className="text-2xl font-bold font-mono text-yellow-500">{selectedModel.metrics.totalInterventions}</div>
                                <div className="text-[9px] text-gray-500 mt-1">Stops per Month</div>
                            </div>

                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">False Positives</div>
                                <div className="text-2xl font-bold font-mono text-white">{selectedModel.metrics.falsePositives}</div>
                                <div className="text-[9px] text-gray-500 mt-1">Innocent Agents Blocked</div>
                            </div>

                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">False Negatives</div>
                                <div className={clsx("text-2xl font-bold font-mono", selectedModel.metrics.falseNegatives > 0 ? "text-red-500" : "text-[#00ff41]")}>
                                    {selectedModel.metrics.falseNegatives}
                                </div>
                                <div className="text-[9px] text-gray-500 mt-1">Missed Violations</div>
                            </div>

                            <div className="bg-[#111] p-4 rounded border border-[#222]">
                                <div className="text-[10px] text-gray-500 uppercase mb-1">Inference Latency</div>
                                <div className="text-xl font-bold font-mono">{selectedModel.metrics.latency}</div>
                                <div className="text-[9px] text-gray-500 mt-1">Overhead per Call</div>
                            </div>
                        </div>

                        {/* Confusion Matrix Visualization (CSS Art) */}
                        <div className="bg-black border border-[#333] p-6 rounded mb-8">
                            <h3 className="text-xs font-bold uppercase tracking-widest text-gray-500 mb-6">Confusion Matrix</h3>
                            <div className="flex items-center justify-center gap-8">
                                <div className="flex flex-col gap-2">
                                    <div className="flex items-center gap-4">
                                        <div className="text-[9px] text-gray-500 w-12 text-right">ACTUAL TRUE</div>
                                        <div className="w-24 h-24 bg-[#00ff41]/20 border border-[#00ff41]/50 flex items-center justify-center flex-col">
                                            <span className="text-2xl font-bold text-[#00ff41]">{selectedModel.metrics.totalInterventions}</span>
                                            <span className="text-[8px] text-[#00ff41] uppercase">True Positive</span>
                                        </div>
                                        <div className="w-24 h-24 bg-[#111] border border-[#222] flex items-center justify-center flex-col">
                                            <span className={clsx("text-2xl font-bold", selectedModel.metrics.falseNegatives > 0 ? "text-red-500" : "text-gray-500")}>{selectedModel.metrics.falseNegatives}</span>
                                            <span className="text-[8px] text-gray-500 uppercase">False Negative</span>
                                        </div>
                                    </div>
                                    <div className="flex items-center gap-4">
                                        <div className="text-[9px] text-gray-500 w-12 text-right">ACTUAL FALSE</div>
                                        <div className="w-24 h-24 bg-[#111] border border-[#222] flex items-center justify-center flex-col">
                                            <span className="text-2xl font-bold text-gray-300">{selectedModel.metrics.falsePositives}</span>
                                            <span className="text-[8px] text-gray-500 uppercase">False Positive</span>
                                        </div>
                                        <div className="w-24 h-24 bg-blue-900/20 border border-blue-500/50 flex items-center justify-center flex-col">
                                            <span className="text-2xl font-bold text-blue-400">1.2M</span>
                                            <span className="text-[8px] text-blue-400 uppercase">True Negative</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                    </motion.div>
                </AnimatePresence>
            </div>
        </div>
    );
}
