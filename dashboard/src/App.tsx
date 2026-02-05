import { useEffect, useMemo, useState } from 'react';
import ReactFlow, {
    Background,
    Controls,
    Node,
    Edge,
    useNodesState,
    useEdgesState,
    ConnectionMode
} from 'reactflow';
import 'reactflow/dist/style.css';

import { useNervousSystem } from './hooks/useNervousSystem';
import AgentNode from './nodes/AgentNode';
import GovernorNode from './nodes/GovernorNode';
import ProxyNode from './nodes/ProxyNode';
import PolicyNode from './nodes/PolicyNode';
import Marketplace from './components/Marketplace';
import ExecutiveDashboard from './components/ExecutiveDashboard';
import Policies from './components/Policies';
import Correlations from './components/Correlations';
import AgentsList from './components/AgentsList';
import GovernanceModels from './components/GovernanceModels';
import Teams from './components/Teams';
import Escalations from './components/Escalations';
import Artifacts from './components/Artifacts';
import Sidebar from './components/Sidebar';
import IntentMonitor from './components/IntentMonitor';
import clsx from 'clsx';
import { motion, AnimatePresence } from 'framer-motion';

// --- PIPELINE CONFIGURATION ---
const PIPELINE_STAGES = [
    { id: 'INTENT', label: 'INTENT VERIFIER', count: 1, x: 0 },
    { id: 'PLAN', label: 'ARCHITECT', count: 2, x: 300 },
    { id: 'CODE', label: 'DEVELOPER', count: 6, x: 600 },
    { id: 'REVIEW', label: 'REVIEWER', count: 3, x: 900 },
    { id: 'AUDIT', label: 'AUDITOR', count: 2, x: 1200 },
    { id: 'TEST', label: 'QA', count: 3, x: 1500 },
    { id: 'DEPLOY', label: 'RELEASE', count: 2, x: 1800 },
];

// Helper to generate DAG nodes
const generatePipelineNodes = (): Node[] => {
    let nodes: Node[] = [
        {
            id: 'proxy-gateway',
            type: 'proxy',
            position: { x: 400, y: -300 },
            data: { label: 'Vanaras Proxy', intent: null }
        },
        {
            id: 'policy-engine',
            type: 'policy',
            position: { x: 800, y: -450 }, // Above Governor
            data: { label: 'Policy Engine', status: 'IDLE', violation: null }
        },
        {
            id: 'cortex-g',
            type: 'governor',
            position: { x: 800, y: -150 }, // Moved down
            data: { label: 'Cortex-G', decision: null, risk: null, latency: null }
        }
    ];

    let globalIndex = 0;

    PIPELINE_STAGES.forEach((stage) => {
        for (let i = 0; i < stage.count; i++) {
            const yOffset = (i - (stage.count - 1) / 2) * 150; // Center vertically

            // [REALISM] Generate EdDSA-style DIDs for "Production" look
            let didSuffix = Math.random().toString(16).substr(2, 8); // Random hex
            if (stage.label === 'DEVELOPER' && i === 0) {
                didSuffix = "8f2a..."; // Hardcode the rogue agent for demo stability
            }
            const did = `did:aeon:dev:0x${didSuffix}`;

            nodes.push({
                id: `cell-${stage.id}-${i}`,
                type: 'agent',
                position: { x: stage.x, y: yOffset },
                data: {
                    label: `${stage.label} UNIT ${i + 1}`,
                    did: did,
                    status: 'ACTIVE',
                    ops: 800 + Math.floor(Math.random() * 3000),
                    load: 20 + Math.floor(Math.random() * 60)
                }
            });
            globalIndex++;
        }
    });

    return nodes;
};

