//! # Balancer Rust SDK
//!
//! The Balancer Rust SDK is a Rust library that wraps provides a high-level API to interact with Balancer contracts.
//!
//! ## Typed API interfaces
//!
//! The contract APIs are generated from json files in the `src/abis` directory. To view the typed apis,
//! you will need to look at the generated contracts directly (see section: `Methods from Deref<Target = Methods>`):`
//!
//! - Vault API: [`generated_contracts::vault::vault::Contract`]

mod generated_contracts;
mod macros;
mod domain;
mod types;

pub use generated_contracts::erc20::*;
pub use helpers::*;
pub use domain::*;
pub use types::*;

pub mod constants;
pub mod helpers;
pub mod pools;
pub mod vault;
