
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult,Uint128, from_binary};

use crate::querier::anchor::claimable_reward;

use crate::error::ContractError;
use crate::msg::ClaimableRewardResponse;
use crate::state::{PoolInfo, CONFIG, DEPOSITORS, BENEFICIARIES};

use cosmwasm_bignumber::{Uint256};
use std::ops::Sub;

use cosmwasm_std::*;
use std::ops::{Div, Mul};
use std::str::FromStr;

use crate::querier::anchor::deduct_tax;
use crate::querier::anchor::{self, epoch_state};

use std::ops::Add;

// Execute deposit
pub fn execute_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    stable_denom: String,
    beneficiary: String,
    beneficiary_amount: Uint256,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let coin_amount: Uint128 = info
        .funds
        .iter()
        .find(|c| c.denom == stable_denom)
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);

    if coin_amount.is_zero() {
        return Err(ContractError::NotAllowZeroAmount {});
    }
    if info.funds.len() > 1 {
        return Err(ContractError::NotAllowOtherDenoms {
            denom: config.stable_denom,});
    }
    if stable_denom != "uusd".to_owned() {
      return Err(ContractError::NotAllowOtherDenoms {
            denom: config.stable_denom,});
    }

    let deposit_addr = info.sender.to_string();
    let new_deposit_count = config.deposit_count + 1;

    CONFIG.update(
        deps.storage,
        |x| -> StdResult<_> {
            let mut config = x;
            config.deposit_count = new_deposit_count;
            Ok(config)
        },
    )?;

    let pool = PoolInfo {
        id: new_deposit_count.to_string(),
        amount: Uint256::from(coin_amount),
        denom: stable_denom,
        aust_amount: None,
        exchange_rate_prev: None,
        depositor_addr: deposit_addr.clone(),
        beneficiary_addr: beneficiary.clone(),
        beneficiary_amount: beneficiary_amount,
    };

    DEPOSITORS.save(
        deps.storage,
        (&deposit_addr, &new_deposit_count.to_string()),
        &pool.clone()
    )?;
    
    BENEFICIARIES.save(
        deps.storage,
        (&beneficiary, &new_deposit_count.to_string()),
        &pool.clone()
    )?;

    deposit(deps, _env, info, coin_amount, &deposit_addr, &beneficiary)
}

// Execute withdrawal, only depositor can execute it 
pub fn execute_withdrawal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let pool = DEPOSITORS.load(deps.storage, (&info.sender.to_string(), &id))?;
    let aust_amount: Uint256 = pool.aust_amount.unwrap();
    let amount = pool.amount;
    if amount == Uint256::zero() {
        return Err(ContractError::NoBalance {});
    }

    let redeem_info: ClaimableRewardResponse = from_binary(
        &claimable_reward(
            deps.as_ref(),
            _env.clone(),
            id.clone(),
            info.sender.clone().to_string(),
        )
        .unwrap(),
    )
    .unwrap();

    let bnf_addr = deps.api.addr_canonicalize(&pool.beneficiary_addr)?;
    let bnf_aust_amount = redeem_info.redeemable_aust;

    let rcpt_aust_amount = aust_amount.sub(bnf_aust_amount.into());
    let rcpt_addr = deps.api.addr_canonicalize(&pool.depositor_addr)?;

    if bnf_aust_amount < pool.beneficiary_amount{

        return Err(ContractError::NoBalance {});
    } 

    DEPOSITORS.remove(deps.storage, (&info.sender.to_string(), &id));
    BENEFICIARIES.remove(deps.storage, (&pool.beneficiary_addr, &id));

    redeem(
        deps.as_ref(),
        _env, info,
        rcpt_aust_amount, 
        rcpt_addr, 
        bnf_aust_amount.into(), 
        bnf_addr,
    )
}
// Execute withdraw interest, only beneficiary can execute it
pub fn execute_withdrawinterest(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let pool = BENEFICIARIES.load(deps.storage, (&info.sender.to_string(), &id))?;
    let amount = pool.amount;
    if amount == Uint256::zero() {
        return Err(ContractError::NoBalance {});
    }

    let redeem_info: ClaimableRewardResponse = from_binary(
        &claimable_reward(
            deps.as_ref(),
            _env.clone(),
            id.clone(),
            pool.depositor_addr.clone().to_string(),
        )
        .unwrap(),
    )
    .unwrap();

    let bnf_addr = deps.api.addr_canonicalize(&pool.beneficiary_addr)?;
    let bnf_aust_amount = redeem_info.redeemable_aust;

    if bnf_aust_amount < pool.beneficiary_amount{

        return Err(ContractError::NoBalance {});
    } 


    redeem_interest(
        deps.as_ref(),
        _env, info,
        bnf_aust_amount.into(), 
        bnf_addr,
    )
}

