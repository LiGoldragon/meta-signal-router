# meta-signal-router architecture

`meta-signal-router` is the owner channel-authority Interface of Router. It is
the narrow relation through which Orchestrate changes which channel paths
Router admits. Mind may decide that policy should change, but Orchestrate owns
the Router relation and produces this Interface's requests.

## Semantic center

A channel grant relates a source endpoint, a destination endpoint, the message
kinds admitted between them, and a lifetime. The Interface can create that
relation, change its lifetime, remove it, or record that an adjudication did not
produce one. A separate owner-only operation controls Router's persisted mirror
gate. These are policy acts on Router, not ordinary routed-message categories.

The operation surface is:

| Request | Meaning |
| --- | --- |
| `Grant` | Install channel authority for an endpoint relation. |
| `Extend` | Replace the lifetime of an existing grant. |
| `Revoke` | Remove an existing grant and carry its reason. |
| `Deny` | Close an adjudication request without a grant. |
| `SetMirrorEnabled` | Set the persisted, owner-only mirror gate. |

Replies confirm the four channel-policy acts and the mirror setting, or return
an explicit policy rejection or implementation refusal. `OperationKind` names
only operations to which those refusals apply. `ChannelMessageKind` deliberately
contains none of the policy verbs.

## Vocabulary ownership

This Interface owns policy-specific relations and distinctions:

- `ChannelGrant`, `ChannelExtension`, `ChannelRevocation`, and
  `AdjudicationDenial`;
- `ConnectionClass`, `ChannelEndpoint`, `ChannelMessageKind`, and
  `ChannelDuration`;
- rejection, unimplemented, request-root, and reply-root declarations.

It imports identities whose meaning is already owned elsewhere:

| Producer | Imported declarations |
| --- | --- |
| `signal-router` | `ChannelIdentifier`, `EngineIdentifier`, `TimestampNanos`, `UnixUserIdentifier` |
| `signal` | `ComponentKind`, `HostName`, `NetworkEndpoint` |

The imports are Ethos imports: the generated Rust writes `signal::ComponentKind`
and the rest fully qualified, so there is one definition and no copy. `signal`
carries `links = "signal"`, which forbids two different sources or revisions of
it in one graph — that key is what enforces a single shared frame type and a
single shared taxonomy across the estate. Every contract in this wave pins the
same source and the same revision.

`ComponentKind` gives internal endpoints the estate-wide component vocabulary.
`NetworkEndpoint` gives network connection classes a structured host and port;
an opaque contract-local peer string is not retained. Other persona engines
combine the ordinary Router engine identity with the shared host identity.

## Authority and projection

`ethos/signal.ethos` is a `Signal` root and the only schema authority. `build.rs`
actualizes it through `ethos-zero` and asserts the checked-in Rust projection in
`src/generated/signal.rs` equals a fresh generation, so the committed code is
the authored schema and nothing else. There is no bootstrap manifest, no
authority transaction, no encoded identity table, and no build-time codegen
beyond that assertion: the generated Rust is readable, and its names are the
schema's own.

`src/lib.rs` re-exports that projection, re-exports the imported identities,
and re-exports the portable frame — `Signal<T>`, `Signalizable`, `ByteViewable`,
`Restorable` — from `signal`, so an owner router frame is the same Rust type as
every other contract's frame. A vendored copy of those four would be a fork of
the wire type.

## The wire

One order is one Signal frame carrying the rkyv archive of `Query`; one answer
is one frame of `Response`. The contract carries no envelope, no exchange
identifier, no lane, no epoch and no route code: the `Query` and `Response`
heads are the discrimination, and the connection is the correlation. The byte
layer — a four-byte big-endian length prefix — belongs to `signal`'s framing and
to the transport, not to this contract.

## Boundary

This crate is a contract. It holds no daemon actor, store table, socket
listener, CLI parser, or command lowering. Ordinary Router observation,
session, forwarding, bootstrap, and actor-registration relations belong to
`signal-router`; policy evaluation and persistence belong to `router`.

## Witnesses

`tests/contract.rs` restores every order and answer from fresh peer bytes,
round-trips every one through Datom text, actualizes every line of
`examples/canonical.datom`, and refuses peer text beyond a one-mebibyte extent.
`examples/canonical.datom` is written by the codec that reads it back, never
by hand.
