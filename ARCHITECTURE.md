# meta-signal-router — architecture

*Meta-signal Signal contract for PersonaRouter channel policy.*

## 0.5 · Direction

`meta-signal-router` is the meta policy contract for PersonaRouter channel authority. The caller is `orchestrate`: Mind decides whether channel policy should change, orders Orchestrate through `meta-signal-orchestrate`, and Orchestrate enacts that decision here. The intended caller is Orchestrate, not Mind; Mind reaches Router channel policy by ordering Orchestrate first. Channel-authority orders live only in this meta-signal contract; ordinary router observation traffic stays in `signal-router`.

## 0 · TL;DR

`meta-signal-router` is the policy signal for
PersonaRouter channel authority. It carries meta-signal orders that
grant, extend, revoke, or deny channel authority in the router.
The caller is PersonaOrchestrate, because Orchestrate owns Router in
the authority graph. Mind decides whether channel policy should
change, then orders Orchestrate through `meta-signal-orchestrate`;
Orchestrate enacts that decision here.

Ordinary router observation traffic stays in `signal-router`.
Router-to-Mind adjudication observation stays in the Mind working
contract until that relation is deliberately moved. Runtime actors,
policy evaluation, socket binding, durable grant tables, and command
lowering live in `router`.

The initial surface is deliberately small:

- `Grant(ChannelGrant)` grants a router channel.
- `Extend(ChannelExtension)` extends an existing router channel.
- `Revoke(ChannelRevocation)` revokes an existing router channel.
- `Deny(AdjudicationDenial)` closes an adjudication request without a
  grant.

## 1 · Contract Surface

| Side | Component |
|---|---|
| Request producer | `orchestrate` meta-signal actor. |
| Request consumer | `router` meta-signal actor. |
| Decision source upstream | `mind`, through `meta-signal-orchestrate`. |

| Operation | Projected Sema class | Meaning |
|---|---|---|
| `Grant` | `Mutate` | Apply meta authority by creating or replacing a live channel grant. |
| `Extend` | `Mutate` | Change the duration of a live channel grant. |
| `Revoke` | `Retract` | Remove a live channel grant. |
| `Deny` | `Mutate` | Record a meta-policy decision that an adjudication request will not receive a grant. |

The Sema classes above are daemon-side projections. The wire carries
contract-local operation roots only; there is no public `Mutate` or
`Retract` wrapper.

| Reply | Meaning |
|---|---|
| `ChannelGranted` | The router accepted and recorded a channel grant. |
| `ChannelExtended` | The router accepted and recorded a channel extension. |
| `ChannelRevoked` | The router accepted and recorded a channel revocation. |
| `AdjudicationDenied` | The router accepted and recorded an adjudication denial. |
| `ChannelOrderRejected` | The order was understood but rejected by router policy. |
| `RequestUnimplemented` | The request is in the contract but not implemented by the current runtime. |

## 2 · Policy Types

`ChannelEndpoint` names internal component endpoints and external
connection classes using `signal-persona-origin` vocabulary.

`ChannelMessageKind` names route categories that can be covered by a
grant. Meta-order names such as channel grant, extension, revocation,
and denial are intentionally absent from this enum; those are
operations on this meta-signal contract, not message categories being
routed through ordinary channels.

`ChannelDuration` is the requested lifetime: one-shot, permanent, or
time-bound.

## 3 · Boundaries

This repo owns:

- meta-signal channel-policy operation roots and payload records;
- meta-signal replies and rejection reasons;
- rkyv and NOTA round-trip shape for the policy signal;
- the contract-local `OperationKind` witness and the `short_header`
  constants, emitted from `schema/lib.schema` by `schema-rust`.

This repo does not own:

- `router` daemon actors;
- router durable grant tables;
- Mind's channel-policy decisions;
- Orchestrate's translation from Mind-level decision to Router-level
  channel order;
- bootstrap policy files;
- ordinary router observation traffic in `signal-router`;
- Mind graph, work graph, or adjudication observation records in
  `signal-mind`;
- CLI argv parsing or socket permissions.

## 4 · Constraints

- The contract exposes meta-signal router channel-policy operations,
  not ordinary router observation queries.
- The intended caller is Orchestrate, not Mind; Mind reaches Router
  channel policy by ordering Orchestrate first.
- Grant, extension, revocation, and denial are meta operations on
  this contract, not message kinds in the routed-channel vocabulary.
- Every operation root is a contract-local verb in verb form.
- The wire shape contains no public Sema wrapper such as `Mutate` or
  `Retract`.
- Channel identifiers are daemon-minted reply data or references to
  existing channels; callers do not mint new channel identifiers for
  grant creation.
- The contract crate contains no runtime actors, database handles,
  sockets, command execution, or policy evaluation logic.

## 5 · Emission

This crate is a real emitting wire contract, not a hand-written
mirror. `schema/lib.schema` is the single source of truth; `build.rs`
runs `schema-rust`'s `GenerationPlan::wire_contract` driver, which
emits the `Input`/`Output` enums, the policy payload records and reply
types, the NOTA codec (gated behind the `nota-text` feature), the
route witnesses, the `short_header` constants, and the
`encode_signal_frame` / `decode_signal_frame` helpers into the
checked-in artifact at `src/schema/lib.rs`. The driver's
`write_or_check` step asserts those artifacts stay byte-identical on
every ordinary build; regenerate them with
`META_SIGNAL_ROUTER_UPDATE_SCHEMA_ARTIFACTS=1 cargo build
--all-features` after any schema edit.

The raw contract frame is a contract-local short header followed by the
rkyv archive. The `router` daemon will carry those bytes through the
triad-runtime length-prefixed process envelope when it binds the meta
listener.

## 6 · Witness Tests

`tests/round_trip.rs` proves:

- operations round-trip through the rkyv archive;
- replies round-trip through the rkyv archive;
- operations and replies round-trip through the emitted signal-frame
  helpers;
- the `short_header` constants are contract-local and distinct;
- operations and replies round-trip through NOTA text and carry
  contract-local verb heads (`(Grant …)`) with no `Mutate`/`Retract`
  wrapper;
- meta-order names are absent from `ChannelMessageKind`.

## Code Map

```text
schema/lib.schema     the meta channel-policy wire-contract schema (source of truth)
build.rs              schema-rust wire_contract generation driver
src/lib.rs            re-exports the generated schema module
src/schema/lib.rs     checked-in generated artifact (do not hand-edit)
tests/round_trip.rs   rkyv + NOTA round trips and contract-local witnesses
```

## See Also

- `../signal-router/ARCHITECTURE.md`
- `../router/ARCHITECTURE.md`
- `../signal-mind/ARCHITECTURE.md`
- `../signal-frame/ARCHITECTURE.md`
- `../signal-sema/ARCHITECTURE.md`
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md`
