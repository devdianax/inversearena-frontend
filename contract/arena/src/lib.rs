#![no_std]
use soroban_sdk::{contract, contractimpl};

/// Arena contract — manages round lifecycle, player choices, and elimination logic.
///
/// Implementation is open for contribution. See the issue tracker for:
/// - Round state machine (OPEN → CLOSED → RESOLVED → SETTLED)
/// - Commit-reveal choice submission
/// - Minority-wins elimination logic
/// - Admin controls and upgrade timelock
///
/// Architecture overview: see `ARCHITECTURE.md` in the workspace root.
#[contract]
pub struct ArenaContract;

#[contractimpl]
impl ArenaContract {}
