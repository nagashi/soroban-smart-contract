#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;

        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().bump(50,100);

        count
    }

    /// Return the current value of the internal key.
    pub fn get_current_value(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        log!(&env, "current count: {}", count);

        count
    }

    /// Return the decremented value of the internal key.
    pub fn get_decrement_value(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        if count > 0 {
            count -= 1;
        
            log!(&env, "decrement count: {}", count);

            env.storage().instance().set(&COUNTER, &count);

            env.storage().instance().bump(50,100);
        }
        count
    }

    /// Return the reseted value of the internal counter.
    pub fn reset_value(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        if count > 0 {
            // reset the value of the key.
            count = 0;
        }        

        log!(&env, "reset count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().bump(50,100);

        count
    }    
}

#[cfg(test)]
mod test;