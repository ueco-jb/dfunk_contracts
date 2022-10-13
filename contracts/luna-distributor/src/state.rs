use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: String,
    pub burn_address: Addr,
    pub developer_address: Addr,
    pub whitelist: Vec<Whitelist>,
    pub weight_per_protocol: Vec<WeightPerProtocol>,
    pub percent_to_burn: Decimal,
    pub percent_to_developer: Decimal,
    pub percent_to_distribute: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Whitelist {
    pub address: Addr,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WeightPerProtocol {
    pub protocol: String,
    pub weight: Decimal,
}

pub const CONFIG: Item<Config> = Item::new("config");
