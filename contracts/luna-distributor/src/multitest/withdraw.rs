use cosmwasm_std::{coin, Uint128};

use super::suite::SuiteBuilder;
use crate::error::ContractError;

#[test]
fn withdraw_works() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_funds(user, &[coin(50, "uluna")])
        .build();

    suite.deposit(user, &[coin(50, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(50, "uluna")
    );
    assert_eq!(
        suite.query_user_balance(user, "uluna").unwrap(),
        Uint128::zero()
    );

    suite.withdraw(user, "uluna", None).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(0, "uluna")
    );
    assert_eq!(
        suite.query_user_balance(user, "uluna").unwrap(),
        Uint128::new(50)
    );
}

#[test]
fn no_balance() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_funds(user, &[coin(50, "uluna")])
        .build();

    suite.deposit(user, &[coin(50, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(50, "uluna")
    );

    suite.withdraw(user, "uluna", None).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(0, "uluna")
    );

    let err = suite.withdraw(user, "uluna", None).unwrap_err();
    assert_eq!(ContractError::NoBalance {}, err.downcast().unwrap());
}
