"""
A2G Protocol SDK - Agent-to-Governance Protocol for Python

This SDK provides Python bindings for the A2G Protocol, enabling easy integration
with AEON governance systems.

Example:
    >>> from a2g_sdk import A2gClient, A2gIntent
    >>> 
    >>> client = A2gClient("ws://localhost:3000")
    >>> await client.connect()
    >>> 
    >>> verdict = await client.request_intent(
    ...     tool="write_file",
    ...     arguments={"path": "/tmp/test.txt", "content": "Hello"}
    ... )
    >>> 
    >>> if verdict.is_approved:
    ...     result = execute_tool(verdict.manifest)
    ...     await client.report_success(verdict.intent_id, result)
"""

from .messages import (
    A2gIntent,
    A2gReport,
    A2gRegister,
    G2aVerdict,
    G2aPolicy,
    RiskAssessment,
    RiskLevel,
    Verdict,
    CapabilityManifest,
    ExecutionStatus,
)

from .client import A2gClient

__version__ = "1.0.0"
__all__ = [
    "A2gClient",
    "A2gIntent",
    "A2gReport", 
    "A2gRegister",
    "G2aVerdict",
    "G2aPolicy",
    "RiskAssessment",
    "RiskLevel",
    "Verdict",
    "CapabilityManifest",
    "ExecutionStatus",
]
