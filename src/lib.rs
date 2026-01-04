#![cfg_attr(not(feature = "std"), no_std)]

// Default version is `v1` to ensure backwards compatibility
pub use v1::{chibi_hash64, ChibiHasher, StreamingChibiHasher};
#[cfg(any(feature = "std", feature = "hashbrown"))]
pub use v1::{ChibiHashMap, ChibiHashSet};

pub mod v1;
pub mod v2;
