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

pub mod generated;
pub use generated::signal::*;

pub use signal_router::{
    ChannelIdentifier, EngineIdentifier, HostName, TimestampNanos, UnixUserIdentifier,
};

use std::marker::PhantomData;

use rkyv::{
    Archive, Deserialize, Serialize,
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    de::Pool,
    rancor::{Error, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");

/// A portable rkyv Signal frame whose target contract is carried in its type.
pub struct Signal<T> {
    bytes: Vec<u8>,
    target: PhantomData<fn() -> T>,
}

/// Data that can form a portable Signal frame.
pub trait Signalizable: Sized {
    fn signalize(&self) -> Result<Signal<Self>, Error>;
}

/// A frame exposes its peer-wire bytes for transport framing.
pub trait ByteViewable {
    fn bytes(&self) -> &[u8];
}

/// A typed portable Signal can restore the contract value it carries.
pub trait Restorable<T> {
    fn restore(&self) -> Result<T, Error>;
}

impl<T> Signalizable for T
where
    T: for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, Error>>,
{
    fn signalize(&self) -> Result<Signal<Self>, Error> {
        Ok(Signal::from(rkyv::to_bytes::<Error>(self)?.to_vec()))
    }
}

impl<T> From<Vec<u8>> for Signal<T> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            target: PhantomData,
        }
    }
}

impl<T> ByteViewable for Signal<T> {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl<T> Restorable<T> for Signal<T>
where
    T: Archive,
    T::Archived:
        for<'a> CheckBytes<HighValidator<'a, Error>> + Deserialize<T, Strategy<Pool, Error>>,
{
    fn restore(&self) -> Result<T, Error> {
        rkyv::from_bytes::<T, Error>(self.bytes())
    }
}
