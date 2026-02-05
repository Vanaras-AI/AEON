import { memo } from 'react';
import { Handle, Position } from 'reactflow';
import clsx from 'clsx';

export default memo(({ data, selected }: { data: any, selected: boolean }) => {
    return (
        <div className={clsx(
            "w-[280px] bg-[#0a0a0a] border rounded-md overflow-hidden shadow-sm transition-all duration-300",
            selected ? "border-white ring-1 ring-white" : "border-[#333] hover:border-[#666]",
            data.status === 'COMPROMISED' && "border-red-500 shadow-[0_0_15px_rgba(239,68,68,0.5)] animate-pulse"
        )}>
            {/* Logic Card Header */}
            <div className={clsx(
                "px-3 py-2 border-b border-[#222] flex items-center justify-between",
                data.status === 'ACTIVE' ? "bg-[#111]" : "bg-[#1a0505] text-red-500"
            )}>
                <div className="flex items-center gap-2">
                    <div className={clsx(
                        "w-2 h-2 rounded-sm",
                        data.status === 'ACTIVE' ? "bg-[#00ff41]" : "bg-red-500 animate-ping"
                    )}></div>
                    <span className="text-xs font-bold tracking-tight">{data.label}</span>
                </div>
                <span className="text-[9px] font-mono text-gray-500 uppercase tracking-wider">v2.1</span>
            </div>

            {/* Properties Body */}
            <div className="p-0 text-[10px] font-mono">
                {/* Row 1: DID */}
                <div className="px-3 py-2 border-b border-[#222] bg-black flex justify-between items-center group">
                    <span className="text-gray-500 group-hover:text-gray-300 transition-colors">IDENTITY</span>
                    <span className="text-gray-400 select-all">{data.did}</span>
                </div>

                {/* Row 2: Metrics */}
                <div className="px-3 py-2 border-b border-[#222] bg-black grid grid-cols-2 gap-4">
                    <div>
                        <div className="text-gray-600 mb-0.5 text-[9px] uppercase">Throughput</div>
                        <div className="text-white flex items-center gap-1.5">
                            <span>{data.ops || 0}</span>
                            <span className="text-gray-600 text-[8px]">ops/s</span>
                        </div>
                    </div>
                    <div>
                        <div className="text-gray-600 mb-0.5 text-[9px] uppercase">Latency</div>
                        <div className="text-white">12ms</div>
                    </div>
                </div>

                {/* Row 3: CPU Load */}
                <div className="px-3 py-2 bg-black flex flex-col gap-1">
                    <div className="flex justify-between text-gray-500 text-[9px]">
                        <span>CPU_LOAD</span>
                        <span>{Math.round(data.load || 0)}%</span>
                    </div>
                    <div className="w-full h-1 bg-[#222] rounded-full overflow-hidden">
                        <div className="h-full bg-white transition-all duration-300" style={{ width: `${Math.min(data.load || 0, 100)}%` }}></div>
                    </div>
                </div>
            </div>

            {/* Ports - Styled as Input/Output Circle Anchors */}
            <Handle
                type="target"
                position={Position.Top}
                className="!w-3 !h-3 !bg-[#222] !border-2 !border-[#555] top-[-6px] rounded-full hover:!bg-white transition-colors"
            />
            <Handle
                type="source"
                position={Position.Bottom}
                className="!w-3 !h-3 !bg-[#222] !border-2 !border-[#555] bottom-[-6px] rounded-full hover:!bg-white transition-colors"
            />

            {/* Side Port (Optional for logic flow) */}
            <Handle
                type="target"
                id="input-side"
                position={Position.Left}
                className="!w-2 !h-8 !rounded-sm !bg-[#222] !border border-[#444] left-[-4px]"
            />
        </div>
    );
});
