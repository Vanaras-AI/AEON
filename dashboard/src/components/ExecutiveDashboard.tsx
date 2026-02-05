import { motion } from 'framer-motion';
import clsx from 'clsx';

// --- MOCK DATA ---

const INBOX_ITEMS = [
    { id: 1, source: 'DeepResearch', subject: 'Competitor Analysis Complete', time: '10:42 AM', summary: 'Identified 3 new entrants in the WASM agent space. Report generated.', status: 'UNREAD' },
    { id: 2, source: 'SecurityBot', subject: 'Vulnerability Detected', time: '09:15 AM', summary: 'CVE-2026-9921 found in dependency tree. Auto-patch drafted.', status: 'URGENT' },
    { id: 3, source: 'CFO-Agent', subject: 'Budget Alert', time: 'Yesterday', summary: 'Compute spend projected to exceed monthly cap by 12%.', status: 'READ' },
];

const CALENDAR_ITEMS = [
    { id: 1, time: '11:00 AM', title: 'Series A Due Diligence', type: 'MEETING' },
    { id: 2, time: '02:00 PM', title: 'Start Sprint 11 Build', type: 'FACTORY' },
    { id: 3, time: '04:30 PM', title: 'Governance Review Board', type: 'DECISION' },
];

const DECISIONS = [
    { id: 1, title: 'Network Egress Request', requester: 'Developer Unit 4', context: 'Access to pypi.org for package install.', risk: 'MEDIUM' },
    { id: 2, title: 'Deploy to Production', requester: 'Release Manager', context: 'Build #8821 passed all unit tests.', risk: 'HIGH' },
];

const FACTORY_UPDATES = [
    { id: 1, repo: 'aeon/core', status: 'BUILDING', branch: 'feat/wasm-isolation', progress: 78 },
    { id: 2, repo: 'aeon/dashboard', status: 'PASSED', branch: 'main', progress: 100 },
];

// --- COMPONENTS ---

const Widget = ({ title, children, className }: { title: string, children: React.ReactNode, className?: string }) => (
    <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className={clsx("bg-[#0a0a0a] border border-[#333] p-5 flex flex-col hover:border-[#444] transition-colors", className)}
    >
        <div className="flex justify-between items-center mb-4 border-b border-[#222] pb-2">
            <h3 className="text-xs font-bold uppercase tracking-widest text-gray-400">{title}</h3>
            <span className="text-[10px] text-[#00ff41]">● LIVE</span>
        </div>
        <div className="flex-1 overflow-y-auto">
            {children}
        </div>
    </motion.div>
);

