"""
A2G Protocol Client

Async client for communicating with AEON governance servers.
"""

import asyncio
import json
from typing import Optional, Dict, Any, Callable, Awaitable
from dataclasses import dataclass
import logging

from .messages import (
    A2gIntent,
    A2gReport,
    A2gRegister,
    G2aVerdict,
    G2aPolicy,
    AgentMetadata,
    ExecutionStatus,
    IntentContext,
)

logger = logging.getLogger(__name__)


@dataclass
class ClientConfig:
    """Configuration for A2G client"""
    agent_did: str
    governance_url: str = "ws://localhost:3000"
    timeout_seconds: float = 30.0
    auto_reconnect: bool = True
    max_retries: int = 3


class A2gClient:
    """
    Async client for A2G Protocol communication.
    
    Example:
        >>> async with A2gClient(config) as client:
        ...     verdict = await client.request_intent("write_file", {"path": "/tmp/x"})
        ...     if verdict.is_approved:
        ...         # Execute tool
        ...         await client.report_success(verdict.intent_id, result, duration_ms=45)
    """
    
    def __init__(self, config: ClientConfig):
        self.config = config
        self._ws = None
        self._connected = False
        self._policy: Optional[G2aPolicy] = None
        self._pending_requests: Dict[str, asyncio.Future] = {}
        self._on_policy_update: Optional[Callable[[G2aPolicy], Awaitable[None]]] = None
        
    async def __aenter__(self):
        await self.connect()
        return self
        
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.disconnect()
        
    async def connect(self) -> None:
        """Connect to governance server and register agent"""
        try:
            import websockets
        except ImportError:
            raise ImportError("websockets package required. Install with: pip install websockets")
            
        logger.info(f"Connecting to {self.config.governance_url}")
        self._ws = await websockets.connect(self.config.governance_url)
        self._connected = True
        
        # Start message receiver task
        asyncio.create_task(self._receive_loop())
        
        # Register with governance
        await self._register()
        logger.info(f"Connected and registered as {self.config.agent_did}")
        
    async def disconnect(self) -> None:
        """Disconnect from governance server"""
        self._connected = False
        if self._ws:
            await self._ws.close()
            self._ws = None
            
    async def _register(self) -> None:
        """Register agent with governance"""
        register = A2gRegister(
            agent_did=self.config.agent_did,
            public_key="",  # TODO: Ed25519 key generation
            capabilities_requested=["write_file", "read_file", "execute_command"],
            metadata=AgentMetadata(name="Python Agent", version="1.0.0", runtime="python"),
        )
        
        await self._send(register.to_jsonrpc())
        
    async def _receive_loop(self) -> None:
        """Background task to receive and route messages"""
        while self._connected and self._ws:
            try:
                raw = await self._ws.recv()
                msg = json.loads(raw)
                await self._handle_message(msg)
            except Exception as e:
                if self._connected:
                    logger.error(f"Receive error: {e}")
                break
                
    async def _handle_message(self, msg: Dict[str, Any]) -> None:
        """Route incoming message to handler"""
        # Check if it's a response to a pending request
        msg_id = msg.get("id")
        if msg_id and msg_id in self._pending_requests:
            future = self._pending_requests.pop(msg_id)
            future.set_result(msg)
            return
            
        # Handle notifications
        method = msg.get("method")
        if method == "g2a/policy":
            policy = G2aPolicy.from_jsonrpc(msg)
            self._policy = policy
            if self._on_policy_update:
                await self._on_policy_update(policy)
                
    async def _send(self, msg: Dict[str, Any]) -> None:
        """Send message to governance server"""
        if not self._ws:
            raise RuntimeError("Not connected")
        await self._ws.send(json.dumps(msg))
        
    async def _request(self, msg: Dict[str, Any]) -> Dict[str, Any]:
        """Send request and wait for response"""
        msg_id = msg["id"]
        future: asyncio.Future = asyncio.get_event_loop().create_future()
        self._pending_requests[msg_id] = future
        
        await self._send(msg)
        
        try:
            response = await asyncio.wait_for(future, timeout=self.config.timeout_seconds)
            return response
        except asyncio.TimeoutError:
            self._pending_requests.pop(msg_id, None)
            raise TimeoutError(f"Request {msg_id} timed out")
    
    async def request_intent(
        self, 
        tool: str, 
        arguments: Dict[str, Any],
        reasoning: Optional[str] = None,
    ) -> G2aVerdict:
        """
        Request permission to execute a tool.
        
        Args:
            tool: Name of tool to execute
            arguments: Tool arguments
            reasoning: Optional explanation for audit
            
        Returns:
            G2aVerdict with approval status and capability manifest
        """
        intent = A2gIntent(
            agent_did=self.config.agent_did,
            tool=tool,
            arguments=arguments,
            context=IntentContext(reasoning=reasoning) if reasoning else None,
        )
        
        response = await self._request(intent.to_jsonrpc())
        return G2aVerdict.from_jsonrpc(response)
    
    async def report_success(
        self, 
        intent_id: str, 
        result: Dict[str, Any], 
        duration_ms: int
    ) -> None:
        """Report successful execution"""
        report = A2gReport.success(
            agent_did=self.config.agent_did,
            intent_id=intent_id,
            result=result,
            duration_ms=duration_ms,
        )
        await self._send(report.to_jsonrpc())
        
    async def report_failure(self, intent_id: str, error: str) -> None:
        """Report failed execution"""
        report = A2gReport.failure(
            agent_did=self.config.agent_did,
            intent_id=intent_id,
            error=error,
        )
        await self._send(report.to_jsonrpc())
        
    def on_policy_update(self, callback: Callable[[G2aPolicy], Awaitable[None]]) -> None:
        """Register callback for policy updates"""
        self._on_policy_update = callback
        
    @property
    def policy(self) -> Optional[G2aPolicy]:
        """Current policy from governance"""
        return self._policy
        
    @property
    def is_connected(self) -> bool:
        """Check if connected to governance"""
        return self._connected


