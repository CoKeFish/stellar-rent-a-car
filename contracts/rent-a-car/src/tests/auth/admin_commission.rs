use soroban_sdk::{testutils::Address as _, Address, IntoVal};
use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use crate::tests::config::contract::ContractTest;

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

