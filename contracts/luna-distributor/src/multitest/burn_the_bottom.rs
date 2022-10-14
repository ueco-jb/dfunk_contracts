use cosmwasm_std::{coin, Uint128};

use super::suite::SuiteBuilder;

#[test]
fn balance_not_enough() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_funds(user, &[coin(100_000_000, "uluna")])
        .build();

    suite.deposit(user, &[coin(100_000_000, "uluna")]).unwrap();

    assert_eq!(suite.query_contract_balance("uluna").unwrap(), 100_000_000);

    suite
        .burn_the_bottom(user, Uint128::new(1_000), "uluna")
        .unwrap();

    // nothing changed
    assert_eq!(suite.query_contract_balance("uluna").unwrap(), 100_000_000);
}

#[test]
fn balance_less_then() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_funds(user, &[coin(100_000_000, "uluna")])
        .build();

    suite.deposit(user, &[coin(700, "uluna")]).unwrap();

    assert_eq!(suite.query_contract_balance("uluna").unwrap(), 700);

    suite
        .burn_the_bottom(user, Uint128::new(1_000), "uluna")
        .unwrap();

    // nothing changed
    assert_eq!(suite.query_contract_balance("uluna").unwrap(), 0);
}

#[test]
fn balance_equal() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_funds(user, &[coin(100_000_000, "uluna")])
        .build();

    suite.deposit(user, &[coin(1_000, "uluna")]).unwrap();

    assert_eq!(suite.query_contract_balance("uluna").unwrap(), 1_000);

    suite
        .burn_the_bottom(user, Uint128::new(1_000), "uluna")
        .unwrap();

    // nothing changed
    assert_eq!(suite.query_contract_balance("uluna").unwrap(), 0);
}
