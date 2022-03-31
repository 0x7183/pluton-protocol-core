use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Addr};
use cosmwasm_bignumber::Uint256;
use cw_storage_plus::{Item, Map};

// Struct for deposit
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PoolInfo {
    pub id: String,
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
pub struct Config {
    pub deposit_count: i32,
    pub stable_denom: String,
    pub aterra_address: CanonicalAddr,
    pub moneymarket: CanonicalAddr
}

// Struct for Profiles
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProfileInfo {
    pub img_url: String,
    pub name: String,
    pub description: String,
    pub github:  String,
    pub linkedin: String,
    pub twitter: String,
}

pub const PROFILES: Map<&Addr, ProfileInfo> = Map::new("profiles");
pub const POOL_INFO: Item<PoolInfo> = Item::new("pool_info");
pub const DEPOSITORS: Map<(&str, &str), PoolInfo> = Map::new("depositors");
pub const BENEFICIARIES: Map<(&str, &str), PoolInfo> = Map::new("beneficiaries");
pub const CONFIG: Item<Config> = Item::new("config");
