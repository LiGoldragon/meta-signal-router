//! Owner Router channel-authority Interface.
//!
//! `ethos/interface.ethos` is the canonical textual projection of one
//! authority-verified, role-free bootstrap Interface. Shared component and
//! network vocabulary resolves through `signal-standard`; ordinary Router
//! identities resolve through `signal-router`. The checked Rust projection
//! carries encoded identities only.

pub mod bootstrap_manifest;
pub mod schema;

pub const META_ROUTER_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const META_ROUTER_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

pub use schema::lib::*;
