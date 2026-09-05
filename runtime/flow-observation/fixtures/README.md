# Pinned independent corpus

Only JSON data/schema bytes from omnia-playbook are vendored in `omnia/`.
No Python validator, oracle implementation or generated producer output is imported.
`omnia-lock.json` records the immutable companion commit and SHA-256 of every file.
The tests check these bytes and independent fixture expectations. LF checkout rules
keep hashes identical on Windows and Unix.

The revised wire schema allows unavailable byte counters as null. Old integer-only
readers reject such documents. Existing valid V1 documents remain valid. NetBSD is
still accepted by the wire parser; the native fixture DTO covers four platforms.

Fixture evaluation is deterministic and never qualifies a live native adapter.
