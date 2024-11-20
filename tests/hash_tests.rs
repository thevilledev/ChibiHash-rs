//! Tests for the ChibiHash algorithm
//! Direct hashing

use chibihash::{chibi_hash64, ChibiHasher, StreamingChibiHasher};
use std::hash::{Hash, Hasher};

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

#[test]
// Tested against a Github comment from the original ChibiHash author
// See https://github.com/N-R-K/ChibiHash/issues/1#issuecomment-2486086163
fn test_streaming_matches_direct() {
    // Helper function to test streaming vs direct
    fn test_streaming(input: &[u8], seed: u64) {
        let direct = chibi_hash64(input, seed);
        
        let mut streaming = StreamingChibiHasher::new(seed);
        streaming.update(input);
        let streaming_result = streaming.finalize();
        
        assert_eq!(
            direct, streaming_result,
            "Streaming and direct hashing mismatch for input: {:?}, seed: {}",
            input, seed
        );
    }

    // Test all the real-world examples
    test_streaming(b"", 0);
    test_streaming(b"", 55555);
    test_streaming(b"hi", 0);
    test_streaming(b"123", 0);
    test_streaming(b"abcdefgh", 0);
    test_streaming(b"Hello, world!", 0);
    test_streaming(b"qwertyuiopasdfghjklzxcvbnm123456", 0);
    test_streaming(b"qwertyuiopasdfghjklzxcvbnm123456789", 0);

    // Also test splitting the input in different ways
    let mut streaming = StreamingChibiHasher::new(0);
    streaming.update(b"Hello, ");
    streaming.update(b"world!");
    assert_eq!(
        streaming.finalize(),
        chibi_hash64(b"Hello, world!", 0),
        "Split streaming should match direct"
    );
}
