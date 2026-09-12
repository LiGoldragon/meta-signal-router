# meta-signal-router

The meta Router Signal contract for channel-authority policy.

`ethos/signal.ethos` is the sole schema authority. It imports ordinary Router
identities from `signal-router` and declares the component and network
vocabulary it needs. `build.rs` actualizes the ethos through `ethos-zero` and
asserts the checked-in projection in `src/generated/signal.rs` equals a fresh
generation. `examples/canonical.datom` carries one Datom line per contract
head, every one of them written by the codec that reads it back.

The contract carries grant, extension, revocation, denial, and mirror-gate
orders from Orchestrate to Router. Ordinary Router observation and forwarding
traffic remains in `signal-router`; daemon state and policy evaluation remain
in `router`.

One order is one Signal frame carrying the rkyv archive of `Query`; one answer
is one frame of `Response`. There is no envelope.

See `ARCHITECTURE.md`.
