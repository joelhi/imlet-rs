// build.rs
use std::env;

fn main() {
    // Check if the "viewer" feature is enabled
    if env::var("CARGO_FEATURE_VIEWER").is_ok() {
        // Apply higher optimization for viewer feature
        println!("cargo:rustc-env=RUSTFLAGS=-C opt-level=3");
    } else {
        // Default optimization level
        println!("cargo:rustc-env=RUSTFLAGS=-C opt-level=1");
    }
}