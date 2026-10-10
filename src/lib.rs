//! aleo-cli — CLI tool for Aleo blockchain
//!
//! This library re-exports the command modules so integration tests can cover them.

pub mod commands;

pub use commands::{balance, deploy, exec, generate, query, record, stablecoin, transfer, verify};
