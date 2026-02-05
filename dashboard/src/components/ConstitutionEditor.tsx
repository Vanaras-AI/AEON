import { memo, useState } from 'react';
import { motion } from 'framer-motion';

export default memo(({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
    const [config, setConfig] = useState(`[network]
allow = ["api.openai.com", "google.com"]
block = ["evil.com", "facebook.com"]

[filesystem]
# The Intent Governor Rules
allow_write = ["/tmp/*", "./*"]
block_delete = ["*.secret", "*.pem", "important_data.csv"]

[resources]
max_memory_mb = 512
max_files = 100
max_threads = 64`);

    if (!isOpen) return null;

    return (
        <div className="absolute inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-12">
            <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.95 }}
                className="w-full max-w-4xl bg-[#050505] border border-[#333] shadow-2xl flex flex-col h-[700px] overflow-hidden"
            >
                {/* Header */}
                <div className="h-12 border-b border-[#333] bg-[#0a0a0a] flex justify-between items-center px-4">
                    <div className="flex items-center gap-3">
                        <div className="w-6 h-6 bg-[#111] flex items-center justify-center border border-[#333]">
                            <span className="text-xs">📜</span>
                        </div>
                        <span className="text-sm font-bold text-white uppercase tracking-wider">
                            Sovereign Constitution <span className="text-gray-600">/</span> Editor
                        </span>
                    </div>
                    <button onClick={onClose} className="text-gray-500 hover:text-white transition-colors">
                        ✕ ESC
                    </button>
                </div>

                {/* Editor Area */}
                <div className="flex-1 flex overflow-hidden">
                    {/* Sidebar / Instructions */}
                    <div className="w-64 border-r border-[#222] bg-[#020202] p-4 font-mono text-[9px] text-gray-400 space-y-6">
                        <section>
                            <h3 className="text-white uppercase mb-2">Instructions</h3>
                            <p className="leading-relaxed">
                                This is your **Global Sovereign Constitution**.
                                These rules override any Agent Mandate.
                            </p>
                        </section>
                        <section>
                            <h3 className="text-white uppercase mb-2">Available Locks</h3>
                            <ul className="space-y-1">
                                <li>● network.allow</li>
                                <li>● network.block</li>
                                <li>● filesystem.block_delete</li>
                                <li>● resources.max_memory_mb</li>
                            </ul>
                        </section>
                        <div className="pt-8">
                            <span className="text-[#00ff41]/50 italic">"The User is the Law."</span>
                        </div>
                    </div>

                    {/* Code Editor */}
                    <div className="flex-1 bg-black relative">
                        <textarea
                            value={config}
                            onChange={(e) => setConfig(e.target.value)}
                            className="w-full h-full bg-transparent p-6 font-mono text-xs text-gray-300 resize-none outline-none selection:bg-[#00ff41]/30"
                            spellCheck={false}
                        />
                    </div>
                </div>

                {/* Footer */}
                <div className="p-4 border-t border-[#333] bg-[#050505] flex justify-between items-center">
                    <div className="flex items-center gap-4">
                        <span className="text-[10px] font-mono text-gray-500 uppercase">Status: <span className="text-[#00ff41]">Valid TOML</span></span>
                    </div>
                    <div className="flex gap-3">
                        <button onClick={onClose} className="px-6 py-2 border border-[#333] text-xs uppercase text-gray-500 hover:text-white transition-colors">
                            Discard
                        </button>
                        <button className="px-6 py-2 bg-white text-black text-xs font-bold uppercase tracking-widest hover:bg-[#00ff41] transition-colors">
                            Apply Globally
                        </button>
                    </div>
                </div>
            </motion.div>
        </div>
    );
});
