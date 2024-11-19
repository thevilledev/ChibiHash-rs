use chibihash::{chibi_hash64, ChibiHasher, ChibiHashError};
use std::hash::Hasher;

fn main() -> Result<(), ChibiHashError> {
    let key = b"Hello, World!";
    let seed = 1337;
    let hash = chibi_hash64(key, seed)?;
    println!("Hash of '{}' is: {:016x}", String::from_utf8_lossy(key), hash);

    let mut hasher = ChibiHasher::new(seed);
    hasher.write(key);
    println!("{:016x}", hasher.finish());
    Ok(())
}
