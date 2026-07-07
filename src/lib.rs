//! Schema-derived Signal contract for the live `schema` component.
//!
//! `schema/lib.schema` is the source of truth. The checked-in
//! `src/schema/lib.rs` file is a freshness-checked `schema-rust` artifact.
//! Runtime state, actors, storage, and current-stack parse/emission bridge code
//! live outside this contract crate.

#[allow(dead_code)]
#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

pub const SIGNAL_SCHEMA_SOURCE: &str = include_str!("../schema/lib.schema");
pub const SIGNAL_SCHEMA_RUST_SOURCE: &str = include_str!("schema/lib.rs");
