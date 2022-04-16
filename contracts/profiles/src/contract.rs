#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::handler::core as Core;
use crate::handler::query as Query;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:profiles";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Execute msg for profile handler
        ExecuteMsg::Register {img, name, description, github, linkedin, twitter} => Core::register(deps, info, img, name, description, github, linkedin, twitter),
        ExecuteMsg::Modify {img, name, description, github, linkedin, twitter} => Core::modify(deps, info, img, name, description, github, linkedin, twitter),
        ExecuteMsg::Delete {} => Core::delete(deps, info),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
         // query for profile handler
         QueryMsg::GetProfile {address} => to_binary(&Query::get_profile(deps, address)?),
    }
}

