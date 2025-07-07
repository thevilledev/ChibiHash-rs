#![cfg_attr(not(feature = "std"), no_std)]

// Default version is `v1` to ensure backwards compatibility
pub use v1::{chibi_hash64, ChibiHashMap, ChibiHashSet, ChibiHasher, StreamingChibiHasher};

pub mod v1;
pub mod v2;
