# Cognitive Substrate — Layer 1 + 2 Acceptance Record

**Status:** `COGNITIVE_SUBSTRATE_LAYER_1_2_READY`
**Accepted:** 2026-06-08T08:46:00Z
**Scope:** Layer 1 (Doctrine) + Layer 2 (Transport Language) only

---

## Acceptance Scope

```text
1. Doctrine          .rhea/           ACCEPTED
2. Transport Lang    .rhea/schemas/   ACCEPTED
3. Runtime           (not scoped)     OUT OF SCOPE
4. Orchestration     (not scoped)     OUT OF SCOPE
5. Products / UI     (not scoped)     OUT OF SCOPE
```

---

## Accepted Files

| File | SHA-256 |
| --- | --- |
| `.rhea/constitution.md` | `11a79630741dc12763dcbafba37f20cbfbc1be28e9a4fc13d7ee597969ab61d0` |
| `.rhea/language.md` | `84a1dbe4f8d07a0e718e6f679e02ba71ec95e23a275a9975c8e790f1b6a37907` |
| `.rhea/protocol.md` | `667e6ede0e09ab1dd5bee0e9761e08a35e27fafdfa61b2109f519f299d2e6029` |
| `.rhea/status.example.md` | `6011fe6641ea8f23f7e5621c344d4496fb6f5c7da786a8f7dfdd709769c3c396` |
| `.rhea/schemas/message.v0.schema.json` | `6fa2f7e26cfe00a6641de4cdc8f4eb43d38a095f161a0c423f7731029085edb6` |
| `.rhea/tools/validate_message.py` | `4910139bf798e9f4d151125b01a981d7624c54259d02b6966dee961c26f52a4a` |
| `.rhea/examples/sample_message.json` | `339d13c2288107796be58ba91713de9ff228abb033b5d66332501e29bc9585f8` |
| `.rhea/tests/test_semantic_preservation.py` | `9d217af85a7e4a642a4caddb8e0d43d084d4fa8019ea9d268c7eea82e0d28a20` |

---

## Applied Patches

1. **Path notation** — All `/.rhea/` replaced with `.rhea/`. Filesystem root vs. repo root.
2. **Schema rename** — `message_schema.json` → `message.v0.schema.json`.
3. **Success message** — `"Rhea schema"` → `"cognitive substrate schema"`.
4. **Status file** — `status.md` → `status.example.md`. Live mutable state must not be committed as canonical doctrine.
5. **No network rule** — `NoNetworkGuard` test class enforces zero HTTP client imports. Structural tests only.
6. **Empty payload rejection** — `minProperties: 1` in schema. Empty `{}` is a syntactically valid semantic vacuum. Rejected at schema level. Path to legitimate no-op: add explicit `"noop"` message type.
7. **Namespace marker** — `.rhea/` marked `LEGACY_NAMESPACE` in `constitution.md`. Migration to neutral name deferred: stability > purity.

---

## Invariants (from `.rhea/constitution.md`)

1. No irreversible mutation — every state transition is verifiable and reversible.
2. Governance-first — doctrine and transport language before any runtime or UI.
3. No layer collapse — doctrine must not contain runtime logic; runtime must not contain doctrine.
4. No vendor lock — no vendor or model name in doctrine; bindings are runtime configuration.
5. Thin viewports — UI products are cognition viewports only; they do not affect governance.

---

## Hard Boundary — Explicitly Rejected

| Rejected Item | Reason |
| --- | --- |
| `runtime/bs-edge-agent/src/cognitive/*` | Layer violation — runtime, not doctrine |
| Rust cognitive modules | Layer violation |
| Tribunal CLI execution | Mythology |
| Vendor model bindings in doctrine | Invariant 4 violation |
| Antigravity IDE settings mutation | Out of scope |
| `"No further approval needed"` wording | Every step requires explicit approval |
| Evidence persistence inside `constitution.md` | Constitution is invariants-only |

---

## Verification

```bash
python3 .rhea/tools/validate_message.py .rhea/examples/sample_message.json
→ [OK] Message is valid according to the cognitive substrate schema.

python3 -m pytest .rhea/tests/ -v
→ 11 passed in 0.06s

Platform: darwin — Python 3.13.6, pytest-9.0.3
```

### Test Suite

| Test | Result |
| --- | --- |
| `NoNetworkGuard::test_no_http_client_imports` | PASSED |
| `RoundTripEquality::test_round_trip` | PASSED |
| `RoundTripEquality::test_canonical_is_stable` | PASSED |
| `DriftDetection::test_identical_messages_same_hash` | PASSED |
| `DriftDetection::test_changed_payload_different_hash` | PASSED |
| `DriftDetection::test_changed_origin_different_hash` | PASSED |
| `AmbiguityRejection::test_empty_payload_rejected` | PASSED |
| `AmbiguityRejection::test_extra_field_rejected` | PASSED |
| `AmbiguityRejection::test_missing_required_field_rejected` | PASSED |
| `AmbiguityRejection::test_unknown_origin_rejected` | PASSED |
| `AmbiguityRejection::test_unknown_type_rejected` | PASSED |

---

## What Was Not Built

Layer 3 (Runtime), Layer 4 (Orchestration), and Layer 5 (Products/UI) are out of scope.
No code was written inside `runtime/bs-edge-agent/`.
No Rust modules. No tribunal runtime. No orchestration config.
Layer 3 requires a separate planning session with explicit approval.

---

`COGNITIVE_SUBSTRATE_LAYER_1_2_READY`
