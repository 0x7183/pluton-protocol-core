use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Register is a base message to register a new profile, only one profile per address
    Register {
        img: String,
        name: String,
        description: String,
        github:  String,
        linkedin: String,
        twitter: String
    },

    // Modify is a base message to modify an existing profile, only the owner can execute it
    Modify {
        img: String,
        name: String,
        description: String,
        github:  String,
        linkedin: String,
        twitter: String
    },

    // Delete is a base message to delete an existing profile, only the owner can execute it
    Delete {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // return profile info for address
    GetProfile {address: String},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProfileResponse {
    pub img_url: String,
    pub name: String,
    pub description: String,
    pub github:  String,
    pub linkedin: String,
    pub twitter: String
}
