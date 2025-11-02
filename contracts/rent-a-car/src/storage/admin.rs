use soroban_sdk::{symbol_short, Address, Env, Symbol};
use crate::storage::types::errors::Error;
use super::types::storage::DataKey;


pub const ADMIN_KEY: &Symbol = &symbol_short!("ADMIN");

pub(crate) fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;

    env.storage().instance().has(&key)
}

pub(crate) fn read_admin(env: &Env) -> Result<Address, Error> {
    let key = DataKey::Admin;

    env.storage().instance().get(&key).ok_or(Error::AdminNotFound)
}


pub(crate) fn write_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;

    env.storage().instance().set(&key, admin);
}

pub(crate) fn read_admin_commission(env: &Env) -> i128 {
    let key = DataKey::AdminCommission;
    
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(0)
}

pub(crate) fn write_admin_commission(env: &Env, commission: i128) {
    let key = DataKey::AdminCommission;
    
    env.storage().instance().set(&key, &commission);
}

pub(crate) fn read_admin_available_to_withdraw(env: &Env) -> i128 {
    let key = DataKey::AdminAvailableToWithdraw;
    
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(0)
}

pub(crate) fn write_admin_available_to_withdraw(env: &Env, amount: i128) {
    let key = DataKey::AdminAvailableToWithdraw;
    
    env.storage().instance().set(&key, &amount);
}