//! Build script for the Windows Rust Driver crate.
//!
//! Based on the [`wdk_build::Config`] parsed from the build tree, this build
//! script will provide `Cargo` with the necessary information to build the
//! driver binary (ex. linker flags)

use std::env;

fn main() -> Result<(), wdk_build::ConfigError> {
    if env::var("CARGO_FEATURE_KERNEL").is_ok() {
        wdk_build::configure_wdk_library_build()
    } else {
        Ok(())
    }
}
