fn main() {
    #[cfg(feature = "ffi")]
    cc::Build::new()
        .file("csrc/chibihash_v1.c")
        .opt_level(3)
        .flag("-march=native")
        .compile("chibihash_v1");
    #[cfg(feature = "ffi")]
    println!("cargo:rerun-if-changed=csrc/chibihash.h");
    #[cfg(feature = "ffi")]
    println!("cargo:rerun-if-changed=csrc/chibihash_v1.c");
}
