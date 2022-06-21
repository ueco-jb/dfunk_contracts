use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub burn_address: Addr,
    pub whitelist: HashMap<Addr, String>,
    pub weight_per_protocol: HashMap<String, Decimal>,
    pub percent_to_burn: Decimal,
    pub percent_to_distribute: Decimal,
}

pub const DEPOSITS: Map<(&Addr, &str), Uint128> = Map::new("deposit");
pub const CONFIG: Item<Config> = Item::new("config");
