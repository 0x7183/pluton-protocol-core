use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, CanonicalAddr};
use cosmwasm_bignumber::Uint256;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolInfo {
    pub amount: Uint256,
    pub denom: String,
    pub aust_amount: Option<Uint256>,
    pub exchange_rate_prev: Option<String>,
    pub depositor_addr: String,
    pub beneficiary_addr: String,
    pub beneficiary_amount: Uint256,
}

impl PoolInfo {
    pub fn get_amount(&self) {
        self.amount;
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub deposit_count: i32,
    pub stable_denom: String,
    pub aterra_address: CanonicalAddr,
    pub moneymarket: CanonicalAddr
}

pub const POOL_INFO: Item<PoolInfo> = Item::new("pool_info");
pub const DEPOSIT: Map<(&str, &str), PoolInfo> = Map::new("deposit");
pub const CONFIG: Item<Config> = Item::new("config");
pub const BALANCES: Map<&str, Uint128> = Map::new("balance");
