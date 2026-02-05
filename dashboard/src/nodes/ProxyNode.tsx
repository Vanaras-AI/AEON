import { memo } from 'react';
import { Handle, Position } from 'reactflow';
import clsx from 'clsx';

export default memo(({ data, selected }: { data: any, selected: boolean }) => {
    return (
        <div className={clsx(
            "w-[300px] bg-[#0a0a0a] border rounded-md overflow-hidden shadow-sm transition-all duration-300",
            selected ? "border-[#00aaff] ring-1 ring-[#00aaff]" : "border-[#333] hover:border-[#00aaff]"
        )}>
            {/* Header */}
            <div className="px-3 py-2 bg-[#021019] border-b border-[#333] flex items-center justify-between">
                <div className="flex items-center gap-2">
                    <div className="w-6 h-6 bg-[#002233] rounded flex items-center justify-center text-xs">🛡️</div>
                    <span className="text-xs font-bold text-white tracking-widest uppercase">INTENT_GATEWAY</span>
                </div>
            </div>

            {/* Body */}
            <div className="p-0 font-mono text-[10px]">

                {/* Live Intent Field */}
                <div className="p-3 bg-black border-b border-[#222]">
                    <div className="flex justify-between items-center mb-2">
                        <span className="text-gray-500 uppercase text-[9px]">Incoming Signal</span>
                        <div className="flex items-center gap-1.5">
                            <span className="w-1.5 h-1.5 bg-[#00aaff] rounded-full animate-pulse"></span>
                            <span className="text-[#00aaff]">HTTPS</span>
                        </div>
                    </div>

                    <div className="p-2 bg-[#050505] border-l-2 border-[#00aaff] text-gray-300 italic min-h-[40px] flex items-center">
                        {data.intent ? `"${data.intent}"` : <span className="opacity-30">Awaiting user input...</span>}
                    </div>
                </div>

                {/* Stats Grid */}
                <div className="grid grid-cols-2 divide-x divide-[#222] bg-black">
                    <div className="p-2 flex items-center justify-between group cursor-pointer hover:bg-[#111]">
                        <span className="text-gray-600 text-[9px] uppercase">Sec_Score</span>
                        <span className="text-white font-bold group-hover:text-[#00aaff]">99.9%</span>
                    </div>
                    <div className="p-2 flex items-center justify-between group cursor-pointer hover:bg-[#111]">
                        <span className="text-gray-600 text-[9px] uppercase">Req/Min</span>
                        <span className="text-white font-bold">42</span>
                    </div>
                </div>
            </div>

            {/* Output Port */}
            <Handle
                type="source"
                position={Position.Right}
                className="!w-3 !h-3 !bg-[#00aaff] !border-2 !border-[#021019] right-[-6px] rounded-full top-1/2"
            />
        </div>
    );
});
