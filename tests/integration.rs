//! Integration tests for the crate
//! This ensures that the default implementation works as expected
//! It will use the `v1` version of the algorithm
use chibihash::{chibi_hash64, ChibiHasher, StreamingChibiHasher, ChibiHashMap, ChibiHashSet};
use core::hash::{Hash, Hasher};

#[test]
fn test_default_basic_hashing() {
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
fn test_default_hasher_trait() {
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

#[test]
fn test_default_streaming_matches_direct() {
    // Helper function to test streaming vs direct and verify known values
    fn test_streaming(input: &[u8], seed: u64, expected: u64) {
        let direct = chibi_hash64(input, seed);
        assert_eq!(
            direct, expected,
            "Direct hash mismatch for input: {:?}, seed: {}, got: {:016X}, expected: {:016X}",
            input, seed, direct, expected
        );

        let mut streaming = StreamingChibiHasher::new(seed);
        streaming.update(input);
        let streaming_result = streaming.finalize();
        assert_eq!(
            streaming_result, expected,
            "Streaming hash mismatch for input: {:?}, seed: {}, got: {:016X}, expected: {:016X}",
            input, seed, streaming_result, expected
        );
    }

    // Test all the known values
    test_streaming(b"", 0, 0x9EA80F3B18E26CFB);
    test_streaming(b"", 55555, 0x2EED9399FC4AC7E5);
    test_streaming(b"hi", 0, 0xAF98F3924F5C80D6);
    test_streaming(b"123", 0, 0x893A5CCA05B0A883);
    test_streaming(b"abcdefgh", 0, 0x8F922660063E3E75);
    test_streaming(b"Hello, world!", 0, 0x5AF920D8C0EBFE9F);
    test_streaming(b"qwertyuiopasdfghjklzxcvbnm123456", 0, 0x2EF296DB634F6551);
    test_streaming(
        b"qwertyuiopasdfghjklzxcvbnm123456789",
        0,
        0x0F56CF3735FFA943,
    );

    // Also test splitting the input in different ways
    let mut streaming = StreamingChibiHasher::new(0);
    streaming.update(b"Hello, ");
    streaming.update(b"world!");
    assert_eq!(
        streaming.finalize(),
        0x5AF920D8C0EBFE9F,
        "Split streaming should match known value"
    );
}

#[test]
fn test_default_chibi_hash_map() {
    let mut map: ChibiHashMap<String, i32> = ChibiHashMap::default();
    map.insert("hello".to_string(), 42);
    assert_eq!(map.get("hello"), Some(&42));
}

#[test]
fn test_default_chibi_hash_set() {
    let mut set: ChibiHashSet<String> = ChibiHashSet::default();
    set.insert("hello".to_string());
    assert!(set.contains("hello"));
}