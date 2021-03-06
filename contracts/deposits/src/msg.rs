use cosmwasm_bignumber::Uint256;
use cosmwasm_std::{StdError, StdResult};
// use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    /// Withdrawal is a base message to withdraw deposit and interest, only depositor can execute it
    Withdrawal {

        id: String,
    },
    // Deposit is a base message to deposit into the smart contract
    Deposit {
        denom: String,
        beneficiary: String,
        beneficiary_amount: Uint256,
    },

    // Withdraw is a base message to withdraw interest, only beneficiary can execute it
    WithdrawInterest {
        id: String,
    },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // return all the incoming donations/payments with id
    BeneficiaryBalance {
        address: String, // beneficiary address
    },

    // return all the outgoing donations/payments with id
    DepositorBalance {
        address: String, // depositor address
    },

    Outgoing {
        address: String, 
        id: String,
    },

    Incoming {
        address: String, 
        id: String,
    },

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
    pub aust_amount: Uint256,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BeneficiariesResponse{
    pub depositor_addr: String,
    pub beneficiary_amount: Uint256,
    pub amount: Uint256,
    pub claimable: Uint256,
}


