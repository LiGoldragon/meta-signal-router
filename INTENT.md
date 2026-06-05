# INTENT — meta-signal-router

*The meta-signal wire contract for PersonaRouter channel policy. Defines the typed
request/reply channel that `persona-orchestrate` uses to grant, extend, revoke,
or deny channel authority in the router.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this meta-signal `meta-signal-router`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `router/INTENT.md`. Ordinary router observation
traffic stays in `signal-router/INTENT.md`.

## Why this repo exists

`meta-signal-router` is the **meta-signal policy signal** for PersonaRouter
channel authority. The caller is `persona-orchestrate`, because Orchestrate owns
Router in the authority graph: Mind decides whether channel policy should change,
orders Orchestrate through `meta-signal-orchestrate`, and Orchestrate
enacts that decision here. Ordinary router observation traffic stays in
`signal-router`; runtime actors, policy evaluation, socket binding, durable grant
tables, and command lowering live in `router`.

## The channel shape

The meta channel carries a deliberately small surface:

- **Requests:** `Grant(ChannelGrant)` (create or replace a live channel grant),
  `Extend(ChannelExtension)` (change a grant's duration),
  `Revoke(ChannelRevocation)` (remove a grant),
  `Deny(AdjudicationDenial)` (close an adjudication request without a grant).
- **Replies:** `ChannelGranted`, `ChannelExtended`, `ChannelRevoked`,
  `AdjudicationDenied`, `ChannelOrderRejected` (understood but rejected by router
  policy), `RequestUnimplemented` (in the contract but not yet implemented).

Policy types: `ChannelEndpoint` names internal component endpoints and external
connection classes (using `signal-persona-origin` vocabulary); `ChannelMessageKind`
names route categories a grant can cover; `ChannelDuration` is the requested
lifetime (one-shot, permanent, or time-bound). Meta-order names (grant,
extension, revocation, denial) are intentionally absent from `ChannelMessageKind`
— those are operations on this contract, not message categories being routed.

## Constraints

- Channel-authority orders live only in the meta-signal contract; ordinary observation
  stays in `signal-router`.
- The wire carries contract-local operation roots only — there is no public
  `Mutate` or `Retract` wrapper. The Sema class is a daemon-side projection.
- Wire enums are closed. No `Unknown` escape hatch.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses — no runtime, no actors, no durable grant tables.
- Every operation and reply round-trips through both rkyv frames and NOTA text.

## Non-ownership

This crate does not own:

- `router` daemon actors, policy evaluation, or socket binding;
- durable channel-grant tables or delivery state;
- command lowering from contract operations to Component Commands;
- ordinary router observation traffic (lives in `signal-router`);
- router-to-Mind adjudication observation (stays in the Mind working contract
  until that relation is deliberately moved).

## See also

- `ARCHITECTURE.md` — contract surface, policy types, and the authority graph.
- `../router/INTENT.md` — daemon-side intent (delivery, channels, adjudication).
- `../signal-router/INTENT.md` — ordinary router observation contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
