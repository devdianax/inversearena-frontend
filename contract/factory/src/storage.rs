use soroban_sdk::{Address, contracttype};

#[contracttype]
pub enum DataKey {
    CreatorStake(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct CreatorStakeRecord {
    pub creator: Address,
    pub amount: i128,
}

#[allow(dead_code)]
pub struct FactoryStorage;

impl FactoryStorage {}
