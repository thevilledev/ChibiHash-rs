//! Tests for the ChibiHash algorithm
//! Direct hashing

use chibihash::{chibi_hash64, ChibiHasher};
use core::hash::{Hash, Hasher};

#[test]
fn test_basic_hashing() {
    let data = b"Hello, World!";
    let hash1 = chibi_hash64(data, 0);
    let hash2 = chibi_hash64(data, 0);
    assert_eq!(hash1, hash2, "Same input should produce same hash");

    let hash3 = chibi_hash64(data, 1);
    assert_ne!(
        hash1, hash3,
        "Different seeds should produce different hashes"
    );
}

#[test]
fn test_hasher_trait() {
    let mut hasher1 = ChibiHasher::new(0);
    let mut hasher2 = ChibiHasher::new(0);

    "Hello, World!".hash(&mut hasher1);
    "Hello, World!".hash(&mut hasher2);

    assert_eq!(
        hasher1.finish(),
        hasher2.finish(),
        "Same input should produce same hash"
    );

    let mut hasher3 = ChibiHasher::new(1);
    "Hello, World!".hash(&mut hasher3);
    assert_ne!(
        hasher1.finish(),
        hasher3.finish(),
        "Different seeds should produce different hashes"
    );
}

#[cfg(feature = "std")]
mod std_tests {
    use chibihash::{ChibiHashMap, ChibiHashSet};

    #[test]
    fn test_chibi_hash_map() {
        let mut map: ChibiHashMap<String, i32> = ChibiHashMap::default();
        map.insert("hello".to_string(), 42);
        assert_eq!(map.get("hello"), Some(&42));
    }

    #[test]
    fn test_chibi_hash_set() {
        let mut set: ChibiHashSet<String> = ChibiHashSet::default();
        set.insert("hello".to_string());
        assert!(set.contains("hello"));
    }
}
