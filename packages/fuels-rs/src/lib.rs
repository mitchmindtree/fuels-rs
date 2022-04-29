//! # Fuel Rust SDK.
//!
//! ## Quickstart: `prelude`
//!
//! A prelude is provided which imports all the important data types and traits for you. Use this when you want to quickly bootstrap a new project.
//!
//! ```no_run
//! # #[allow(unused)]
//! use fuels_rs::prelude::*;
//! use fuels_abigen_macro::abigen;
//! ```
//!
//! Note that `fuels_abigen_macro` isn't included in the `fuels_rs` crate because
//! it is a `proc_macro` package.
//!
//! Examples on how you can use the types imported by the prelude can be found in
//! the [main test suite](https://github.com/FuelLabs/fuels-rs/blob/master/fuels-abigen-macro/tests/harness.rs)

pub mod contract {
    pub use fuels_contract::*;
}

pub mod core {
    pub use fuels_core::*;
}

pub mod signers {
    pub use fuels_signers::*;
}

pub mod test_helpers {
    pub use fuels_test_helpers::*;
}

/// Easy imports of frequently used
#[doc(hidden)]
pub mod prelude {
    //! The fuels-rs prelude
    //!
    //! The purpose of this module is to alleviate imports of many common types:
    //!
    //! ```
    //! # #![allow(unused_imports)]
    //! use fuels_rs::prelude::*;
    //! ```

    pub use super::contract::contract::Contract;
    pub use super::contract::parameters::*;
    pub use super::core::constants::*;
    pub use super::core::errors::Error;
    pub use super::core::{Token, Tokenizable};
    pub use super::signers::provider::*;
    pub use super::signers::{LocalWallet, Signer};
    pub use super::test_helpers::*;
}

/// Expose the minimum supported rustc version as a semver string. E.g. `1.58.0`.
///
/// This allows downstream tooling (i.e. `forc`) to generate test harness manifest files with an
/// equivalent `rust-version = <version>` field, ensuring that users are notified in the case that
/// their locally installed rustc version is out of date.
///
/// The version is detected by including the `fuels` `Cargo.toml` as a constant string slice and
/// manually finding and parsing the line with the assigned rust-version.
pub fn min_supported_rust_version() -> &'static str {
    const CARGO_MANIFEST_TOML: &str = include_str!("../Cargo.toml");
    const RUST_VERSION_PREFIX: &str = "rust-version =";
    CARGO_MANIFEST_TOML
        .lines()
        .find(|line| line.starts_with(RUST_VERSION_PREFIX))
        .and_then(|line| line[RUST_VERSION_PREFIX.len()..].split('"').nth(1))
        .expect("unable to parse `rust-version` field from fuels manifest")
}

#[test]
fn min_supported_rust_version_exists_and_is_valid_semver() {
    let s = min_supported_rust_version();
    semver::Version::parse(s).unwrap();
}
