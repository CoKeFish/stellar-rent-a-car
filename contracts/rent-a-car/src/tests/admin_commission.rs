use soroban_sdk::{testutils::Address as _, Address, IntoVal};
use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use crate::{storage::admin::read_admin_commission, tests::config::contract::ContractTest};

#[test]
pub fn test_set_admin_commission_successfully() {
    let ContractTest { env, contract, admin, .. } = ContractTest::setup();

    let commission = 100_i128;

    contract
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "set_admin_commission",
                args: (commission,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_admin_commission(&commission);

    let stored_commission = env.as_contract(&contract.address, || {
        read_admin_commission(&env)
    });

    assert_eq!(stored_commission, commission);
}

#[test]
pub fn test_set_admin_commission_zero() {
    let ContractTest { env, contract, admin, .. } = ContractTest::setup();

    let commission = 0_i128;

    contract
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "set_admin_commission",
                args: (commission,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_admin_commission(&commission);

    let stored_commission = env.as_contract(&contract.address, || {
        read_admin_commission(&env)
    });

    assert_eq!(stored_commission, commission);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
pub fn test_set_admin_commission_with_negative_value_fails() {
    let ContractTest { contract, env, .. } = ContractTest::setup();
    let commission = -100_i128;

    env.mock_all_auths();

    contract.set_admin_commission(&commission);
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
pub fn test_unauthorized_user_cannot_set_admin_commission() {
    let ContractTest { env, contract, .. } = ContractTest::setup();

    let fake_admin = Address::generate(&env);
    let commission = 100_i128;

    contract
        .mock_auths(&[MockAuth {
            address: &fake_admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "set_admin_commission",
                args: (commission,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .set_admin_commission(&commission);
}

#[test]
pub fn test_update_admin_commission() {
    let ContractTest { env, contract, .. } = ContractTest::setup();

    let first_commission = 100_i128;
    let second_commission = 200_i128;

    env.mock_all_auths();

    contract.set_admin_commission(&first_commission);

    let stored_commission = env.as_contract(&contract.address, || {
        read_admin_commission(&env)
    });
    assert_eq!(stored_commission, first_commission);

    contract.set_admin_commission(&second_commission);

    let stored_commission = env.as_contract(&contract.address, || {
        read_admin_commission(&env)
    });
    assert_eq!(stored_commission, second_commission);
}

