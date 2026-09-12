//! Meta Signal contract for the Router's channel authority.
//!
//! The contract carries channel-policy orders only: grants, extensions,
//! revocations, adjudication denials, and the mirror switch. Orchestrate calls
//! this surface. Mind decides at the cognitive level and orders Orchestrate
//! first; it does not call this contract directly. Ordinary router
//! observations live in `signal-router`.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against a fresh
//! generation.
//!
//! # The wire
//!
//! One order is one [`Signal`] frame carrying the rkyv archive of [`Query`];
//! one answer is one frame of [`Response`]. The contract carries no envelope,
//! no exchange identifier, no lane and no route code: the `Query` and
//! `Response` heads are the discrimination, and the connection is the
//! correlation.
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so an owner router frame is the same Rust type as every
//! other contract's frame.

pub mod generated;
pub use generated::signal::*;

pub use signal::{
    ByteViewable, ComponentKind, HostName, NetworkEndpoint, Restorable, Signal, Signalizable,
};
pub use signal_router::{ChannelIdentifier, EngineIdentifier, TimestampNanos, UnixUserIdentifier};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
