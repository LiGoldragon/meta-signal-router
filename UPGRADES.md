# Upgrades

## 4.0.0 → 4.1.0

First Datom/Protos 0.33 adoption. Authored Ethos, library, build script and
canonical examples are unchanged. Eleven generated aliases become tuple
structs with Datom derives. Fixtures construct the local and imported
Router/Signal wrappers around the same values. Checked reader-budget
construction preserves the one-mebibyte extent, depth 256 and exhausted
four-unit refusal test. The generic test helper drops the obsolete
Datomizable associated Output constraint; current Datomizable returns Datom
directly. Assertions and product logic remain unchanged.

Selected cohort: Datom eabead4 / Protos 3c7ade3 (0.33.0), Signal 7afb68b
(10.3.0), ordinary Router 2b65703 (5.1.0), and Ethos Zero 5a5daca (19.2.0).
Registry packages/checksums are retained. No compact normalization is used.

Historical 25a92c6 source with its own tools and the candidate produce
byte-equal archive/frame samples for nine Query values (the five root
operations plus four additional endpoint classes) and all seven Response
roots. Each local frame restores and both directions of typed binary/text
cross-reads succeed. Raw text is equal 11/16; balanced-string/token inspection
verifies layout-only differences in the other five samples. These samples
cover root and endpoint classes, not every nested value/refusal, source-API,
storage or runtime behavior.

Checks cover x86_64-linux only; the other three declared systems are omitted.
The existing Router app's legacy dependency/features remain untouched.
This source release is not a consumer or runtime rollout.
