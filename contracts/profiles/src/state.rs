use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Map};

// Struct for Profiles
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProfileInfo {
    pub img: String,
    pub name: String,
    pub description: String,
    pub github:  String,
    pub linkedin: String,
    pub twitter: String,
}

pub const PROFILES: Map<&Addr, ProfileInfo> = Map::new("profiles");
