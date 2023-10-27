#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct GetCurrentValueContract;

#[contractimpl]
impl GetCurrentValueContract {
    /// Increment an internal counter; return the new value.
    pub fn increment_current_value(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;

        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().bump(50,100);

        count
    }

    /// Return the current value of the internal counter.
    pub fn get_current_value(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        log!(&env, "current count: {}", count);

        count
    }    
}

#[cfg(test)]
mod test;