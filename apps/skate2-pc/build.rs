//! The build's secret: see `../build_secret.rs`. A development build makes it anew whenever
//! anything the executable is built from changes.

#[path = "../build_secret.rs"]
mod build_secret;

fn main() {
    build_secret::emit(&["build.rs", "../build_secret.rs", "Cargo.toml", "src", "../../crates", "../../Cargo.toml", "../../Cargo.lock"]);
}
