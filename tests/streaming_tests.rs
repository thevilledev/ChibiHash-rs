//! Tests for the ChibiHash algorithm
//! Streaming hashing

use chibihash::{chibi_hash64, StreamingChibiHasher};

#[test]
// Tested against a Github comment from the original ChibiHash author
// See https://github.com/N-R-K/ChibiHash/issues/1#issuecomment-2486086163
fn test_streaming_matches_direct() {
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
    test_streaming(b"qwertyuiopasdfghjklzxcvbnm123456789", 0, 0x0F56CF3735FFA943);

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
