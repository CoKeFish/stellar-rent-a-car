use soroban_sdk::{testutils::Address as _, vec, Address, IntoVal, Symbol};
use crate::{storage::{car::read_car, contract_balance::read_contract_balance, rental::read_rental, types::car_status::CarStatus}, tests::config::contract::ContractTest};
use crate::tests::config::utils::get_contract_events;

#[test]
pub fn test_rental_car_successfully() {
    let ContractTest { env, contract, token, .. } = ContractTest::setup();

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;

    env.mock_all_auths();

    let (_, token_admin, _) = token;

    let amount_mint = 10_000_i128;
    token_admin.mint( &renter, &amount_mint);

    contract.add_car(&owner, &price_per_day);

    let initial_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    assert_eq!(initial_contract_balance, 0);

    contract.rental(&renter, &owner, &total_days, &amount);
    let contract_events = get_contract_events(&env, &contract.address);

    let updated_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    // When commission is 0 (default), total amount equals rental amount
    assert_eq!(updated_contract_balance, amount);

    let car = env.as_contract(&contract.address, || read_car(&env, &owner)).unwrap();
    assert_eq!(car.car_status, CarStatus::Rented);
    assert_eq!(car.available_to_withdraw, amount);

    let rental = env.as_contract(&contract.address, || read_rental(&env, &renter, &owner)).unwrap();
    assert_eq!(rental.total_days_to_rent, total_days);
    assert_eq!(rental.amount, amount);
    assert_eq!(
        contract_events,
        vec![
            &env,
            (
                contract.address.clone(),
                vec![
                    &env,
                    *Symbol::new(&env, "rented").as_val(),
                    renter.clone().into_val(&env),
                    owner.clone().into_val(&env),
                ],
                (total_days, amount).into_val(&env)
            )
        ]
    );
}

#[test]
pub fn test_rental_with_admin_commission() {
    let ContractTest { env, contract, token, .. } = ContractTest::setup();

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;
    let commission = 500_i128;
    let expected_total_amount = amount + commission;

    env.mock_all_auths();

    let (_, token_admin, _) = token;

    let amount_mint = 10_000_i128;
    token_admin.mint(&renter, &amount_mint);

    contract.add_car(&owner, &price_per_day);
    
    // Set admin commission
    contract.set_admin_commission(&commission);

    let initial_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    assert_eq!(initial_contract_balance, 0);

    contract.rental(&renter, &owner, &total_days, &amount);

    let updated_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    // Contract balance should include both amount and commission
    assert_eq!(updated_contract_balance, expected_total_amount);

    let car = env.as_contract(&contract.address, || read_car(&env, &owner)).unwrap();
    assert_eq!(car.car_status, CarStatus::Rented);
    // Owner should receive the full rental amount (commission is added to deposit, not deducted)
    assert_eq!(car.available_to_withdraw, amount);

    let rental = env.as_contract(&contract.address, || read_rental(&env, &renter, &owner)).unwrap();
    assert_eq!(rental.total_days_to_rent, total_days);
    assert_eq!(rental.amount, amount);
}

#[test]
pub fn test_rental_with_zero_commission() {
    let ContractTest { env, contract, token, .. } = ContractTest::setup();

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;

    env.mock_all_auths();

    let (_, token_admin, _) = token;

    let amount_mint = 10_000_i128;
    token_admin.mint(&renter, &amount_mint);

    contract.add_car(&owner, &price_per_day);
    
    // Set commission to zero (default behavior)
    contract.set_admin_commission(&0_i128);

    contract.rental(&renter, &owner, &total_days, &amount);

    let updated_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    // When commission is 0, total amount equals rental amount
    assert_eq!(updated_contract_balance, amount);

    let car = env.as_contract(&contract.address, || read_car(&env, &owner)).unwrap();
    // Owner should receive full amount when commission is 0
    assert_eq!(car.available_to_withdraw, amount);
}

#[test]
pub fn test_rental_with_commission_added_to_deposit() {
    let ContractTest { contract, env, token, .. } = ContractTest::setup();

    let owner = Address::generate(&env);
    let renter = Address::generate(&env);
    let price_per_day = 1500_i128;
    let total_days = 3;
    let amount = 4500_i128;
    let commission = 5000_i128; // Commission can be any value, it's added to deposit

    env.mock_all_auths();

    let (_, token_admin, _) = token;

    let amount_mint = 20_000_i128; // Enough to cover amount + commission
    token_admin.mint(&renter, &amount_mint);

    contract.add_car(&owner, &price_per_day);
    
    // Set commission (even if greater than amount, it just gets added to deposit)
    contract.set_admin_commission(&commission);

    let initial_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    assert_eq!(initial_contract_balance, 0);

    contract.rental(&renter, &owner, &total_days, &amount);

    let updated_contract_balance = env.as_contract(&contract.address, || read_contract_balance(&env));
    // Contract balance should include amount + commission
    assert_eq!(updated_contract_balance, amount + commission);

    let car = env.as_contract(&contract.address, || read_car(&env, &owner)).unwrap();
    // Owner receives full rental amount
    assert_eq!(car.available_to_withdraw, amount);
}