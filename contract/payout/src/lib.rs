#![no_std]
use soroban_sdk::{contract, contractimpl};

/// Payout contract — records and distributes winnings to the last surviving player.
///
/// Implementation is open for contribution. See the issue tracker for:
/// - Idempotent payout execution (principal + accumulated yield)
/// - Winner verification against arena settlement state
/// - Admin-gated distribution controls
/// - Payout lookup helpers for off-chain reconciliation
///
/// Architecture overview: see `ARCHITECTURE.md` in the workspace root.
#[contract]
pub struct PayoutContract;

#[contractimpl]
impl PayoutContract {}