export default function ExecutiveDashboard() {
    return (
        <div className="flex-1 bg-black text-white p-8 font-sans h-full overflow-hidden flex flex-col relative">

            {/* Background Grid */}
            <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-5 pointer-events-none"></div>

            {/* Header */}
            <div className="mb-8 z-10 flex justify-between items-end">
                <div>
                    <h1 className="text-3xl font-bold tracking-tight uppercase">Command Center</h1>
                    <p className="text-gray-500 font-mono text-xs mt-1">EXECUTIVE BRIEFING // <span className="text-[#00ff41]">{new Date().toLocaleDateString()}</span></p>
                </div>
            </div>

            {/* Dashboard Grid */}
            <div className="flex-1 grid grid-cols-3 grid-rows-2 gap-6 z-10 min-h-0">

                {/* 1. AGENT INBOX (Large) */}
                <Widget title="Agent Inbox" className="col-span-2 row-span-1">
                    <div className="space-y-1">
                        {INBOX_ITEMS.map((msg) => (
                            <div key={msg.id} className="group flex items-start gap-4 p-3 hover:bg-[#111] border rounded border-transparent hover:border-[#333] cursor-pointer transition-all">
                                <div className={clsx(
                                    "w-2 h-2 rounded-full mt-1.5",
                                    msg.status === 'URGENT' ? 'bg-red-500 animate-pulse' :
                                        msg.status === 'UNREAD' ? 'bg-[#00ff41]' : 'bg-gray-600'
                                )}></div>
                                <div className="flex-1 min-w-0">
                                    <div className="flex justify-between items-baseline mb-0.5">
                                        <span className="font-bold text-sm text-white">{msg.subject}</span>
                                        <span className="text-[10px] font-mono text-gray-500">{msg.time}</span>
                                    </div>
                                    <div className="text-xs text-blue-400 font-mono mb-1">{msg.source}</div>
                                    <p className="text-xs text-gray-400 truncate">{msg.summary}</p>
                                </div>
                            </div>
                        ))}
                    </div>
                </Widget>

                {/* 2. CALENDAR (Side) */}
                <Widget title="Schedule" className="col-span-1 row-span-1">
                    <div className="space-y-4">
                        {CALENDAR_ITEMS.map((item) => (
                            <div key={item.id} className="flex gap-4 items-center">
                                <div className="w-16 text-right font-mono text-xs text-gray-500">{item.time}</div>
                                <div className="h-full w-[1px] bg-[#333] h-8"></div>
                                <div>
                                    <div className="text-sm font-bold">{item.title}</div>
                                    <div className="text-[10px] uppercase text-gray-600 tracking-wider">{item.type}</div>
                                </div>
                            </div>
                        ))}
                    </div>
                </Widget>

                {/* 3. DECISIONS PENDING (Bottom Left) */}
                <Widget title="Pending Approvals" className="col-span-1 row-span-1">
                    <div className="space-y-4">
                        {DECISIONS.map((d) => (
                            <div key={d.id} className="bg-[#111] p-3 rounded border border-[#222]">
                                <div className="flex justify-between items-start mb-2">
                                    <span className="text-xs font-bold text-white">{d.title}</span>
                                    <span className={clsx("text-[9px] px-1.5 py-0.5 rounded font-bold", d.risk === 'HIGH' ? 'bg-red-900 text-red-100' : 'bg-yellow-900 text-yellow-100')}>{d.risk}</span>
                                </div>
                                <p className="text-[10px] text-gray-400 mb-3 leading-relaxed">{d.context}</p>
                                <div className="flex gap-2">
                                    <button className="flex-1 bg-[#00ff41]/10 text-[#00ff41] hover:bg-[#00ff41]/20 py-1 text-[10px] font-bold uppercase rounded transition-colors">Approve</button>
                                    <button className="flex-1 bg-red-900/10 text-red-500 hover:bg-red-900/20 py-1 text-[10px] font-bold uppercase rounded transition-colors">Deny</button>
                                </div>
                            </div>
                        ))}
                    </div>
                </Widget>

                {/* 4. FACTORY STATUS (Bottom Right) */}
                <Widget title="Software Factory" className="col-span-2 row-span-1">
                    <div className="grid grid-cols-2 gap-4">
                        {FACTORY_UPDATES.map((f) => (
                            <div key={f.id} className="bg-[#111] p-4 border border-[#333] flex flex-col justify-between">
                                <div>
                                    <div className="flex justify-between mb-2">
                                        <span className="font-mono text-xs text-gray-300">{f.repo}</span>
                                        <span className={clsx("text-[10px] font-bold", f.status === 'BUILDING' ? 'text-blue-400 animate-pulse' : 'text-[#00ff41]')}>{f.status}</span>
                                    </div>
                                    <div className="text-[10px] text-gray-500 mb-4">{f.branch}</div>
                                </div>
                                <div>
                                    <div className="flex justify-between text-[10px] mb-1 text-gray-400">
                                        <span>Progress</span>
                                        <span>{f.progress}%</span>
                                    </div>
                                    <div className="h-1 bg-[#222] w-full overflow-hidden rounded-full">
                                        <div className="h-full bg-blue-500 transition-all duration-1000" style={{ width: `${f.progress}%` }}></div>
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                </Widget>
            </div>
        </div>
    );
}
