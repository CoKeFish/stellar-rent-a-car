use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ContractInitialized = 0,
    ContractNotInitialized = 1,
    CarNotFound = 2,
    AdminTokenConflict = 3,
    ContributionBelowMinimum = 5,
    AmountMustBePositive = 6,
    RentalNotFound = 7,
    InsufficientBalance = 8,
    BalanceNotAvailableForAmountRequested = 9,
    CarAlreadyExist = 10,
    RentalDurationCannotBeZero = 11,
    SelfRentalNotAllowed = 12,
    CarAlreadyRented = 13,
}