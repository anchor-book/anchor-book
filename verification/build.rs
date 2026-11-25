//! Enable ghost annotations only in the private proof build.
fn main() {
    println!("cargo:rustc-cfg=anchorbook_verification");
    println!("cargo:rerun-if-changed=build.rs");
}
