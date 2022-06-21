use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub burn_address: Addr,
    pub whitelist: Vec<Whitelist>,
    pub weight_per_protocol: Vec<WeightPerProtocol>,
    pub percent_to_burn: Decimal,
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

pub const DEPOSITS: Map<(&Addr, String), Uint128> = Map::new("deposit");
pub const CONFIG: Item<Config> = Item::new("config");
