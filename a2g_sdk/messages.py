"""
A2G Protocol Message Types

Defines the core message types for Agent-to-Governance communication.
"""

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any
from enum import Enum
from datetime import datetime
import uuid


# =============================================================================
# ENUMS
# =============================================================================

class Verdict(str, Enum):
    """Governance verdict on an intent"""
    APPROVED = "APPROVED"
    DENIED = "DENIED"
    ESCALATE = "ESCALATE"
    CONDITIONAL = "CONDITIONAL"


class RiskLevel(str, Enum):
    """Risk level classification"""
    CRITICAL = "CRITICAL"
    HIGH = "HIGH"
    MEDIUM = "MEDIUM"
    LOW = "LOW"
    
    @classmethod
    def from_score(cls, score: float) -> "RiskLevel":
        """Determine risk level from score"""
        if score >= 0.9:
            return cls.CRITICAL
        elif score >= 0.7:
            return cls.HIGH
        elif score >= 0.4:
            return cls.MEDIUM
        return cls.LOW


class ExecutionStatus(str, Enum):
    """Execution outcome status"""
    SUCCESS = "SUCCESS"
    FAILURE = "FAILURE"
    TIMEOUT = "TIMEOUT"
    ABORTED = "ABORTED"


# =============================================================================
# A2G MESSAGES (Agent → Governance)
# =============================================================================

@dataclass
class IntentContext:
    """Context information for an intent"""
    session_id: Optional[str] = None
    parent_intent: Optional[str] = None
    reasoning: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        return {k: v for k, v in {
            "session_id": self.session_id,
            "parent_intent": self.parent_intent,
            "reasoning": self.reasoning,
        }.items() if v is not None}


@dataclass
class A2gIntent:
    """A2G_INTENT: Request permission to perform an action"""
    agent_did: str
    tool: str
    arguments: Dict[str, Any]
    intent_id: str = field(default_factory=lambda: str(uuid.uuid4()))
    context: Optional[IntentContext] = None
    
    def to_jsonrpc(self) -> Dict[str, Any]:
        """Convert to JSON-RPC format"""
        params = {
            "agent_did": self.agent_did,
            "intent_id": self.intent_id,
            "tool": self.tool,
            "arguments": self.arguments,
        }
        if self.context:
            params["context"] = self.context.to_dict()
            
        return {
            "jsonrpc": "2.0",
            "method": "a2g/intent",
            "params": params,
            "id": str(uuid.uuid4()),
        }


@dataclass
class ExecutionMetrics:
    """Metrics from tool execution"""
    duration_ms: int
    memory_used_mb: Optional[int] = None
    cpu_percent: Optional[float] = None
    
    def to_dict(self) -> Dict[str, Any]:
        return {k: v for k, v in {
            "duration_ms": self.duration_ms,
            "memory_used_mb": self.memory_used_mb,
            "cpu_percent": self.cpu_percent,
        }.items() if v is not None}


@dataclass
class A2gReport:
    """A2G_REPORT: Report execution outcome"""
    agent_did: str
    intent_id: str
    status: ExecutionStatus
    result: Optional[Dict[str, Any]] = None
    metrics: Optional[ExecutionMetrics] = None
    error: Optional[str] = None
    
    def to_jsonrpc(self) -> Dict[str, Any]:
        """Convert to JSON-RPC format"""
        params: Dict[str, Any] = {
            "agent_did": self.agent_did,
            "intent_id": self.intent_id,
            "status": self.status.value,
        }
        if self.result:
            params["result"] = self.result
        if self.metrics:
            params["metrics"] = self.metrics.to_dict()
        if self.error:
            params["error"] = self.error
            
        return {
            "jsonrpc": "2.0",
            "method": "a2g/report",
            "params": params,
            "id": str(uuid.uuid4()),
        }
    
    @classmethod
    def success(cls, agent_did: str, intent_id: str, result: Dict[str, Any], 
                duration_ms: int) -> "A2gReport":
        """Create a success report"""
        return cls(
            agent_did=agent_did,
            intent_id=intent_id,
            status=ExecutionStatus.SUCCESS,
            result=result,
            metrics=ExecutionMetrics(duration_ms=duration_ms),
        )
    
    @classmethod
    def failure(cls, agent_did: str, intent_id: str, error: str) -> "A2gReport":
        """Create a failure report"""
        return cls(
            agent_did=agent_did,
            intent_id=intent_id,
            status=ExecutionStatus.FAILURE,
            error=error,
        )


@dataclass
class AgentMetadata:
    """Agent metadata for registration"""
    name: str
    version: str
    runtime: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        return {k: v for k, v in {
            "name": self.name,
            "version": self.version,
            "runtime": self.runtime,
        }.items() if v is not None}


@dataclass
class A2gRegister:
    """A2G_REGISTER: Register agent with governance"""
    agent_did: str
    public_key: str
    capabilities_requested: List[str]
    metadata: Optional[AgentMetadata] = None
    
    def to_jsonrpc(self) -> Dict[str, Any]:
        """Convert to JSON-RPC format"""
        params: Dict[str, Any] = {
            "agent_did": self.agent_did,
            "public_key": self.public_key,
            "capabilities_requested": self.capabilities_requested,
        }
        if self.metadata:
            params["metadata"] = self.metadata.to_dict()
            
        return {
            "jsonrpc": "2.0",
            "method": "a2g/register",
            "params": params,
            "id": str(uuid.uuid4()),
        }


