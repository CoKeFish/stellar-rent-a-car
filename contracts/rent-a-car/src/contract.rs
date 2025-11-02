use crate::interfaces::contract::RentACarContractTrait;
use crate::methods::token::token::token_transfer;
use crate::storage::admin::{read_admin, write_admin};
use crate::storage::car::{read_car, remove_car, write_car};
use crate::storage::contract_balance::{read_contract_balance, write_contract_balance};
use crate::storage::rental::write_rental;
use crate::storage::structs::car::Car;
use crate::storage::structs::rental::Rental;
use crate::storage::token::write_token;
use crate::storage::types::car_status::CarStatus;
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct RentACarContract;

#[contractimpl]
impl RentACarContractTrait for RentACarContract {
    fn __constructor(env: &Env, admin: Address, token: Address) {
        write_admin(env, &admin);
        write_token(env, &token);
    }

    fn get_admin(env: &Env) -> Address {
        read_admin(env)
    }

    fn add_car(env: &Env, owner: Address, price_per_day: i128) {
        let car = Car {
            price_per_day,
            car_status: CarStatus::Available,
            available_to_withdraw: 0,
        };

        write_car(env, &owner, &car);
    }

    fn get_car_status(env: &Env, owner: Address) -> CarStatus {
        let car = read_car(env, &owner);

        car.car_status
    }

    fn rental(env: &Env, renter: Address, owner: Address, total_days_to_rent: u32, amount: i128) {
        renter.require_auth();

        let mut car = read_car(env, &owner);

        car.car_status = CarStatus::Rented;
        car.available_to_withdraw += amount;

        let rental = Rental {
            total_days_to_rent,
            amount,
        };

        let mut contract_balance = read_contract_balance(&env);
        contract_balance += amount;

        write_contract_balance(&env, &contract_balance);
        write_car(env, &owner, &car);
        write_rental(env, &renter, &owner, &rental);

        token_transfer(&env, &renter, &env.current_contract_address(), &amount);
    }

    fn payout_owner(env: &Env, owner: Address, amount: i128) {
        owner.require_auth();

        let mut car = read_car(&env, &owner);
        let mut contract_balance = read_contract_balance(&env);

        car.available_to_withdraw -= amount;
        contract_balance -= amount;

        write_car(&env, &owner, &car);
        write_contract_balance(&env, &contract_balance);

        token_transfer(&env, &env.current_contract_address(), &owner, &amount);
    }

    fn remove_car(env: &Env, owner: Address) {
        remove_car(env, &owner);
    }
}
