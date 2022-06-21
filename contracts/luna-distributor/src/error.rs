use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Zero balance")]
    NoBalance {},

    #[error("Trying to deposit more then one denom at once")]
    DepositMoreThenOne {},

    #[error("Trying to deposit unsupported denom {0}")]
    UnsupportedDenom(String),

    #[error("Provided protocol {0} is not on whitelist")]
    DistributionNoSuchProtocol(String),
}