# =============================================================================
# G2A MESSAGES (Governance → Agent)
# =============================================================================

@dataclass
class RiskAssessment:
    """Risk assessment result from governance"""
    score: float
    level: RiskLevel
    model_score: Optional[float] = None
    heuristic_score: Optional[float] = None
    threats: List[str] = field(default_factory=list)
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "RiskAssessment":
        return cls(
            score=data["score"],
            level=RiskLevel(data["level"]),
            model_score=data.get("model_score"),
            heuristic_score=data.get("heuristic_score"),
            threats=data.get("threats", []),
        )


@dataclass
class CapabilityManifest:
    """Capability limits for execution"""
    max_memory_mb: Optional[int] = None
    max_cpu_percent: Optional[int] = None
    timeout_seconds: Optional[int] = None
    network_allowed: Optional[bool] = None
    filesystem_scope: Optional[List[str]] = None
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CapabilityManifest":
        return cls(
            max_memory_mb=data.get("max_memory_mb"),
            max_cpu_percent=data.get("max_cpu_percent"),
            timeout_seconds=data.get("timeout_seconds"),
            network_allowed=data.get("network_allowed"),
            filesystem_scope=data.get("filesystem_scope"),
        )


@dataclass  
class G2aVerdict:
    """G2A_VERDICT: Governance response to intent"""
    verdict: Verdict
    intent_id: str
    risk_assessment: RiskAssessment
    capability_manifest: Optional[CapabilityManifest] = None
    conditions: Optional[List[str]] = None
    expires_at: Optional[datetime] = None
    error_code: Optional[int] = None
    error_message: Optional[str] = None
    
    @property
    def is_approved(self) -> bool:
        """Check if intent was approved"""
        return self.verdict == Verdict.APPROVED
    
    @property
    def is_denied(self) -> bool:
        """Check if intent was denied"""
        return self.verdict == Verdict.DENIED
    
    @classmethod
    def from_jsonrpc(cls, response: Dict[str, Any]) -> "G2aVerdict":
        """Parse from JSON-RPC response"""
        if "error" in response and response["error"]:
            error = response["error"]
            # Extract from error response
            data = error.get("data", {})
            return cls(
                verdict=Verdict.DENIED,
                intent_id=data.get("intent_id", ""),
                risk_assessment=RiskAssessment(
                    score=data.get("risk_score", 1.0),
                    level=RiskLevel.CRITICAL,
                    threats=[error.get("message", "Unknown error")],
                ),
                error_code=error.get("code"),
                error_message=error.get("message"),
            )
        
        result = response.get("result", {})
        return cls(
            verdict=Verdict(result["verdict"]),
            intent_id=result["intent_id"],
            risk_assessment=RiskAssessment.from_dict(result["risk_assessment"]),
            capability_manifest=(
                CapabilityManifest.from_dict(result["capability_manifest"])
                if result.get("capability_manifest") else None
            ),
            conditions=result.get("conditions"),
            expires_at=(
                datetime.fromisoformat(result["expires_at"].replace("Z", "+00:00"))
                if result.get("expires_at") else None
            ),
        )


@dataclass
class ToolPolicy:
    """Policy for a specific tool"""
    allowed: bool
    constraints: Optional[Dict[str, Any]] = None
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ToolPolicy":
        return cls(
            allowed=data["allowed"],
            constraints=data.get("constraints"),
        )


@dataclass
class NetworkPolicy:
    """Network access policy"""
    allowed_domains: List[str] = field(default_factory=list)
    blocked_domains: List[str] = field(default_factory=list)
    max_requests_per_minute: Optional[int] = None


@dataclass
class ResourceLimits:
    """Global resource limits"""
    max_memory_mb: Optional[int] = None
    max_cpu_percent: Optional[int] = None
    max_disk_mb: Optional[int] = None


@dataclass
class G2aPolicy:
    """G2A_POLICY: Governance capabilities for agent"""
    agent_did: str
    version: str
    tools: Dict[str, ToolPolicy] = field(default_factory=dict)
    network: Optional[NetworkPolicy] = None
    resources: Optional[ResourceLimits] = None
    constitution_hash: Optional[str] = None
    
    @classmethod
    def from_jsonrpc(cls, response: Dict[str, Any]) -> "G2aPolicy":
        """Parse from JSON-RPC notification"""
        params = response.get("params", {})
        capabilities = params.get("capabilities", {})
        
        tools = {}
        for name, policy in capabilities.get("tools", {}).items():
            tools[name] = ToolPolicy.from_dict(policy)
            
        return cls(
            agent_did=params["agent_did"],
            version=params["version"],
            tools=tools,
            constitution_hash=params.get("constitution_hash"),
        )


# =============================================================================
# ERROR CODES
# =============================================================================

class A2gErrorCodes:
    """Standard A2G Protocol error codes"""
    PARSE_ERROR = -32700
    INVALID_REQUEST = -32600
    METHOD_NOT_FOUND = -32601
    INVALID_PARAMS = -32602
    POLICY_VIOLATION = -32000
    EXECUTION_ERROR = -32001
    REGISTRATION_FAILED = -32002
    CAPABILITY_EXHAUSTED = -32003
    SESSION_EXPIRED = -32004
