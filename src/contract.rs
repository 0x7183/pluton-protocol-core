#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,};
use crate::handler::core as Core;
use crate::handler::core::execute_withdrawinterest;
use crate::handler::query as Query;
use crate::handler::profile as Profile;

use crate::msg::ExecuteMsg;
use crate::msg::InstantiateMsg;
use crate::msg::QueryMsg;


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
        
        // Execute msg for deposit handler
        ExecuteMsg::Withdrawal{id,}=>Core::execute_withdrawal(deps, env, info, id),
        ExecuteMsg::Deposit{denom,beneficiary,beneficiary_amount}=>Core::execute_deposit(deps, env, info,denom, beneficiary, beneficiary_amount),
        ExecuteMsg::WithdrawInterest {id} => execute_withdrawinterest(deps, env, info, id),
        
        // Execute msg for profile handler
        ExecuteMsg::Register {img_url, name, description, github, linkedin, twitter} => Profile::register(deps, info, img_url, name, description, github, linkedin, twitter),
        ExecuteMsg::Modify {img_url, name, description, github, linkedin, twitter} => Profile::modify(deps, info, img_url, name, description, github, linkedin, twitter),
        ExecuteMsg::Delete {} => Profile::delete(deps, info),
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        
        // Query for deposit handler
        QueryMsg::BeneficiaryBalance {address} => to_binary(&Query::query_beneficiary(deps, address)?),
        QueryMsg::DepositorBalance {address} => to_binary(&Query::query_depositor(deps, address)?),
        QueryMsg::Incoming {address, id} => to_binary(&Query::query_interest(deps, _env, address, id)?),
        QueryMsg::Outgoing {address, id} => to_binary(&Query::query_deposit(deps, address, id)?),

        // query for profile handler
        QueryMsg::GetProfile {address} => to_binary(&Query::get_profile(deps, address)?),
    }
}