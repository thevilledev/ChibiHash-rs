use chibihash::chibi_hash64;

fn main() {
    let text = b"Hello, World!";
    let hash = chibi_hash64(text, 0);
    println!("Hash of '{}' is: {:016x}", String::from_utf8_lossy(text), hash);
}