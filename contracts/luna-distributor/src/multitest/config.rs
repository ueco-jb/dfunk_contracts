use cosmwasm_std::{Addr, Decimal};

use super::suite::SuiteBuilder;
use crate::error::ContractError;
use crate::state::{Config, WeightPerProtocol, Whitelist};

#[test]
fn query() {
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("curve", 30), ("multichain", 20)])
        .build();

    let whitelist = vec![
        Whitelist {
            address: Addr::unchecked("contract1"),
            protocol: "terraswap".to_owned(),
        },
        Whitelist {
            address: Addr::unchecked("contract2"),
            protocol: "curve".to_owned(),
        },
        Whitelist {
            address: Addr::unchecked("contract3"),
            protocol: "multichain".to_owned(),
        },
    ];
    let weight_per_protocol = vec![
        WeightPerProtocol {
            protocol: "terraswap".to_owned(),
            weight: Decimal::percent(50),
        },
        WeightPerProtocol {
            protocol: "curve".to_owned(),
            weight: Decimal::percent(30),
        },
        WeightPerProtocol {
            protocol: "multichain".to_owned(),
            weight: Decimal::percent(20),
        },
    ];

    let res = suite.query_config().unwrap();
    assert_eq!(
        res,
        Config {
            admin: suite.owner().to_string(),
            burn_address: Addr::unchecked(suite.burn_address()),
            developer_address: Addr::unchecked(suite.developer_address()),
            whitelist,
            weight_per_protocol,
            percent_to_burn: Decimal::percent(70),
            percent_to_developer: Decimal::percent(10),
            percent_to_distribute: Decimal::percent(20),
        }
    );
}

#[test]
fn update() {
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("curve", 30), ("multichain", 20)])
        .build();

    let new_whitelist = vec![
        crate::msg::Whitelist {
            address: "contract1".to_owned(),
            protocol: "terraswap".to_owned(),
        },
        crate::msg::Whitelist {
            address: "contract2".to_owned(),
            protocol: "some_other_protocol".to_owned(),
        },
        crate::msg::Whitelist {
            address: "contract33".to_owned(),
            protocol: "multichain".to_owned(),
        },
    ];
    let new_weight_per_protocol = vec![
        crate::msg::WeightPerProtocol {
            protocol: "terraswap".to_owned(),
            weight: Decimal::percent(60),
        },
        crate::msg::WeightPerProtocol {
            protocol: "some_other_protocol".to_owned(),
            weight: Decimal::percent(23),
        },
        crate::msg::WeightPerProtocol {
            protocol: "multichain".to_owned(),
            weight: Decimal::percent(17),
        },
    ];

    let owner = suite.owner();
    suite
        .update_config(
            owner.as_str(),
            None,
            None,
            new_whitelist,
            new_weight_per_protocol,
        )
        .unwrap();

    let whitelist = vec![
        Whitelist {
            address: Addr::unchecked("contract1"),
            protocol: "terraswap".to_owned(),
        },
        Whitelist {
            address: Addr::unchecked("contract2"),
            protocol: "some_other_protocol".to_owned(),
        },
        Whitelist {
            address: Addr::unchecked("contract33"),
            protocol: "multichain".to_owned(),
        },
    ];
    let weight_per_protocol = vec![
        WeightPerProtocol {
            protocol: "terraswap".to_owned(),
            weight: Decimal::percent(60),
        },
        WeightPerProtocol {
            protocol: "some_other_protocol".to_owned(),
            weight: Decimal::percent(23),
        },
        WeightPerProtocol {
            protocol: "multichain".to_owned(),
            weight: Decimal::percent(17),
        },
    ];

    let res = suite.query_config().unwrap();
    assert_eq!(
        res,
        Config {
            admin: suite.owner().to_string(),
            burn_address: Addr::unchecked(suite.burn_address()),
            developer_address: Addr::unchecked(suite.developer_address()),
            whitelist,
            weight_per_protocol,
            percent_to_burn: Decimal::percent(70),
            percent_to_developer: Decimal::percent(10),
            percent_to_distribute: Decimal::percent(20),
        }
    );
}

#[test]
fn update_unauthorized() {
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("curve", 30), ("multichain", 20)])
        .build();

    let err = suite
        .update_config("someone_else", None, None, None, None)
        .unwrap_err();
    assert_eq!(ContractError::Unauthorized {}, err.downcast().unwrap());
}

#[test]
fn update_with_empty_admin() {
    let mut suite = SuiteBuilder::new()
        .with_admin("".to_owned())
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("curve", 30), ("multichain", 20)])
        .build();

    let owner = suite.owner();
    let err = suite
        .update_config(owner.as_str(), None, None, None, None)
        .unwrap_err();
    assert_eq!(
        ContractError::ConfigNotUpdatable {},
        err.downcast().unwrap()
    );
}

#[test]
fn update_to_empty_admin() {
    let mut suite = SuiteBuilder::new()
        .with_whitelist(&[
            ("contract1", "terraswap"),
            ("contract2", "curve"),
            ("contract3", "multichain"),
        ])
        .with_weights_per_protocol(&[("terraswap", 50), ("curve", 30), ("multichain", 20)])
        .build();

    let owner = suite.owner();
    // first update to empty admin ""
    suite
        .update_config(owner.as_str(), Some("".to_owned()), None, None, None)
        .unwrap();

    assert!(matches!(suite.query_config().unwrap(),
            Config {
                admin, ..
            } if admin.is_empty()));

    let err = suite
        .update_config(owner.as_str(), None, None, None, None)
        .unwrap_err();
    assert_eq!(
        ContractError::ConfigNotUpdatable {},
        err.downcast().unwrap()
    );
}