# =============================================================================
# SYNC WRAPPER
# =============================================================================

class A2gClientSync:
    """
    Synchronous wrapper for A2G client.
    
    For use in non-async contexts.
    """
    
    def __init__(self, config: ClientConfig):
        self._config = config
        self._client: Optional[A2gClient] = None
        self._loop: Optional[asyncio.AbstractEventLoop] = None
        
    def connect(self) -> None:
        """Connect to governance server"""
        self._loop = asyncio.new_event_loop()
        self._client = A2gClient(self._config)
        self._loop.run_until_complete(self._client.connect())
        
    def disconnect(self) -> None:
        """Disconnect from server"""
        if self._client and self._loop:
            self._loop.run_until_complete(self._client.disconnect())
            self._loop.close()
            
    def request_intent(
        self, 
        tool: str, 
        arguments: Dict[str, Any],
        reasoning: Optional[str] = None,
    ) -> G2aVerdict:
        """Request permission to execute tool"""
        if not self._client or not self._loop:
            raise RuntimeError("Not connected")
        return self._loop.run_until_complete(
            self._client.request_intent(tool, arguments, reasoning)
        )
        
    def report_success(self, intent_id: str, result: Dict[str, Any], duration_ms: int) -> None:
        """Report successful execution"""
        if not self._client or not self._loop:
            raise RuntimeError("Not connected")
        self._loop.run_until_complete(
            self._client.report_success(intent_id, result, duration_ms)
        )
        
    def report_failure(self, intent_id: str, error: str) -> None:
        """Report failed execution"""
        if not self._client or not self._loop:
            raise RuntimeError("Not connected")
        self._loop.run_until_complete(
            self._client.report_failure(intent_id, error)
        )
