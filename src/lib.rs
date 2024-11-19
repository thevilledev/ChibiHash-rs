#[inline(always)]
fn load_u64_le(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes[..8].try_into().unwrap())
}

/// A small, fast 64-bit hash function.
/// 
/// Port of ChibiHash (https://github.com/N-R-K/ChibiHash)
/// 
/// # Arguments
/// 
/// * `key` - Bytes to hash
/// * `seed` - Initial seed value
/// 
/// # Returns
/// 
/// A 64-bit hash value
pub fn chibi_hash64(key: &[u8], seed: u64) -> u64 {
    const P1: u64 = 0x2B7E151628AED2A5;
    const P2: u64 = 0x9E3793492EEDC3F7;
    const P3: u64 = 0x3243F6A8885A308D;

    let mut h = [P1, P2, P3, seed];
    let len = key.len();
    let mut k = key;

    // Process 32-byte chunks
    while k.len() >= 32 {
        for i in 0..4 {
            let lane = load_u64_le(&k[i * 8..]);
            h[i] ^= lane;
            h[i] = h[i].wrapping_mul(P1);
            h[(i + 1) & 3] ^= lane.rotate_left(40);
        }
        k = &k[32..];
    }

    // Add length mix
    h[0] = h[0].wrapping_add(((len as u64) << 32) | ((len as u64) >> 32));

    // Handle single byte if present
    if k.len() & 1 != 0 {
        h[0] ^= k[0] as u64;
        k = &k[1..];
    }
    h[0] = h[0].wrapping_mul(P2);
    h[0] ^= h[0] >> 31;

    // Process remaining 8-byte chunks
    let mut i = 1;
    while k.len() >= 8 {
        h[i] ^= load_u64_le(k);
        h[i] = h[i].wrapping_mul(P2);
        h[i] ^= h[i] >> 31;
        k = &k[8..];
        i += 1;
    }

    // Process remaining 2-byte chunks
    i = 0;
    while k.len() >= 2 {
        h[i] ^= u64::from(k[0]) | (u64::from(k[1]) << 8);
        h[i] = h[i].wrapping_mul(P3);
        h[i] ^= h[i] >> 31;
        k = &k[2..];
        i += 1;
    }

    // Final mixing
    let mut x = seed;
    x ^= h[0].wrapping_mul((h[2] >> 32) | 1);
    x ^= h[1].wrapping_mul((h[3] >> 32) | 1);
    x ^= h[2].wrapping_mul((h[0] >> 32) | 1);
    x ^= h[3].wrapping_mul((h[1] >> 32) | 1);

    // moremur mixing
    x ^= x >> 27;
    x = x.wrapping_mul(0x3C79AC492BA7B653);
    x ^= x >> 33;
    x = x.wrapping_mul(0x1C69B3F74AC4AE35);
    x ^= x >> 27;

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hashing() {
        let data = b"Hello, World!";
        let hash1 = chibi_hash64(data, 0);
        let hash2 = chibi_hash64(data, 0);
        assert_eq!(hash1, hash2, "Same input should produce same hash");
        
        let hash3 = chibi_hash64(data, 1);
        assert_ne!(hash1, hash3, "Different seeds should produce different hashes");
    }
}