use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Pool: Zero amount not allowed")]
    NotAllowZeroAmount {},

    #[error("Pool: other denom except {denom:?} is not allowed")]
    NotAllowOtherDenoms { denom: String },

    #[error("Zero balance")]
    NoBalance {},

    #[error("Can not deposit")]
    InvalidDeposit {},

}
