import { memo } from 'react';
import { Handle, Position } from 'reactflow';
import clsx from 'clsx';

export default memo(({ data, selected }: { data: any, selected: boolean }) => {
    return (
        <div className={clsx(
            "w-[340px] bg-[#0a0a0a] border rounded-md overflow-hidden shadow-lg transition-all duration-300",
            selected ? "border-[#ffd700] ring-1 ring-[#ffd700]" : "border-[#333] hover:border-[#ffd700]"
        )}>
            {/* Header */}
            <div className="px-3 py-2 bg-[#2a2400] border-b border-[#333] flex items-center justify-between">
                <div className="flex items-center gap-2">
                    <div className="w-6 h-6 bg-[#3d3400] rounded flex items-center justify-center text-xs">⚖️</div>
                    <span className="text-xs font-bold text-[#ffd700] tracking-widest uppercase">POLICY ENGINE</span>
                </div>
                <span className="px-1.5 py-0.5 bg-[#ffd700]/20 rounded text-[9px] text-[#ffd700] border border-[#ffd700]/30 animate-pulse">
                    ENFORCING
                </span>
            </div>

            {/* Body */}
            <div className="p-0 font-mono text-[10px]">

                {/* Verdict Field */}
                <div className="p-3 border-b border-[#222] bg-black">
                    <div className="text-gray-500 mb-2 uppercase text-[9px] flex justify-between">
                        <span>Constitutional Verdict</span>
                        <span className="text-[#ffd700]">v1.0.0</span>
                    </div>
                    <div className={clsx(
                        "p-2 border rounded min-h-[40px] flex items-center justify-center font-bold tracking-wider",
                        data.status === 'BLOCKED' ? "bg-red-900/20 border-red-500 text-red-500" :
                            data.status === 'PASSED' ? "bg-[#ffd700]/10 border-[#ffd700]/30 text-[#ffd700]" :
                                "bg-[#050505] border-[#222] text-gray-500"
                    )}>
                        {data.status || 'IDLE'}
                    </div>
                    {data.violation && (
                        <div className="mt-2 text-red-400 text-[9px] border-l-2 border-red-500 pl-2">
                            {data.violation}
                        </div>
                    )}
                </div>

                {/* Active Rules Row */}
                <div className="bg-[#111] p-2 border-b border-[#222]">
                    <div className="text-gray-600 text-[8px] uppercase mb-1">Active Articles</div>
                    <div className="flex gap-1 flex-wrap">
                        {['I.1 Non-Destruction', 'II.1 Egress Policy', 'III.1 Fair Use'].map(rule => (
                            <span key={rule} className="px-1.5 py-0.5 bg-[#222] rounded text-[8px] text-gray-400 border border-[#333]">
                                {rule}
                            </span>
                        ))}
                    </div>
                </div>
            </div>

            {/* Ports */}
            <Handle type="target" position={Position.Top} className="!w-3 !h-3 !bg-[#ffd700] !border-2 !border-[#2a2400] top-[-6px] rounded-full" />
            <Handle type="source" position={Position.Bottom} className="!w-3 !h-3 !bg-[#ffd700] !border-2 !border-[#2a2400] bottom-[-6px] rounded-full" />
        </div>
    );
});
