use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub struct ArenaEvents;

impl ArenaEvents {
    /// Emit arena initialized event
    pub fn arena_initialized(env: &Env, admin: &Address) {
        env.events().publish((symbol_short!("INIT"),), admin);
    }

    /// Emit arena configured event
    pub fn arena_configured(env: &Env) {
        env.events().publish((symbol_short!("CFGD"),), ());
    }

    /// Emit game started event
    pub fn game_started(env: &Env) {
        env.events().publish((symbol_short!("START"),), ());
    }

    /// Emit game finished event
    pub fn game_finished(env: &Env) {
        env.events().publish((symbol_short!("FINISH"),), ());
    }

    /// Emit player joined event
    pub fn player_joined(env: &Env, player: &Address) {
        env.events().publish((symbol_short!("JOIN"),), player);
    }

    /// Emit choice submitted event
    pub fn choice_submitted(env: &Env, player: &Address) {
        env.events().publish((symbol_short!("CHOICE"),), player);
    }

    /// Emit player eliminated event
    pub fn player_eliminated(env: &Env, player: &Address) {
        env.events().publish((symbol_short!("ELIM"),), player);
    }

    /// Emit prize claimed event
    pub fn prize_claimed(env: &Env, winner: &Address) {
        env.events().publish((symbol_short!("CLAIMED"),), winner);
    }

    /// Emit arena cancelled event
    pub fn arena_cancelled(env: &Env) {
        env.events().publish((symbol_short!("CNCL"),), ());
    }

    /// Emit refund claimed event
    pub fn refund_claimed(env: &Env, player: &Address) {
        env.events().publish((symbol_short!("REFUND"),), player);
    }

    /// Emit stake deposited event
    pub fn stake_deposited(env: &Env, amount: i128) {
        env.events().publish((symbol_short!("STK_DEP"),), amount);
    }

    /// Emit stake withdrawn event
    pub fn stake_withdrawn(env: &Env, amount: i128) {
        env.events().publish((symbol_short!("STK_WTH"),), amount);
    }

    /// Emit contract paused event
    pub fn contract_paused(env: &Env, admin: &Address, reason: &Symbol) {
        env.events().publish((symbol_short!("PAUSED"), admin.clone()), reason.clone());
    }

    /// Emit contract unpaused event
    pub fn contract_unpaused(env: &Env, admin: &Address) {
        env.events().publish((symbol_short!("UNPAUS"), admin.clone()), ());
    }

    /// Emit treasury address updated event
    pub fn treasury_updated(env: &Env, admin: &Address, new_treasury: &Address) {
        env.events().publish((symbol_short!("TRSRY"), admin.clone()), new_treasury);
    }

    /// Emit cooldown configured event
    pub fn cooldown_configured(env: &Env, admin: &Address, cooldown_seconds: &u64) {
        env.events().publish((symbol_short!("COOLDN"), admin.clone()), *cooldown_seconds);
    }

    /// Emit creator stake deposited event with creator, amount, and new total
    pub fn creator_stake_deposited(env: &Env, creator: &Address, amount: i128, total: i128) {
        env.events().publish((symbol_short!("CS_DEP"), creator.clone()), (amount, total));
    }

    /// Emit creator stake withdrawn event with creator, amount, and slashed flag
    pub fn creator_stake_withdrawn(env: &Env, creator: &Address, amount: i128, slashed: bool) {
        env.events().publish((symbol_short!("CS_WTH"), creator.clone()), (amount, slashed));
    }

    /// Emit player auto-eliminated event (AFK - did not submit a choice)
    pub fn player_auto_eliminated(env: &Env, player: &Address) {
        env.events().publish((symbol_short!("AFK_ELIM"),), player);
    }
}

