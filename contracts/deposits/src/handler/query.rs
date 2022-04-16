use crate::msg::{DepositorsResponse, BeneficiariesResponse, ClaimableRewardResponse};

use crate::querier::anchor::claimable_reward;
use crate::state::{DEPOSITORS, BENEFICIARIES, PoolInfo};


use cosmwasm_std::{StdResult, Env, from_binary, Order};
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


pub fn query_interest(deps: Deps, env: Env, address: String, id: String) -> StdResult<BeneficiariesResponse> {
    let info = BENEFICIARIES
        .may_load(deps.storage, (address.to_string().as_str(), &id))?
        .unwrap();
    let rewards: ClaimableRewardResponse = from_binary(&claimable_reward(
                deps,
                env.clone(),
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





