use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    ContractBalance,
    AdminCommission,
    AdminAvailableToWithdraw,
    Car(Address),
    Rental(Address, Address),
}