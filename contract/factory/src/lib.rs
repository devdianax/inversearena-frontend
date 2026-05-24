#![no_std]
use soroban_sdk::{contract, contractimpl};

/// Factory contract — deploys arena instances and enforces protocol-level rules.
///
/// Implementation is open for contribution. See the issue tracker for:
/// - Pool creation with host whitelist enforcement
/// - Arena WASM hash management
/// - Minimum stake validation
/// - Admin and upgrade timelock flow
///
/// Architecture overview: see `ARCHITECTURE.md` in the workspace root.
#[contract]
pub struct FactoryContract;

#[contractimpl]
impl FactoryContract {}
