fn main() {
    #[cfg(feature = "ffi")]
    cc::Build::new()
        .file("csrc/chibihash.c")
        .opt_level(3)
        .flag("-march=native")
        .compile("chibihash");
    #[cfg(feature = "ffi")]
    println!("cargo:rerun-if-changed=csrc/chibihash.h");
    #[cfg(feature = "ffi")]
    println!("cargo:rerun-if-changed=csrc/chibihash.c");
}