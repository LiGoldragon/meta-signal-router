# meta-signal-router

The owner Router Interface for channel-authority policy.

`ethos/interface.ethos` is the sole schema authority. It imports ordinary
Router identities from `signal-router` and shared component/network vocabulary
from `signal-standard`; Rust is one encoded projection, not the source model.

The Interface carries grant, extension, revocation, denial, and mirror-gate
orders from Orchestrate to Router. Ordinary Router observation and forwarding
traffic remains in `signal-router`; daemon state and policy evaluation remain
in `router`.

See `ARCHITECTURE.md` for the contract boundary and `skills.md` before editing.
