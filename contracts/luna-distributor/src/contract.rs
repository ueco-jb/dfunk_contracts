#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;

use std::collections::HashMap;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, DEPOSITS};

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
    let mut whitelist: HashMap<Addr, String> = HashMap::new();
    for entry in msg.whitelist {
        let address = deps.api.addr_validate(&entry.address)?;
        whitelist.insert(address, entry.protocol.clone());
    }
    let mut weight_per_protocol: HashMap<String, Decimal> = HashMap::new();
    for entry in msg.weight_per_protocol {
        weight_per_protocol.insert(entry.protocol.clone(), entry.weight);
    }
    let config = Config {
        burn_address: deps.api.addr_validate(&msg.burn_address)?,
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => execute::deposit(deps, info),
        ExecuteMsg::Withdraw { amount, denom } => execute::withdraw(deps, info, amount, denom),
        ExecuteMsg::Distribute { denom } => execute::distribute(deps, info, denom),
    }
}

mod execute {
    use super::*;

    pub fn deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let deposit_addr = info.sender;

        let funds = if info.funds.len() != 1 {
            return Err(ContractError::DepositMoreThenOne {});
        } else {
            info.funds[0].clone()
        };

        let tokens = if funds.denom == "uusd" || funds.denom == "uluna" {
            (info.funds[0].amount, info.funds[0].denom.clone())
        } else {
            return Err(ContractError::UnsupportedDenom(funds.denom));
        };

        DEPOSITS.update(
            deps.storage,
            (&deposit_addr, &tokens.1),
            |deposit: Option<Uint128>| -> StdResult<_> {
                Ok(deposit.unwrap_or_default() + tokens.0)
            },
        )?;

        Ok(Response::new().add_attribute("method", "deposit"))
    }
    pub fn withdraw(
        deps: DepsMut,
        info: MessageInfo,
        amount: Option<Uint128>,
        denom: String,
    ) -> Result<Response, ContractError> {
        let deposited = DEPOSITS.load(deps.storage, (&info.sender, &denom))?;
        if deposited == Uint128::zero() {
            return Err(ContractError::NoBalance {});
        }

        let amount = amount.unwrap_or(deposited);

        DEPOSITS.update(
            deps.storage,
            (&info.sender, &denom),
            |deposited: Option<Uint128>| -> StdResult<_> {
                Ok(deposited.unwrap_or_default() - amount)
            },
        )?;

        Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin(u128::from(amount), denom)],
        })))
    }

    pub fn distribute(
        deps: DepsMut,
        info: MessageInfo,
        denom: String,
    ) -> Result<Response, ContractError> {
        let deposited = DEPOSITS.load(deps.storage, (&info.sender, &denom))?;
        if deposited == Uint128::zero() {
            return Err(ContractError::NoBalance {});
        }

        let config = CONFIG.load(deps.storage)?;
        let amount_to_distribute = deposited * config.percent_to_distribute;
        let amount_to_burn = deposited * config.percent_to_burn;

        let mut response = Response::new().add_submessage(SubMsg::new(BankMsg::Send {
            to_address: config.burn_address.to_string(),
            amount: vec![coin(amount_to_burn.u128(), denom.clone())],
        }));

        for (address, protocol) in config.whitelist.iter() {
            if let Some(protocol_weight) = config.weight_per_protocol.get(protocol) {
                let amount = amount_to_distribute * *protocol_weight;
                let msg = SubMsg::new(BankMsg::Send {
                    to_address: address.to_string(),
                    amount: vec![coin(amount.u128(), denom.clone())],
                });
                response = response.add_submessage(msg);
            }
        }

        Ok(response)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetDeposits { address: _ } => to_binary(&""),
    }
}

// pub fn query_deposits(
//     deps: Deps,
//     address: Addr,
// ) -> Result<std::vec::Vec<(std::vec::Vec<u8>, Deposit)>, cosmwasm_std::StdError> {
//     let all: Vec<_> = DEPOSITS
//         .prefix(&address)
//         .range(deps.storage, None, None, Order::Ascending)
//         .collect::<StdResult<_>>()?;
//
//     Ok(all)
// }
