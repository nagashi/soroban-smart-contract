#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct DecrementorContract;

#[contractimpl] 
impl DecrementorContract {
    /// Increment an internal counter and return the value, i.e. current value -1.
    pub fn increment_decrement (env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);

        count += 1;
        
        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().bump(50,100);

        count
    }

    /// Return the decrement value, i.e. -1, of the internal counter.
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
}

#[cfg(test)]
mod test;