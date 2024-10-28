fn main() {
    println!("cargo:rustc-link-arg=-Tprototype/linker.ld");
    println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=+avx,-avx512f");
}

