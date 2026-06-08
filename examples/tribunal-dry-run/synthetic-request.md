# Tribunal Advisory Request
**Status**: PENDING
**Date**: 2026-06-07
**Author**: Antigravity Local Agent

## Execution Plan Synopsis
Implementing the MTU safeguard in `executor/FreeBSD.rs`. Adding an IPC call to `bs-watchdog` with the MTU snapshot.

```json
{
  "milestone": "M7",
  "intent": "Implement Dead-Man's Switch Rollback",
  "affected_components": [
    "runtime/bs-edge-agent/src/executor/FreeBSD.rs",
    "runtime/bs-edge-agent/src/watchdog.rs"
  ]
}
```

## Security & Boundary Checklist
- [x] Are any raw shell commands being generated?
- [x] Is `executor/FreeBSD.rs` the ONLY location invoking `Command::new`?
- [x] Do these changes violate the Bounded Profile Engine rules?

## Request for Advisory
*This form is to be submitted to a local evaluator agent (or human) for a passive advisory review.*
