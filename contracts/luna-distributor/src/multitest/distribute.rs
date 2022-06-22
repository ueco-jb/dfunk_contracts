use cosmwasm_std::{coin, Attribute, Event};

use super::suite::SuiteBuilder;
use crate::error::ContractError;

#[test]
fn distribute_works() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("curve", 30), ("multichain", 20)])
        .with_funds(user, &[coin(100_000_000, "uluna")])
        .build();

    suite.deposit(user, &[coin(100_000_000, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(100_000_000, "uluna")
    );

    let response = suite.distribute(user, "uluna").unwrap().events;
    assert_eq!(
        response[1],
        Event::new("transfer").add_attributes(vec![
            Attribute {
                key: "recipient".to_owned(),
                value: suite.burn_address(),
            },
            Attribute {
                key: "sender".to_owned(),
                value: suite.contract(),
            },
            Attribute {
                key: "amount".to_owned(),
                value: "77780000uluna".to_owned(), // 77.78% of 100_000_000
            }
        ])
    );
    assert_eq!(
        response[2],
        Event::new("transfer").add_attributes(vec![
            Attribute {
                key: "recipient".to_owned(),
                value: "contract1".to_owned(),
            },
            Attribute {
                key: "sender".to_owned(),
                value: suite.contract(),
            },
            Attribute {
                key: "amount".to_owned(),
                value: "11110000uluna".to_owned(), // 22.22% of 100_000_000 times 0.5 weight
            }
        ])
    );
    assert_eq!(
        response[3],
        Event::new("transfer").add_attributes(vec![
            Attribute {
                key: "recipient".to_owned(),
                value: "contract2".to_owned(),
            },
            Attribute {
                key: "sender".to_owned(),
                value: suite.contract(),
            },
            Attribute {
                key: "amount".to_owned(),
                value: "6666000uluna".to_owned(), // 22.22% of 100_000_000 times 0.3 weight
            }
        ])
    );
    assert_eq!(
        response[4],
        Event::new("transfer").add_attributes(vec![
            Attribute {
                key: "recipient".to_owned(),
                value: "contract3".to_owned(),
            },
            Attribute {
                key: "sender".to_owned(),
                value: suite.contract(),
            },
            Attribute {
                key: "amount".to_owned(),
                value: "4444000uluna".to_owned(), // 22.22% of 100_000_000 times 0.2 weight
            }
        ])
    );
}

#[test]
fn missing_protocol_weight() {
    let user = "user";
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("multichain", 20)])
        .with_funds(user, &[coin(100_000_000, "uluna")])
        .build();

    suite.deposit(user, &[coin(100_000_000, "uluna")]).unwrap();
    assert_eq!(
        suite.query_deposit(user, "uluna").unwrap(),
        coin(100_000_000, "uluna")
    );

    let err = suite.distribute(user, "uluna").unwrap_err();
    assert_eq!(
        ContractError::MissingProtocol("curve".to_owned()),
        err.downcast().unwrap()
    );
}
