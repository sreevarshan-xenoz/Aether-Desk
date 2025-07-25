fn main() {
    // No build script needed for pure Rust application
    println!("cargo:rerun-if-changed=build.rs");
}
