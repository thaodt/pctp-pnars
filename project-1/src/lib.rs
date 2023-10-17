#![deny(missing_docs)]
//! A simple in-memory key/value store (no networked, no persist)

pub use kv::KvStore;
mod kv;
