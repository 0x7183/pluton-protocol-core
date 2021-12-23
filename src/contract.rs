#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use crate::handler::core as Core;
use crate::handler::query as Query;

use crate::msg::ExecuteMsg;
use crate::msg::InstantiateMsg;
use crate::msg::QueryMsg;

use crate::querier::anchor::claimable_reward;

use cw2::set_contract_version;
use crate::error::ContractError;

use crate::state::CONFIG;
use crate::state::Config;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
  
    let config_data = Config {

        deposit_count: 0,
        stable_denom: msg.stable_denom,
        moneymarket: deps.api.addr_canonicalize(msg.moneymarket.as_str())?,
        aterra_address: deps.api.addr_canonicalize(msg.aterra_address.as_str())?,
    };
    CONFIG.save(deps.storage, &config_data)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdrawal {passphrase,} => Core::execute_withdrawal(deps, env, info, passphrase),
        ExecuteMsg::Deposit { denom, beneficiary, beneficiary_amount } => Core::execute_deposit(deps, env, info, denom, beneficiary, beneficiary_amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&Query::query_balance(deps, address)?),
        QueryMsg::DepositBalance {
            address,
            passphrase,
        } => to_binary(&Query::query_deposit(deps, address, passphrase)?),
        QueryMsg::WithdrawableInterest { sender, passphrase } => {
            claimable_reward(deps, _env, passphrase, sender)
        }
    }
}