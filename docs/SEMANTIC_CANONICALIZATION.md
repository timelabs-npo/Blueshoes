# Semantic Canonicalization Contract

To guarantee identical provenance receipts across independent environments and runtimes, all entity state must be subjected to a strict canonicalization pass before cryptographic hashing. This ensures the deterministic reproduction of the semantic lineage.

## Rules of Canonicalization

1. **UTF-8 Encoding**: All serialized content MUST be strictly UTF-8 encoded without a BOM (Byte Order Mark).
2. **Sorted Object Keys**: All JSON objects, properties, and map representations MUST sort their keys lexicographically by ascending Unicode code point.
3. **Deterministic Array Ordering Rules**: Unordered collections (e.g., entity `relations`) MUST be explicitly sorted prior to hashing. Relations are sorted by `predicate` in ascending lexicographical order, followed by `target_id`.
4. **Normalized Timestamps**: All timestamps must be represented as `u64` Unix Epoch offsets (seconds since 1970-01-01T00:00:00Z) without timezone suffixes.
5. **No Implicit Defaults**: Optional fields must be explicitly omitted or explicitly set. No inferred zeros or empty strings during the hashing process.
6. **Explicit Null Handling**: `null` values are not permitted in `properties`. A missing property key denotes the absence of the property.
7. **Stable Floating-Point Formatting**: Numeric metrics are formatted using standard string representations without exponent notation unless the value explicitly demands it. Extraneous trailing zeros after the decimal point must not be used unless mandated by schema types.
8. **Newline Normalization**: The resulting canonical JSON string MUST NOT contain any pretty-printing whitespace, including newlines (`\n`, `\r\n`), spaces outside of strings, or indentation.
9. **Deterministic JSON Export Requirements**: The exported `JSON-LD` substrate graph must sort its array of nodes by `@id`.

Failure to adhere to these rules constitutes a violation of the provenance chain integrity, triggering a TAMPER alarm.
