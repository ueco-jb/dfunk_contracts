use cosmwasm_std::coin;

use super::suite::SuiteBuilder;
use crate::error::ContractError;

#[test]
fn unsupported_denom() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_funds(user, &[coin(50, "BTC")])
        .build();

    let err = suite.deposit(user, &[coin(50, "BTC")]).unwrap_err();
    assert_eq!(
        ContractError::UnsupportedDenom("BTC".to_owned()),
        err.downcast().unwrap()
    );
}

#[test]
fn more_then_one_coin() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_funds(user, &[coin(50, "uluna"), coin(50, "uusd")])
        .build();

    let err = suite
        .deposit(user, &[coin(50, "uluna"), coin(50, "uusd")])
        .unwrap_err();
    assert_eq!(
        ContractError::DepositMoreThenOne {},
        err.downcast().unwrap()
    );
}

#[test]
fn deposit_works() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_funds(user, &[coin(60, "uluna"), coin(4000, "uusd")])
        .build();

    suite.deposit(user, &[coin(50, "uluna")]).unwrap();
    suite.deposit(user, &[coin(3333, "uusd")]).unwrap();

    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(50, "uluna")
    );
    assert_eq!(
        suite.query_deposit(user, "uusd").unwrap(),
        coin(3333, "uusd")
    );
}

#[test]
fn deposit_multiple_times() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_funds(user, &[coin(5000, "uluna")])
        .build();

    suite.deposit(user, &[coin(50, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(50, "uluna")
    );
    suite.deposit(user, &[coin(3000, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(3050, "uluna")
    );
    suite.deposit(user, &[coin(600, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(3650, "uluna")
    );
}
