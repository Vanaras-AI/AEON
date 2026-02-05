import { memo } from 'react';
import clsx from 'clsx';

// Minimal Icon Component for consistent 1.5px stroke "Technical" look
const Icon = ({ path }: { path: string }) => (
    <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.5"
        strokeLinecap="square"
        strokeLinejoin="miter"
        className="w-5 h-5"
    >
        <path d={path} />
    </svg>
);

const NavItem = ({ path, label, id, currentView, onNavigate, alert = false }: { path: string, label: string, id: string, currentView: string, onNavigate: (id: string) => void, alert?: boolean }) => (
    <div
        onClick={() => onNavigate(id)}
        className={clsx(
            "flex items-center gap-3 px-4 py-2.5 cursor-pointer transition-all border-l-2 group relative",
            currentView === id
                ? "bg-[#111] text-white border-white"
                : "border-transparent text-gray-500 hover:text-gray-300 hover:bg-[#0a0a0a]"
        )}>
        <Icon path={path} />
        <span className="text-[10px] font-mono uppercase tracking-widest font-bold pt-0.5">{label}</span>
        {alert && <div className="absolute right-2 w-1 h-1 bg-red-500 rounded-full animate-pulse"></div>}
    </div>
);

interface SidebarProps {
    currentView: string;
    onNavigate: (view: string) => void;
}

export default memo(({ currentView, onNavigate }: SidebarProps) => {
    return (
        <div className="w-64 h-full bg-black border-r border-[#333] flex flex-col z-30">

            {/* User Profile - Digital ID Card Style */}
            <div className="p-4 border-b border-[#333] flex items-center gap-3 bg-[#050505]">
                <div className="w-8 h-8 bg-[#111] border border-[#333] flex items-center justify-center text-gray-400">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" className="w-4 h-4">
                        <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
                        <circle cx="12" cy="7" r="4" />
                    </svg>
                </div>
                <div className="flex flex-col">
                    <div className="text-[10px] font-bold text-white uppercase tracking-wider leading-none mb-1">Lakshmi Vijay</div>
                    <div className="text-[8px] font-mono text-[#00ff41] leading-none">DID:AEON:0x9686L1v2p2026015e2</div>
                </div>
            </div>

            {/* Core Modules */}
            <div className="flex-1 py-4 overflow-y-auto space-y-0.5">
                <div className="px-4 mb-2 mt-2 text-[8px] font-mono uppercase text-[#444] tracking-widest font-bold">Platform</div>

                {/* Briefing: Command Center */}
                <NavItem id="BRIEFING" currentView={currentView} onNavigate={onNavigate} path="M3 12h18M3 6h18M3 18h18" label="Briefing" />

                {/* Dashboard: Layout Grid -> LIVE OPS */}
                <NavItem id="DASHBOARD" currentView={currentView} onNavigate={onNavigate} path="M3 3h7v7H3zM14 3h7v7h-7zM14 14h7v7h-7zM3 14h7v7H3z" label="Live Ops" />

                {/* Marketplace: Grid/Store */}
                <NavItem id="MARKETPLACE" currentView={currentView} onNavigate={onNavigate} path="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" label="Exchange" alert={true} />

                {/* Policies: File Shield */}
                <NavItem id="POLICIES" currentView={currentView} onNavigate={onNavigate} path="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" label="Policies" />

                {/* Correlations: Network */}
                <NavItem id="CORRELATIONS" currentView={currentView} onNavigate={onNavigate} path="M18 6a3 3 0 1 0-6 0 3 3 0 0 0 6 0zm-6 12a3 3 0 1 0-6 0 3 3 0 0 0 6 0zm12 0a3 3 0 1 0-6 0 3 3 0 0 0 6 0zM12 9v6m-6 3h12" label="Correlations" />

                <div className="px-4 mb-2 mt-6 text-[8px] font-mono uppercase text-[#444] tracking-widest font-bold">Execution</div>

                {/* Agents: CPU / Chip */}
                <NavItem id="AGENTS" currentView={currentView} onNavigate={onNavigate} path="M4 4h16v16H4zM9 4v-3M15 4v-3M9 20v3M15 20v3M20 9h3M20 15h3M4 9h-3M4 15h-3" label="Agents" />

                {/* Governance: Eye / Oversight */}
                <NavItem id="GOVERNANCE" currentView={currentView} onNavigate={onNavigate} path="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8zM12 9a3 3 0 1 0 0 6 3 3 0 0 0 0-6z" label="Governance" />

                {/* Teams: Users / Collaboration -> Replaces Factory */}
                <NavItem id="TEAMS" currentView={currentView} onNavigate={onNavigate} path="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2M16 3.13a4 4 0 0 1 0 7.75" label="Teams" />

                {/* Artifacts: Documentation / Files */}
                <NavItem id="ARTIFACTS" currentView={currentView} onNavigate={onNavigate} path="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z M14 2v6h6" label="Artifacts" />

                {/* Escalations: Alert Triangle */}
                <NavItem id="ESCALATIONS" currentView={currentView} onNavigate={onNavigate} path="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0zM12 9v4M12 17h.01" label="Escalations" alert={true} />
            </div>

            {/* System Tools */}
            <div className="p-0 border-t border-[#333]">
                <div className="py-2">
                    <NavItem id="SETTINGS" currentView={currentView} onNavigate={onNavigate} path="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm0 0v3m0-9V6m3 6h3m-9 0H6m9.5 5.5l2.1 2.1m-11.2-11.2l-2.1-2.1m0 11.2l-2.1 2.1m11.2-11.2l2.1-2.1" label="Settings" />
                    <NavItem id="HELP" currentView={currentView} onNavigate={onNavigate} path="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3M12 17h.01" label="Help" />
                </div>

                <div className="px-4 py-3 bg-black border-t border-[#333] flex justify-between items-center group cursor-pointer">
                    <div className="flex items-center gap-2 text-[#444] group-hover:text-gray-400 transition-colors">
                        <span className="text-[10px] uppercase">●</span>
                        <span className="text-[9px] font-mono">v5.2.0</span>
                    </div>
                    <div className="w-1.5 h-1.5 bg-[#333] group-hover:bg-[#00ff41] transition-colors"></div>
                </div>
            </div>

        </div>
    );
});
