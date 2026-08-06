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
| `signal-standard` | `ComponentKind`, `HostName`, `NetworkEndpoint` |

The imports are producer identities, not copied declarations or readable Rust
aliases. `build.rs` resolves each producer's Cargo-published Ethos directory,
proves it is the exact source compiled by the pinned dependency, imports its
authority seats, and projects an explicit encoded Rust path for each type.

`ComponentKind` gives internal endpoints the estate-wide component vocabulary.
`NetworkEndpoint` gives network connection classes a structured host and port;
an opaque contract-local peer string is not retained. Other persona engines
combine the ordinary Router engine identity with the shared host identity.

## Authority and projection

`ethos/interface.ethos` is a role-free `Interface.{1 0 0}` and the only schema
source. The three role lists are empty. `MetaRouterRequest` and
`MetaRouterReply` are ordinary declarations because request/reply seating is
behavior at this bootstrap stage, not textual authority smuggled into a role
slot.

`src/bootstrap_manifest.rs` contains the explicit, already-minted authority,
grammar, declaration, variant, and canonical-order seats. `build.rs` constructs
the prior catalog, adds the exact producer seats, authorizes precisely the
manifested transition, revalidates it through Core Ethos/Nomos, and asks Rust
Logos for the checked encoded projection at
`src/schema/lib/generated.rs`.

The generated file contains only encoded coordinates. Visible spellings remain
in Ethos, Dotos, diagnostics, and explicit route behavior; no readable schema
type is copied into Rust.

`src/schema/lib/behavior.rs` owns the behavior not yet expressed by the
bootstrap language:

- structural conversion for local and imported values;
- Dotos encoding and decoding at the text edge;
- rkyv behavior for encoded declarations;
- request/reply route seating;
- Signal framing at contract binding 8, wire revision 2.

The structural adapters translate the common recursive wire shape at producer
boundaries. They do not rename, wrap, or redefine imported identities.

## Boundaries

This repository contains no Router actor, policy evaluator, durable grant
table, socket listener, bootstrap file reader, CLI parser, or command lowering.
Those belong to the `router` runtime. Ordinary observation, forwarding,
session, bootstrap, and actor-registration relations belong to
`signal-router`.

The contract assumes no permanent compiler, host language, transport process,
or operating system. Its durable meaning is the relation expressed in Ethos;
Rust, rkyv, and the current Signal envelope are projections and behavior at the
present machine boundary.

## Verification

The witness suite proves:

- every one of the five requests traverses the bound Signal frame;
- every one of the seven replies traverses Signal and rkyv;
- every root round-trips through Dotos with its visible head;
- canonical Dotos examples are exact;
- imported Router and standard types cross the structural boundary;
- the producer pins and corrected generator revision are exact;
- bootstrap dependencies do not enter the default runtime graph;
- the legacy schema source, emitter vocabulary, Nota edge, and copied
  declarations are absent.

After changing the Interface, update the explicit manifest first and regenerate
with `META_SIGNAL_ROUTER_UPDATE_INTERFACE_ARTIFACTS=1 cargo build
--all-features`. An ordinary build must then prove the checked projection is
fresh without the update variable.
