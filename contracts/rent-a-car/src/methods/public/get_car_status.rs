use soroban_sdk::{Address, Env};
use crate::storage::car::{has_car, read_car};
use crate::storage::types::car_status::CarStatus;
use crate::storage::types::errors::Error;

pub(crate) fn get_car_status(env: &Env, owner: &Address) -> Result<CarStatus, Error> {
    if !has_car(env, &owner) {
        return Err(Error::CarNotFound);
    }

    let car = read_car(env, &owner)?;

    Ok(car.car_status)
}