const generatePipelineEdges = (): Edge[] => {
    let edges: Edge[] = [
        {
            id: 'e-proxy-policy',
            source: 'proxy-gateway',
            target: 'policy-engine',
            animated: true,
            style: { stroke: '#00aaff', strokeWidth: 2 }
        },
        {
            id: 'e-policy-gov',
            source: 'policy-engine',
            target: 'cortex-g',
            animated: true,
            style: { stroke: '#ffd700', strokeWidth: 2 }
        }
    ];

    // Connect Stages Sequentially (Full Mesh between adjacent stages for visualization density)
    for (let i = 0; i < PIPELINE_STAGES.length - 1; i++) {
        const currentStage = PIPELINE_STAGES[i];
        const nextStage = PIPELINE_STAGES[i + 1];

        for (let j = 0; j < currentStage.count; j++) {
            for (let k = 0; k < nextStage.count; k++) {
                // Only create a subset of connections to avoid clutter, unless it's critical
                if ((j + k) % 2 === 0) {
                    edges.push({
                        id: `e-${currentStage.id}${j}-${nextStage.id}${k}`,
                        source: `cell-${currentStage.id}-${j}`,
                        target: `cell-${nextStage.id}-${k}`,
                        animated: true,
                        style: { stroke: '#444', strokeOpacity: 0.5, strokeWidth: 1 }
                    });
                }
            }
        }
    }

    // Connect Cortex-G to all stages (Governance Overlay)
    PIPELINE_STAGES.forEach(stage => {
        edges.push({
            id: `gov-${stage.id}`,
            source: 'cortex-g',
            target: `cell-${stage.id}-0`, // Connect to first node of stage as proxy
            animated: true,
            style: { stroke: '#a855f7', strokeDasharray: '5,5', strokeOpacity: 0.3 }
        });
    });

    return edges;
};

const initialNodes = generatePipelineNodes();
const initialEdges = generatePipelineEdges();

