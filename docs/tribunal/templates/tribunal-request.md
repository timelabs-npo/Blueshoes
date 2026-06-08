# Tribunal Advisory Request
**Status**: [DRAFT | PENDING | REVIEWED]
**Date**: YYYY-MM-DD
**Author**: [Name/Agent]

## Execution Plan Synopsis
Provide a brief overview of the exact changes being proposed. Include file paths and command boundaries.

```json
{
  "milestone": "X.Y",
  "intent": "Brief description of the intent",
  "affected_components": [
    "path/to/component"
  ]
}
```

## Security & Boundary Checklist
- [ ] Are any raw shell commands being generated?
- [ ] Is `executor/FreeBSD.rs` the ONLY location invoking `Command::new`?
- [ ] Do these changes violate the Bounded Profile Engine rules?

## Request for Advisory
*This form is to be submitted to a local evaluator agent (or human) for a passive advisory review.*
