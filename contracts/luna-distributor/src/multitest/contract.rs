use cosmwasm_std::{coin, Addr, Decimal};

use super::suite::SuiteBuilder;
use crate::state::{Config, WeightPerProtocol, Whitelist};

#[test]
fn config_query_works() {
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
            owner: suite.owner(),
            burn_address: Addr::unchecked("burnaddress"),
            whitelist,
            weight_per_protocol,
            percent_to_burn: Decimal::from_ratio(7778u128, 10000u128),
            percent_to_distribute: Decimal::from_ratio(2222u128, 10000u128),
        }
    );
}
