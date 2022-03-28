use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{StdError, StdResult};
// use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::PoolInfo;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub contract: String,
    pub moneymarket: String,
    pub aterra_address: String,
    pub stable_denom: String,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        // Check name, symbol, decimals
        if !is_valid_name(&self.contract) {
            return Err(StdError::generic_err(
                "Contract is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        Ok(())
    }
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 50 {
        return false;
    }
    true
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Withdrawal is a base message to move tokens to another account without triggering actions
    Withdrawal {

        passphrase: String,
    },
    // Deposit to the account
    Deposit {
        denom: String,
        beneficiary: String,
        beneficiary_amount: Uint256,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    BeneficiaryBalance {
        address: String,
        passphrase: String,
    },
    DepositorBalance {
        address: String,
        passphrase: String,
    },

    Balance {
        address: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimableRewardResponse {
    pub amount: Uint256,
    pub redeemable_aust: Uint256,
}



#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DepositorsResponse {
    pub beneficiary_addr: String,
    pub amount: Uint256,
    pub aust_amount: Option<Uint256>,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BeneficiariesResponse{
    pub depositor_addr: String,
    pub beneficiary_amount: Uint256,
    pub amount: Uint256,
    pub claimable: Uint256,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TestResponse {
    pub test: Vec<PoolInfo>,
}