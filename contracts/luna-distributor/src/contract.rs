#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, BankMsg, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, WeightPerProtocol, Whitelist};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:luna-distributor";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut whitelist: Vec<crate::state::Whitelist> = Vec::new();
    for entry in msg.whitelist {
        let address = deps.api.addr_validate(&entry.address)?;
        whitelist.push(crate::state::Whitelist {
            address,
            protocol: entry.protocol.clone(),
        });
    }
    let mut weight_per_protocol: Vec<crate::state::WeightPerProtocol> = Vec::new();
    for entry in msg.weight_per_protocol {
        weight_per_protocol.push(crate::state::WeightPerProtocol {
            protocol: entry.protocol.clone(),
            weight: entry.weight,
        });
    }

    let burn_address = deps.api.addr_validate(&msg.burn_address)?;
    let config = Config {
        admin: msg.admin,
        burn_address,
        whitelist,
        weight_per_protocol,
        percent_to_burn: Decimal::from_ratio(7778u128, 10000u128), // 77.78%
        percent_to_distribute: Decimal::from_ratio(2222u128, 10000u128), // 22.22%
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Distribute { denom } => execute::distribute(deps, env, denom),
        ExecuteMsg::UpdateConfig {
            admin,
            burn_address,
            whitelist,
            weight_per_protocol,
        } => execute::update_config(
            deps,
            info,
            admin,
            burn_address,
            whitelist,
            weight_per_protocol,
        ),
    }
}

mod execute {
    use super::*;

    use cosmwasm_std::{BalanceResponse, BankQuery, QueryRequest};

    pub fn distribute(deps: DepsMut, env: Env, denom: String) -> Result<Response, ContractError> {
        let contract_address = env.contract.address;
        let balance: BalanceResponse =
            deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
                address: contract_address.to_string(),
                denom: denom.clone(),
            }))?;
        let balance = balance.amount.amount;

        if balance == Uint128::zero() {
            return Err(ContractError::NoBalance {});
        }

        let config = CONFIG.load(deps.storage)?;
        let amount_to_distribute = balance * config.percent_to_distribute;
        let amount_to_burn = balance * config.percent_to_burn;

        let mut response = Response::new().add_submessage(SubMsg::new(BankMsg::Send {
            to_address: config.burn_address.to_string(),
            amount: vec![coin(amount_to_burn.u128(), denom.clone())],
        }));

        // Iter through whitelist
        for wl_item in config.whitelist.iter() {
            let weight_per_protocol = config
                .weight_per_protocol
                .iter()
                // find appropriate protocol
                .find(|wpp| wpp.protocol == wl_item.protocol);
            // if contract has been found, add extra bank message
            if let Some(wpp) = weight_per_protocol {
                let amount = amount_to_distribute * wpp.weight;
                let msg = SubMsg::new(BankMsg::Send {
                    to_address: wl_item.address.to_string(),
                    amount: vec![coin(amount.u128(), denom.clone())],
                });
                response = response.add_submessage(msg);
            } else {
                return Err(ContractError::MissingProtocol(wl_item.protocol.clone()));
            }
        }

        Ok(response)
    }

    pub fn update_config(
        deps: DepsMut,
        info: MessageInfo,
        admin: Option<String>,
        burn_address: Option<String>,
        whitelist: Option<Vec<Whitelist>>,
        weight_per_protocol: Option<Vec<WeightPerProtocol>>,
    ) -> Result<Response, ContractError> {
        let mut config = CONFIG.load(deps.storage)?;
        if config.admin.is_empty() {
            return Err(ContractError::ConfigNotUpdatable {});
        }
        let cfg_admin = deps.api.addr_validate(&config.admin)?;
        if cfg_admin != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        if let Some(admin) = admin {
            config.admin = admin
        }

        if let Some(burn_address) = burn_address {
            config.burn_address = deps.api.addr_validate(&burn_address)?;
        }

        if let Some(whitelist) = whitelist {
            config.whitelist.clear();
            for entry in whitelist {
                let address = deps.api.addr_validate(&entry.address)?;
                config.whitelist.push(crate::state::Whitelist {
                    address,
                    protocol: entry.protocol.clone(),
                });
            }
        }

        if let Some(weight_per_protocol) = weight_per_protocol {
            config.weight_per_protocol.clear();
            for entry in weight_per_protocol {
                config
                    .weight_per_protocol
                    .push(crate::state::WeightPerProtocol {
                        protocol: entry.protocol.clone(),
                        weight: entry.weight,
                    });
            }
        }

        CONFIG.save(deps.storage, &config)?;

        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
    }
}

mod query {
    use super::*;

    pub fn config(deps: Deps) -> StdResult<Config> {
        CONFIG.load(deps.storage)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
