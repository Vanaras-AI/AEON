import { memo } from 'react';
import { Handle, Position } from 'reactflow';
import clsx from 'clsx';

export default memo(({ data, selected }: { data: any, selected: boolean }) => {
    return (
        <div className={clsx(
            "w-[340px] bg-[#0a0a0a] border rounded-md overflow-hidden shadow-lg transition-all duration-300",
            selected ? "border-[#a855f7] ring-1 ring-[#a855f7]" : "border-[#333] hover:border-[#a855f7]"
        )}>
            {/* Header */}
            <div className="px-3 py-2 bg-[#1e102e] border-b border-[#333] flex items-center justify-between">
                <div className="flex items-center gap-2">
                    <div className="w-6 h-6 bg-[#2d1b46] rounded flex items-center justify-center text-xs">🧠</div>
                    <span className="text-xs font-bold text-white tracking-widest uppercase">CORTEX-G_MODEL</span>
                </div>
                <span className="px-1.5 py-0.5 bg-[#a855f7]/20 rounded text-[9px] text-[#a855f7] border border-[#a855f7]/30">
                    v5.0-RC
                </span>
            </div>

            {/* Body */}
            <div className="p-0 font-mono text-[10px]">

                {/* Decision Output Field */}
                <div className="p-3 border-b border-[#222] bg-black">
                    <div className="text-gray-500 mb-2 uppercase text-[9px] flex justify-between">
                        <span>Latest Inference</span>
                        <span>{data.latency || '0ms'}</span>
                    </div>
                    <div className="p-2 bg-[#050505] border border-[#222] rounded text-gray-300 min-h-[60px] max-h-[100px] overflow-y-auto leading-relaxed">
                        {data.decision ? (
                            <span>{data.decision}</span>
                        ) : (
                            <span className="italic opacity-30">Waiting for trigger...</span>
                        )}
                    </div>
                </div>

                {/* Risk & Stats Row */}
                <div className="grid grid-cols-3 divide-x divide-[#222] bg-black border-b border-[#222]">
                    <div className="p-2 text-center">
                        <div className="text-gray-600 text-[8px] uppercase">Risk Rating</div>
                        <div className={clsx("text-xs font-bold mt-1", data.risk === 'HIGH' ? "text-red-500" : "text-[#00ff41]")}>
                            {data.risk || 'N/A'}
                        </div>
                    </div>
                    <div className="p-2 text-center">
                        <div className="text-gray-600 text-[8px] uppercase">Tokens/Sec</div>
                        <div className="text-xs font-bold text-white mt-1">128</div>
                    </div>
                    <div className="p-2 text-center">
                        <div className="text-gray-600 text-[8px] uppercase">Context</div>
                        <div className="text-xs font-bold text-white mt-1">4k</div>
                    </div>
                </div>
            </div>

            {/* Ports */}
            <Handle type="target" position={Position.Top} className="!w-3 !h-3 !bg-[#a855f7] !border-2 !border-[#1e102e] top-[-6px] rounded-full" />
            <Handle type="source" position={Position.Bottom} className="!w-3 !h-3 !bg-[#a855f7] !border-2 !border-[#1e102e] bottom-[-6px] rounded-full" />
        </div>
    );
});