// Handler for execute_deposit
pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    received: Uint128,
    deposit_addr: &str,
    beneficiary_addr : &str,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage).unwrap();

    let epoch_state = epoch_state(deps.as_ref(), _env.clone(), &config.moneymarket)?;

    let ust_amount = deduct_tax(
        deps.as_ref(),
        Coin {
            denom: config.stable_denom.clone(),
            amount: received,
        },
    )?
    .amount;

    let aust_amount = Uint256::from(ust_amount).div(epoch_state.exchange_rate);
    
    DEPOSITORS.update(
        deps.storage,
        (&deposit_addr, &config.deposit_count.to_string()),
        |x| -> StdResult<_> {
            let mut info = x.unwrap();
            info.aust_amount = Some(Uint256::from_str(&aust_amount.to_string()).unwrap());
            info.exchange_rate_prev = Some(epoch_state.exchange_rate.to_string());
            Ok(info)
        },
    )?;

    BENEFICIARIES.update(
        deps.storage,
        (&beneficiary_addr, &config.deposit_count.to_string()),
        |x| -> StdResult<_> {
            let mut info = x.unwrap();
            info.aust_amount = Some(Uint256::from_str(&aust_amount.to_string()).unwrap());
            info.exchange_rate_prev = Some(epoch_state.exchange_rate.to_string());
            Ok(info)
        },
    )?;

    Ok(Response::new()
        .add_messages(anchor::deposit_stable_msg(
            deps.as_ref(),
            &config.moneymarket,
            &config.stable_denom,
            received.into(),
        )?)
        .add_attribute("action", "deposit")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("amount", aust_amount.clone().to_string())
        .add_attribute("id", &config.deposit_count.to_string())
        .add_attribute("exchange_rate_prev", epoch_state.exchange_rate.to_string()))

}
/// Handler for  execute_withdrawall

pub fn redeem(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    rcpt_aust_amount: Uint256,
    rcpt_address: CanonicalAddr,
    bnf_aust_amount: Uint256,
    bnf_address: CanonicalAddr,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage).unwrap();

    let epoch_state = anchor::epoch_state(deps, _env.clone(), &config.moneymarket)?;
    let rcpt_ust_amount = rcpt_aust_amount.mul(epoch_state.exchange_rate);

    let aust_amount = rcpt_aust_amount.add(bnf_aust_amount);
    let rcpt_redeem_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: deduct_tax(
                deps,
                Coin {
                    denom: config.stable_denom.clone(),
                    amount: rcpt_ust_amount.into(),
                },
            )
            .unwrap()
            .amount,
        },
    )
    .unwrap();

    let bnf_ust_amount = bnf_aust_amount.mul(epoch_state.exchange_rate);
    let bnf_redeem_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: deduct_tax(
                deps,
                Coin {
                    denom: config.stable_denom.clone(),
                    amount: bnf_ust_amount.into(),
                },
            )
            .unwrap()
            .amount,
        },
    )
    .unwrap();

    Ok(Response::new()
        .add_messages(anchor::redeem_stable_msg(
            deps,
            &config.moneymarket,
            &config.aterra_address,
            aust_amount.into(),
        )?)
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: deps.api.addr_humanize(&rcpt_address).unwrap().into_string(),
            amount: vec![coin(
                u128::from(rcpt_redeem_amount.amount),
                rcpt_redeem_amount.denom.clone(),
            )],
        }))
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: deps.api.addr_humanize(&bnf_address).unwrap().into_string(),
            amount: vec![coin(
                u128::from(bnf_redeem_amount.amount),
                bnf_redeem_amount.denom.clone(),
            )],
        }))
        .add_attribute("action", "redeem")
        .add_attribute("sender", _env.contract.address)
        .add_attribute("rcpt_amount", rcpt_redeem_amount.to_string())
        .add_attribute("recipient", rcpt_address.clone().to_string())
        .add_attribute("bnf_amount", bnf_redeem_amount.to_string())
        .add_attribute("beneficiary", bnf_address.clone().to_string()))
}

// Handler for execute_withdrawinterest
pub fn redeem_interest(
    deps: Deps,
    _env: Env,
    _info: MessageInfo,
    bnf_aust_amount: Uint256,
    bnf_address: CanonicalAddr,
) -> Result<Response, ContractError> {

    let config = CONFIG.load(deps.storage).unwrap();

    let epoch_state = anchor::epoch_state(deps, _env.clone(), &config.moneymarket)?;
    let bnf_ust_amount = bnf_aust_amount.mul(epoch_state.exchange_rate);

    let bnf_redeem_amount = deduct_tax(
        deps,
        Coin {
            denom: config.stable_denom.clone(),
            amount: deduct_tax(
                deps,
                Coin {
                    denom: config.stable_denom.clone(),
                    amount: bnf_ust_amount.into(),
                },
            )
            .unwrap()
            .amount,
        },
    )
    .unwrap();

    Ok(Response::new()
        .add_messages(anchor::redeem_stable_msg(
            deps,
            &config.moneymarket,
            &config.aterra_address,
            bnf_aust_amount.into(),
        )?)
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: deps.api.addr_humanize(&bnf_address).unwrap().into_string(),
            amount: vec![coin(
                u128::from(bnf_redeem_amount.amount),
                bnf_redeem_amount.denom.clone(),
            )],
        }))
        .add_attribute("action", "redeem")
        .add_attribute("sender", _env.contract.address)
        .add_attribute("bnf_amount", bnf_redeem_amount.to_string())
        .add_attribute("beneficiary", bnf_address.clone().to_string()))
}