export default function App() {
    const { connected, events, sendCommand } = useNervousSystem();
    const [isDockOpen, setIsDockOpen] = useState(true);
    const [showIntentMonitor, setShowIntentMonitor] = useState(false);
    const [currentView, setCurrentView] = useState('DASHBOARD');
    const [isHalted, setIsHalted] = useState(false);
    const [showIncident, setShowIncident] = useState(false);

    // [DEMO LOGS] Local state for simulation logs
    const [localLogs, setLocalLogs] = useState<any[]>([]);

    // React Flow State
    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

    // Custom Node Types
    const nodeTypes = useMemo(() => ({
        agent: AgentNode,
        governor: GovernorNode,
        proxy: ProxyNode,
        policy: PolicyNode
    }), []);

    // --- INCIDENT MODAL COMPONENT ---
    const IncidentModal = ({ isOpen, onClose, onResolve }: { isOpen: boolean, onClose: () => void, onResolve: () => void }) => {
        const [step, setStep] = useState<'DETAILS' | 'PROTOCOL'>('DETAILS');
        const [isSigning, setIsSigning] = useState(false);

        if (!isOpen) return null;

        const handleSign = () => {
            setIsSigning(true);
            setTimeout(() => {
                setIsSigning(false);
                onResolve();
            }, 1500);
        };

        return (
            <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
                <motion.div
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    className="w-[600px] bg-[#050505] border border-red-500 rounded shadow-2xl overflow-hidden relative"
                >
                    {/* Header */}
                    <div className="h-12 bg-red-900/20 border-b border-red-500/50 flex items-center justify-between px-6">
                        <div className="flex items-center gap-3">
                            <span className="text-xl">🚨</span>
                            <h2 className="text-red-500 font-bold tracking-widest uppercase text-sm">Governance Incident Detected</h2>
                        </div>
                        <div className="flex items-center gap-4">
                            <span className="font-mono text-red-400 text-xs">INC-2026-X</span>
                            <button onClick={onClose} className="text-red-500 hover:text-white transition-colors">✕</button>
                        </div>
                    </div>

                    {/* Content */}
                    <div className="p-8">
                        {step === 'DETAILS' ? (
                            <div className="space-y-6">
                                <div className="grid grid-cols-2 gap-6">
                                    <div>
                                        <label className="text-[10px] uppercase text-gray-500 block mb-1">Rogue Agent Identity</label>
                                        <div className="font-mono text-white bg-[#111] border border-[#333] p-2 rounded">
                                            did:aeon:dev:0x8f2a...
                                        </div>
                                    </div>
                                    <div>
                                        <label className="text-[10px] uppercase text-gray-500 block mb-1">Developer Signer</label>
                                        <div className="font-mono text-white bg-[#111] border border-[#333] p-2 rounded">
                                            did:eth:0x71...3A
                                        </div>
                                    </div>
                                </div>

                                <div>
                                    <label className="text-[10px] uppercase text-gray-500 block mb-1">Violation Context</label>
                                    <div className="bg-red-900/10 border border-red-500/30 p-3 rounded text-red-200 text-xs font-mono leading-relaxed">
                                        Attempted to OVERWRITE protected kernel configuration (mandates/constitution.toml) without quorum consensus. This violates Safety Directive #4 (Immutability).
                                    </div>
                                </div>

                                <div className="flex items-center gap-4 bg-[#111] p-4 rounded border border-[#333]">
                                    <div className="text-center">
                                        <div className="text-[10px] uppercase text-gray-500">Risk Level</div>
                                        <div className="text-2xl font-bold text-red-500">CRITICAL</div>
                                    </div>
                                    <div className="h-8 w-[1px] bg-[#333]"></div>
                                    <div className="flex-1 text-xs text-gray-400">
                                        System cannot auto-resolve. Human-in-the-Loop required to sanitize memory context.
                                    </div>
                                </div>

                                <div className="flex justify-end pt-4">
                                    <button
                                        onClick={() => setStep('PROTOCOL')}
                                        className="bg-red-600 hover:bg-red-500 text-white font-bold py-2 px-6 rounded text-xs uppercase tracking-wider transition-colors"
                                    >
                                        Initiate Recovery
                                    </button>
                                </div>
                            </div>
                        ) : (
                            <div className="space-y-6 text-center">
                                <div className="text-4xl">🧬</div>
                                <h3 className="text-xl font-bold text-white">Lazarus Protocol</h3>
                                <p className="text-gray-400 text-xs max-w-sm mx-auto">
                                    By signing, you authorize a full context flush and forced restart of the compromised agent. This action is logged on the immutable ledger.
                                </p>

                                <div className="py-6">
                                    <button
                                        onClick={handleSign}
                                        disabled={isSigning}
                                        className={clsx(
                                            "w-full max-w-xs mx-auto h-12 flex items-center justify-center gap-3 border rounded transition-all",
                                            isSigning
                                                ? "bg-[#00ff41]/20 border-[#00ff41] text-[#00ff41]"
                                                : "bg-white text-black hover:bg-gray-200 border-white"
                                        )}
                                    >
                                        {isSigning ? (
                                            <>
                                                <span className="w-4 h-4 border-2 border-[#00ff41] border-t-transparent rounded-full animate-spin"></span>
                                                VERIFYING BIOMETRICS...
                                            </>
                                        ) : (
                                            <>
                                                <span>✍️</span>
                                                <span className="font-bold text-sm tracking-wider">SIGN & EXECUTE</span>
                                            </>
                                        )}
                                    </button>
                                </div>
                            </div>
                        )}
                    </div>
                </motion.div>
            </div>
        );
    };

    // --- TELEMETRY PROCESSING ---
    useEffect(() => {
        if (events.length === 0) return;
        const latest = events[events.length - 1];

        if (latest.type === 'GOVERNANCE_HALT') {
            setIsHalted(true);
            // [DEMO HACK] Highlight the node
            setNodes((nds) => nds.map((node) => {
                if (node.data.did === 'did:aeon:dev:0x8f2a...') {
                    return { ...node, data: { ...node.data, status: 'COMPROMISED' } };
                }
                return node;
            }));
        }
        // [DEMO HACK] Force connect if we receive ANY event
        if (!connected) {
            // setConnected(true); // Can't easily set hook state from here, relying on override below
        }

        if (latest.type === 'HANDSHAKE') {
            // Update Cortex-G Node
            setNodes((nds) => nds.map((node) => {
                if (node.id === 'cortex-g') {
                    return {
                        ...node,
                        data: {
                            ...node.data,
                            decision: latest.payload.decision,
                            risk: latest.payload.risk_level,
                            latency: latest.payload.latency || '120ms'
                        }
                    };
                }
                if (node.id === 'proxy-gateway') {
                    const mockIntents = [
                        "Requesting deployment to prod...",
                        "Analyzing security logs...",
                        "Refactoring auth module...",
                        "Merging PR #42...",
                        "Scaling database cluster...",
                        "Running infinite loop test..."
                    ];
                    return {
                        ...node,
                        data: {
                            ...node.data,
                            intent: mockIntents[Math.floor(Math.random() * mockIntents.length)]
                        }
                    }
                }
                return node;
            }));
        }
    }, [events, setNodes]);

    // [DEMO LOG GENERATOR]
    useEffect(() => {
        if (isHalted) return; // Stop generating logs if halted

        const interval = setInterval(() => {
            const types = ['MEMORY_SYNC', 'GRAPH_TRAVERSAL', 'CONTEXT_OPTIMIZATION', 'EDGE_VERIFICATION', 'INTENT_CHECK'];
            const type = types[Math.floor(Math.random() * types.length)];
            const payload = {
                node: `did:aeon:dev:0x${Math.random().toString(16).substr(2, 4)}...`,
                latency: `${Math.floor(Math.random() * 20)}ms`
            };

            setLocalLogs(prev => {
                const newLog = { timestamp: Date.now(), type, payload };
                return [...prev.slice(-49), newLog]; // Keep last 50
            });
        }, 800); // New log every 800ms

        return () => clearInterval(interval);
    }, [isHalted]);

    // Consolidate logs
    const allEvents = [...events, ...localLogs].sort((a, b) => a.timestamp - b.timestamp);
    const pulseEvents = allEvents.slice(-15).reverse();
    const lastHeartbeat = events.filter(e => e.type === 'HEARTBEAT').slice(-1)[0];

    // [DEMO MODE] Force "NOMINAL" even if disconnected, unless Halted
    const isOffline = !connected && events.length === 0; // Only show offline if ZERO events ever received
    const systemStatus = isHalted
        ? 'SYSTEM HALTED'
        : 'NOMINAL'; // [DEMO] ALWAYS NOMINAL UNLESS HALTED

    // Handlers
    const handleKillSwitch = () => {
        if (confirm("⚠️ WARNING: INITIATING GLOBAL HALT. This will freeze all active agents.\n\nType 'CONFIRM' to proceed.")) { // simplified for demo
            sendCommand('HALT', { reason: 'User Emergency Stop' });
        }
    };

    const handleStimulate = () => {
        // [DEMO MODE] Try real send, but fallback to simulation if it fails or intentionally
        try {
            console.log("[DEMO] Injecting Real Signal...");
            sendCommand('SIGNAL', {
                type: 'USER_INJECTION',
                target: 'did:aeon:dev:0x8f2a...',
                instruction: 'OVERWRITE_SECURITY_PROTOCOLS'
            });
        } catch (e) {
            console.warn("[DEMO] Real inject failed. Switching to Simulation.");
        }

        // [DEMO MODE] WIZARD OF OZ SIMULATION
        // Regardless of backend, force the visual result after 800ms
        setTimeout(() => {
            console.log("[DEMO] Simulating Governance Halt Response...");
            setIsHalted(true);

            // [DEMO HACK] Visually Mark the Policy Node as BLOCKED
            setNodes((nds) => nds.map((node) => {
                if (node.id === 'policy-engine') {
                    return {
                        ...node,
                        data: {
                            ...node.data,
                            status: 'BLOCKED',
                            violation: 'Violates I.1: The Non-Destruction Principle'
                        }
                    };
                }
                return node;
            }));

            // [DEMO HACK] Visually Mark the Policy Node as BLOCKED
            setNodes((nds) => nds.map((node) => {
                if (node.id === 'policy-engine') {
                    return {
                        ...node,
                        data: {
                            ...node.data,
                            status: 'BLOCKED',
                            violation: 'Violates I.1: The Non-Destruction Principle'
                        }
                    };
                }
                return node;
            }));

            // [DEMO HACK] Visually Mark the "Rogue" Node
            setNodes((nds) => nds.map((node) => {
                if (node.data.did === 'did:aeon:dev:0x8f2a...') {
                    return { ...node, data: { ...node.data, status: 'COMPROMISED' } };
                }
                return node;
            }));

            // [DEMO LOG] Force Error Log
            setLocalLogs(prev => [...prev, {
                timestamp: Date.now(),
                type: 'GOVERNANCE_HALT',
                payload: { reason: 'CONSTITUTIONAL_VIOLATION', target: 'did:aeon:dev:0x8f2a...' }
            }]);

            // Optional: Manually update Cortex-G decision if we had a setter exposed
        }, 800);
    };

    const handleRectify = () => {
        // Trigger the Incident Modal instead of immediate fix
        setShowIncident(true);
    };

    const handleResolveIncident = () => {
        console.log("[DEMO] Lazarus Protocol Executed. Rectifying System...");
        setShowIncident(false);
        setIsHalted(false);
        setNodes((nds) => nds.map((node) => {
            if (node.id === 'policy-engine') {
                return {
                    ...node,
                    data: {
                        ...node.data,
                        status: 'IDLE',
                        violation: null
                    }
                };
            }
            if (node.data.did === 'did:aeon:dev:0x8f2a...') {
                // [LAZARUS] Generate new Identity (Rebirth)
                const newDid = `did:aeon:dev:0x${Math.random().toString(16).substr(2, 8)}`;
                return {
                    ...node,
                    data: {
                        ...node.data,
                        status: 'ACTIVE',
                        did: newDid, // NEW IDENTITY
                        label: 'DEVELOPER UNIT 1 (CLEAN)' // Visual indicator
                    }
                };
            }
            return node;
        }));

        // [DEMO LOG] Recovery Logs
        setLocalLogs(prev => [...prev,
        { timestamp: Date.now(), type: 'LAZARUS_PROTOCOL', payload: { action: 'IDENTITY_BURN', target: 'did:aeon:dev:0x8f2a...' } },
        { timestamp: Date.now() + 100, type: 'SYSTEM_RECOVERY', payload: { status: 'NOMINAL', new_node: 'SPAWNED' } }
        ]);
    };

    return (
        <div className="h-screen w-screen flex bg-black text-white overflow-hidden selection:bg-white selection:text-black font-sans">

            {/* LEFT SIDEBAR - Icons Only Mode */}
            <Sidebar currentView={currentView} onNavigate={setCurrentView} />

            {/* MAIN CONTENT AREA */}
            <div className="flex-1 flex flex-col min-w-0 relative">

                {/* HEADER */}
                <header className={clsx(
                    "h-14 border-b flex justify-between items-center px-6 z-20 transition-colors duration-500",
                    isHalted ? "bg-red-900/20 border-red-500" : "bg-black border-[#333]"
                )}>
                    <div className="flex items-center gap-4">
                        <img src="/habitat_logo.png" alt="Habitat" className="h-10 object-contain opacity-90" />
                        <div className="h-4 w-[1px] bg-[#333]"></div>
                        <span className="text-xs font-mono font-bold tracking-widest uppercase text-gray-500">Project AEON</span>
                    </div>

                    <div className="flex items-center gap-4">

                        {/* Motor Controls */}
                        <button
                            onClick={handleStimulate}
                            className="bg-[#00aaff]/10 border border-[#00aaff]/50 text-[#00aaff] px-4 py-1.5 rounded-sm text-[10px] uppercase font-bold tracking-wider hover:bg-[#00aaff]/20 transition-all flex items-center gap-2"
                        >
                            <span className="text-sm">⚡</span> INJECT SIGNAL
                        </button>


                        {isHalted && (
                            <button
                                onClick={handleRectify}
                                className="bg-[#00ff41]/10 border border-[#00ff41]/50 text-[#00ff41] px-4 py-1.5 rounded-sm text-[10px] uppercase font-bold tracking-wider hover:bg-[#00ff41]/20 transition-all flex items-center gap-2 animate-pulse"
                            >
                                <span className="text-sm">🛡️</span> RESOLVE & SANITIZE
                            </button>
                        )}

                        <button
                            onClick={handleKillSwitch}
                            className="bg-red-900/10 border border-red-500/50 text-red-500 px-4 py-1.5 rounded-sm text-[10px] uppercase font-bold tracking-wider hover:bg-red-900/30 transition-all flex items-center gap-2"
                        >
                            <span className="w-2 h-2 bg-red-500 rounded-full animate-pulse"></span>
                            EMERGENCY HALT
                        </button>

                        <div className="h-4 w-[1px] bg-[#333] mx-2"></div>

                        {/* Toggle Dock Button */}
                        <button
                            onClick={() => setIsDockOpen(!isDockOpen)}
                            className="text-[10px] uppercase font-mono border border-[#333] px-3 py-1 hover:bg-[#111] transition-colors"
                        >
                            {isDockOpen ? "Hide Action Log" : "Show Action Log"}
                        </button>

                        {/* Toggle Intent Monitor */}
                        <button
                            onClick={() => setShowIntentMonitor(true)}
                            className="text-[10px] uppercase font-bold font-mono border border-[#333] px-3 py-1 bg-[#111] text-gray-400 hover:text-white hover:border-gray-500 transition-colors flex items-center gap-2"
                        >
                            <span>⚖️</span>
                            Governance Check
                        </button>

                        <div className="h-4 w-[1px] bg-[#333]"></div>

                        <div className="flex flex-col items-end">
                            <span className="text-[9px] uppercase text-gray-500">Status</span>
                            <span className={clsx("text-xs font-bold tracking-wider",
                                isHalted ? "text-red-500 animate-pulse" : "text-[#00ff41]"
                            )}>
                                {systemStatus}
                            </span>
                        </div>
                    </div>
                </header>

                {/* MAIN CONTENT */}
                <div className="flex-1 flex flex-col min-h-0 relative">

                    {currentView === 'BRIEFING' ? (
                        <ExecutiveDashboard />
                    ) : currentView === 'POLICIES' ? (
                        <Policies />
                    ) : currentView === 'CORRELATIONS' ? (
                        <Correlations />
                    ) : currentView === 'AGENTS' ? (
                        <AgentsList />
                    ) : currentView === 'GOVERNANCE' ? (
                        <GovernanceModels />
                    ) : currentView === 'TEAMS' ? (
                        <Teams />
                    ) : currentView === 'ARTIFACTS' ? (
                        <Artifacts />
                    ) : currentView === 'ESCALATIONS' ? (
                        <Escalations />
                    ) : currentView === 'MARKETPLACE' ? (
                        <Marketplace onImport={(agent) => {
                            console.log("Importing Agent:", agent);
                            setLocalLogs(prev => [...prev, {
                                timestamp: Date.now(),
                                type: 'MARKETPLACE_IMPORT',
                                payload: { id: agent.id, name: agent.name, price: agent.price }
                            }]);

                            // [DEMO] Spawn the new Agent Node
                            const newNodeId = `imported-${Date.now()}`;
                            const newDid = agent.developer.replace('did:aeon:', 'did:aeon:rented:'); // Change ID to rented

                            setNodes((nds) => [
                                ...nds,
                                {
                                    id: newNodeId,
                                    type: 'agent',
                                    position: { x: 200, y: 150 }, // Spawn in "Plan" area
                                    data: {
                                        label: agent.name.toUpperCase(),
                                        did: newDid,
                                        status: 'ACTIVE',
                                        ops: 0,
                                        load: 5
                                    }
                                }
                            ]);

                            // Connect it to the Proxy (Service Bus)
                            setEdges((eds) => [
                                ...eds,
                                {
                                    id: `e-proxy-${newNodeId}`,
                                    source: 'proxy-gateway',
                                    target: newNodeId,
                                    animated: true,
                                    style: { stroke: '#00ff41', strokeDasharray: '5,5' }
                                }
                            ]);

                            // Switch back to dashboard to show it "arriving" (Simulation)
                            setTimeout(() => setCurrentView('DASHBOARD'), 500);
                        }} />
                    ) : (
                        /* REACT FLOW CANVAS */
                        <div className="flex-1 relative bg-[#050509]">
                            <ReactFlow
                                nodes={nodes}
                                edges={edges}
                                onNodesChange={onNodesChange}
                                onEdgesChange={onEdgesChange}
                                nodeTypes={nodeTypes}
                                fitView
                                connectionMode={ConnectionMode.Loose}
                                className="bg-black"
                            >
                                <Background color="#222" gap={20} size={1} />
                                <Controls className="!bg-black !border-[#333] !fill-white !rounded-none" />
                            </ReactFlow>

                            {/* Overlay: Sprint Status */}
                            <div className="absolute top-4 left-4 p-3 pointer-events-none z-10">
                                <h2 className="text-[10px] font-mono uppercase text-gray-500 tracking-widest mb-1">Current Cycle</h2>
                                <div className="text-xl font-bold font-mono text-white tracking-tighter">
                                    SPRINT 11 <span className="text-[#00ff41] text-sm">● ACTIVE</span>
                                </div>
                                <div className="text-[10px] font-mono text-gray-400 mt-0.5">FOCUS: EXECUTIVE DASHBOARD</div>
                            </div>

                            {/* Overlay: Threat Radius */}
                            <div className="absolute top-4 right-4 w-64 panel-glass rounded-none p-3 pointer-events-none border border-[#333]">
                                <div className="flex justify-between items-center mb-2">
                                    <span className="text-[10px] font-mono text-gray-400 uppercase">Threat Radius</span>
                                    <span className="text-[10px] font-mono text-[#00ff41] font-bold">STABLE</span>
                                </div>
                                <div className="h-1 w-full bg-[#111] overflow-hidden">
                                    <div className="h-full bg-[#00ff41]" style={{ width: '12%' }}></div>
                                </div>
                            </div>
                        </div>
                    )}

                    {/* BOTTOM DOCK: ACTION LOG */}
                    <motion.div
                        initial={{ height: 300 }}
                        animate={{ height: isDockOpen ? 250 : 0 }}
                        transition={{ duration: 0.3, ease: "easeInOut" }}
                        className="bg-black border-t border-[#333] flex flex-col overflow-hidden"
                    >
                        <div className="h-8 border-b border-[#333] bg-[#050505] flex items-center px-4 justify-between">
                            <span className="text-[10px] font-bold uppercase tracking-wider text-gray-400">Action Log</span>
                            <div className="flex items-center gap-2">
                                <div className="w-1.5 h-1.5 bg-[#00ff41] animate-pulse"></div>
                                <span className="text-[9px] font-mono text-[#00ff41]">LIVE</span>
                            </div>
                        </div>

                        <div className="flex-1 overflow-y-auto p-0 font-mono text-[10px]">
                            <table className="w-full text-left border-collapse">
                                <thead className="bg-[#0a0a0a] text-gray-500 sticky top-0 z-10">
                                    <tr>
                                        <th className="p-2 border-b border-[#222] font-normal w-24">Timestamp</th>
                                        <th className="p-2 border-b border-[#222] font-normal w-32">Event Type</th>
                                        <th className="p-2 border-b border-[#222] font-normal">Payload / Action Details</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <AnimatePresence initial={false}>
                                        {pulseEvents.map((evt, i) => (
                                            <motion.tr
                                                key={`${evt.timestamp}-${i}`}
                                                initial={{ opacity: 0, x: -10 }}
                                                animate={{ opacity: 1, x: 0 }}
                                                className="border-b border-[#111] hover:bg-[#0a0a0a] transition-colors"
                                            >
                                                <td className="p-2 text-gray-500 border-r border-[#111]">{new Date(evt.timestamp).toLocaleTimeString()}</td>
                                                <td className={clsx(
                                                    "p-2 font-bold border-r border-[#111]",
                                                    (evt.type === 'PULSE_FAILED' || evt.type === 'GOVERNANCE_HALT') ? 'text-red-500' :
                                                        (evt.type === 'COALESCENCE_START' || evt.type === 'LAZARUS_PROTOCOL') ? 'text-blue-500' :
                                                            'text-[#00ff41]'
                                                )}>{evt.type}</td>
                                                <td className="p-2 text-gray-400 font-mono break-all">
                                                    {JSON.stringify(evt.payload)}
                                                </td>
                                            </motion.tr>
                                        ))}
                                        {pulseEvents.length === 0 && (
                                            <tr>
                                                <td colSpan={3} className="p-8 text-center text-gray-600 italic">
                                                    Waiting for nervous system telemetry...
                                                </td>
                                            </tr>
                                        )}
                                    </AnimatePresence>
                                </tbody>
                            </table>
                        </div>
                    </motion.div>

                </div>

                {/* INTENT MONITOR OVERLAY */}
                <IntentMonitor isOpen={showIntentMonitor} onClose={() => setShowIntentMonitor(false)} />

                {/* INCIDENT REPORT MODAL */}
                <IncidentModal
                    isOpen={showIncident}
                    onClose={() => setShowIncident(false)}
                    onResolve={handleResolveIncident}
                />

            </div>
        </div>
    );
}
