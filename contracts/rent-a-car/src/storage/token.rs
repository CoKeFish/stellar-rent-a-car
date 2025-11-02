use soroban_sdk::{Address, Env};
use crate::storage::types::errors::Error;
use crate::storage::types::storage::DataKey;

pub(crate) fn read_token(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Token)
        .ok_or(Error::TokenNotFound)
}

pub(crate) fn write_token(env: &Env, token: &Address) {
    env.storage()
        .instance()
        .set(&DataKey::Token, &token);
}