use anyhow::{anyhow, Result as AnyResult};
use schemars::JsonSchema;
use std::fmt;

use cosmwasm_std::{
    testing::{mock_env, MockApi, MockStorage},
    Addr, Coin, Decimal, Empty, Uint128,
};
use terra_multi_test::{
    App, AppBuilder, AppResponse, BankKeeper, Contract, ContractWrapper, Executor,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, WeightPerProtocol, Whitelist};
use crate::state::Config;

pub fn contract_distributor<C>() -> Box<dyn Contract<C>>
where
    C: Clone + fmt::Debug + PartialEq + JsonSchema + 'static,
{
    let contract = ContractWrapper::new_with_empty(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

// fn contract_distributor() -> Box<dyn Contract<Empty>> {
//     let contract = ContractWrapper::new(
//         crate::contract::execute,
//         crate::contract::instantiate,
//         crate::contract::query,
//     );
//
//     Box::new(contract)
// }

/// Builder for test suite
#[derive(Debug)]
pub struct SuiteBuilder {
    pub burn_address: String,
    pub whitelist: Vec<Whitelist>,
    pub weight_per_protocol: Vec<WeightPerProtocol>,
    pub funds: Vec<(Addr, Vec<Coin>)>,
}

impl SuiteBuilder {
    pub fn new() -> Self {
        Self {
            burn_address: "burnaddress".to_owned(),
            whitelist: vec![],
            weight_per_protocol: vec![],
            funds: vec![],
        }
    }

    pub fn with_whitelist(mut self, whitelist: &[(&str, &str)]) -> Self {
        let mut list = vec![];
        for (address, protocol) in whitelist {
            let entry = Whitelist {
                address: address.to_string(),
                protocol: protocol.to_string(),
            };
            list.push(entry);
        }
        self.whitelist = list;
        self
    }

    pub fn with_weights_per_protocol(mut self, list: &[(&str, u64)]) -> Self {
        let mut weights = vec![];
        for (protocol, weight) in list {
            let entry = WeightPerProtocol {
                protocol: protocol.to_string(),
                weight: Decimal::percent(*weight),
            };
            weights.push(entry);
        }
        self.weight_per_protocol = weights;
        self
    }

    /// Sets initial amount of distributable tokens on address
    pub fn with_funds(mut self, addr: &str, funds: &[Coin]) -> Self {
        self.funds.push((Addr::unchecked(addr), funds.into()));
        self
    }

    #[track_caller]
    pub fn build(self) -> Suite {
        let mut app: App = AppBuilder::new().build();

        let owner = Addr::unchecked("cosmos1dl34yx429w7q5e68tlc9w3ahgycqc7edff0edu");

        let distributor_id = app.store_code(contract_distributor());
        let distributor_contract = app
            .instantiate_contract(
                distributor_id,
                owner.clone(),
                &InstantiateMsg {
                    burn_address: self.burn_address,
                    whitelist: self.whitelist,
                    weight_per_protocol: self.weight_per_protocol,
                },
                &[],
                "distributor",
                None,
            )
            .unwrap();

        let funds = self.funds;
        for (addr, coin) in funds {
            app.init_bank_balance(&addr, coin).unwrap();
        }

        Suite {
            app,
            owner,
            contract: distributor_contract,
        }
    }
}

/// Test suite
pub struct Suite {
    /// The multitest app
    app: App,
    owner: Addr,
    /// Address of Market contract
    contract: Addr,
}

impl Suite {
    pub fn app(&mut self) -> &mut App {
        &mut self.app
    }

    pub fn owner(&mut self) -> Addr {
        self.owner.clone()
    }

    pub fn query_config(&self) -> AnyResult<Config> {
        let contract = self.contract.clone();
        let response: Config = self
            .app
            .wrap()
            .query_wasm_smart(contract, &QueryMsg::Config {})?;
        Ok(response)
    }
}
