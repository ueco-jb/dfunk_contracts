use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Decimal;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub burn_address: String,
    pub developer_address: String,
    pub whitelist: Vec<Whitelist>,
    pub weight_per_protocol: Vec<WeightPerProtocol>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Distribute {
        denom: String,
    },
    UpdateConfig {
        admin: Option<String>,
        burn_address: Option<String>,
        developer_address: Option<String>,
        whitelist: Option<Vec<Whitelist>>,
        weight_per_protocol: Option<Vec<WeightPerProtocol>>,
    },
    /// Burn any leftover tokens that stays on the contract
    BurnTheBottom {
        /// Denom of leftover tokens to burn
        denom: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Whitelist {
    pub address: String,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WeightPerProtocol {
    pub protocol: String,
    pub weight: Decimal,
}
