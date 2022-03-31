
use crate::msg::{DepositorsResponse, BeneficiariesResponse, ClaimableRewardResponse, ProfileResponse};

use crate::querier::anchor::claimable_reward;
use crate::state::{DEPOSITORS, BENEFICIARIES, PoolInfo, PROFILES};


use cosmwasm_std::{StdResult, Env, from_binary, Order, Addr};
use cosmwasm_std::Deps;


pub fn query_beneficiary(deps: Deps, address: String) -> Result<std::vec::Vec<(std::vec::Vec<u8>, PoolInfo)>, cosmwasm_std::StdError>{

    let all: Vec<_> = BENEFICIARIES
            .prefix(&address)
            .range(deps.storage, None, None, Order::Ascending)
            .collect::<StdResult<_>>()?;

    Ok(all)

}

pub fn query_depositor(deps: Deps, address: String) -> Result<std::vec::Vec<(std::vec::Vec<u8>, PoolInfo)>, cosmwasm_std::StdError>{

    let all: Vec<_> = DEPOSITORS
    .prefix(&address)
    .range(deps.storage, None, None, Order::Ascending)
    .collect::<StdResult<_>>()?;
 
    Ok(all)

}


pub fn query_interest(deps: Deps, _env: Env, address: String, id: String) -> StdResult<BeneficiariesResponse> {
    let info = BENEFICIARIES
        .may_load(deps.storage, (address.to_string().as_str(), &id))?
        .unwrap();
    let rewards: ClaimableRewardResponse = from_binary(&claimable_reward(
                deps,
                _env.clone(),
                id.clone(),
                info.depositor_addr.clone().to_string(),
                )
                .unwrap(),
            )
            .unwrap();

    Ok(BeneficiariesResponse {
        depositor_addr: info.depositor_addr,
        beneficiary_amount: info.beneficiary_amount,
        amount: info.amount,
        claimable: rewards.amount,
       
    })
}

pub fn query_deposit(
    deps: Deps,
    address: String,
    id: String,
) -> StdResult<DepositorsResponse> {
    let info = DEPOSITORS
        .may_load(deps.storage, (address.to_string().as_str(), &id))?
        .unwrap();

    Ok(DepositorsResponse {
        beneficiary_addr: info.beneficiary_addr,
        amount: info.amount,
        aust_amount: info.aust_amount,
        denom: info.denom,
    })

}

pub fn get_profile(deps: Deps, address: String) -> StdResult<ProfileResponse> {

    let checked: Addr = deps.api.addr_validate(&address)?;
    let profile = PROFILES.may_load(deps.storage, &checked)?.unwrap();

    Ok(ProfileResponse {    
        img_url: profile.img_url,
        name: profile.name,
        description: profile.description,
        github: profile.github,
        linkedin: profile.linkedin,
        twitter: profile.twitter,
    })
}



