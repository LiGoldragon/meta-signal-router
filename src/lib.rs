//! Meta (owner-only policy) Signal contract for the PersonaRouter
//! channel-authority surface.
//!
//! Wire-only: the rkyv + NOTA codec and the signal-frame mail envelope for
//! the meta channel-policy operations — granting, extending, revoking a
//! router channel, and denying an adjudication request
//! (Grant/Extend/Revoke/Deny). The peer-callable read/observe surface lives
//! in the ordinary `signal-router` contract; runtime actors, policy
//! evaluation, socket binding, and durable grant tables live in the
//! `router` daemon.
//!
//! The wire types, the `Input`/`Output` enums, the route witnesses, the
//! short headers, and the `encode_signal_frame`/`decode_signal_frame` mail
//! envelope are all emitted from `schema/lib.schema` by `schema-rust-next`;
//! the checked-in artifacts live in `src/schema/`.

pub mod schema;

pub use schema::lib::*;
