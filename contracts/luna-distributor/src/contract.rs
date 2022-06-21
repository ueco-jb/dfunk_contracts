#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    Order, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Whitelist, CONFIG, DEPOSITS};

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
    let whitelist = msg
        .whitelist
        .iter()
        .map(|wl| {
            Ok(Whitelist {
                address: deps.api.addr_validate(&wl.address)?,
                protocol: wl.protocol.clone(),
            })
        })
        .collect::<Result<Vec<Whitelist>, ContractError>>()?;
    let config = Config {
        burn_address: deps.api.addr_validate(&msg.burn_address)?,
        whitelist,
        weight_per_protocol: msg.weight_per_protocol,
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
        ExecuteMsg::Deposit {} => execute_deposit(deps, info),
        ExecuteMsg::Withdrawal { amount, denom } => execute_withdrawal(deps, info, amount, denom),
    }
}

pub fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let deposit_addr = info.sender;

    assert_eq!(info.funds.len(), 1);
    let tokens = if info.funds[0].denom == "uusd" || info.funds[0].denom == "uluna" {
        (info.funds[0].amount, info.funds[0].denom)
    } else {
        return Err(ContractError::Unauthorized {});
    };

    DEPOSITS.update(
        deps.storage,
        (&deposit_addr, tokens.1),
        |deposit: Option<Uint128>| -> StdResult<_> { Ok(deposit.unwrap_or_default() + tokens.0) },
    )?;

    Ok(Response::new().add_attribute("method", "deposit"))
}
pub fn execute_withdrawal(
    deps: DepsMut,
    info: MessageInfo,
    amount: Option<Uint128>,
    denom: String,
) -> Result<Response, ContractError> {
    let deposited = DEPOSITS.load(deps.storage, (&info.sender, denom))?;
    if deposited == Uint128::zero() {
        return Err(ContractError::NoBalance {});
    }

    let amount = amount.unwrap_or(deposited);

    DEPOSITS.update(
        deps.storage,
        (&info.sender, denom),
        |deposited: Option<Uint128>| -> StdResult<_> { Ok(deposited.unwrap_or_default() - amount) },
    )?;

    Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![coin(u128::from(amount), denom)],
    })))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // match msg {
    //     QueryMsg::GetDeposits { address } => to_binary(&query_deposits(deps, address)?),
    // }
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
