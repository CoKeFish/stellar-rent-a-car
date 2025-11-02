use soroban_sdk::{testutils::Address as _, Address, IntoVal};
use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use crate::tests::config::contract::ContractTest;

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
pub fn test_unauthorized_user_cannot_withdraw_admin_commission() {
    let ContractTest { env, contract, .. } = ContractTest::setup();

    let fake_admin = Address::generate(&env);
    let amount = 100_i128;

    contract
        .mock_auths(&[MockAuth {
            address: &fake_admin,
            invoke: &MockAuthInvoke {
                contract: &contract.address.clone(),
                fn_name: "withdraw_admin_commission",
                args: (amount,).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .withdraw_admin_commission(&amount);
}

