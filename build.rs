fn main() {
    cc::Build::new()
        .file("csrc/chibihash.c")
        .opt_level(3)
        .flag("-march=native")
        .compile("chibihash");
    println!("cargo:rerun-if-changed=csrc/chibihash.h");
    println!("cargo:rerun-if-changed=csrc/chibihash.c");
}