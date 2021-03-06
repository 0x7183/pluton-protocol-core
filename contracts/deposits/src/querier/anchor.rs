use std::{
    ops::{Div, Mul},
    str::FromStr,
};

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::*;
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terra_cosmwasm::TerraQuerier;

use crate::{
    msg,
    state::{CONFIG, DEPOSITORS},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    EpochState {
        block_height: Option<u64>,
        distributed_interest: Option<Uint256>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EpochStateResponse {
    pub exchange_rate: Decimal256,
    pub aterra_supply: Uint256,
}

pub fn epoch_state(deps: Deps, env: Env, market: &CanonicalAddr) -> StdResult<EpochStateResponse> {
    let epoch_state: EpochStateResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.addr_humanize(market).unwrap().to_string(),
            msg: to_binary(&QueryMsg::EpochState {
                block_height: Some(env.block.height),
                distributed_interest: None,
            })?,
        }))?;

    Ok(epoch_state)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    DepositStable {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Return stable coins to a user
    /// according to exchange rate
    RedeemStable {},
}

pub fn deposit_stable_msg(
    deps: Deps,
    market: &CanonicalAddr,
    denom: &str,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(market).unwrap().to_string(),
        msg: to_binary(&HandleMsg::DepositStable {})?,
        funds: vec![deduct_tax(
            deps,
            Coin {
                denom: denom.to_string(),
                amount,
            },
        )?],
    })])
}

pub fn redeem_stable_msg(
    deps: Deps,
    market: &CanonicalAddr,
    token: &CanonicalAddr,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(token).unwrap().to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(market).unwrap().to_string(),
            amount,
            msg: to_binary(&Cw20HookMsg::RedeemStable {}).unwrap(),
        })?,
        funds: vec![],
    })])
}

pub fn deduct_tax(deps: Deps, coin: Coin) -> StdResult<Coin> {
    let tax_amount = compute_tax(deps, &coin)?;
    Ok(Coin {
        denom: coin.denom,
        amount: (Uint256::from(coin.amount) - tax_amount).into(),
    })
}

pub fn compute_tax(deps: Deps, coin: &Coin) -> StdResult<Uint256> {
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate = Decimal256::from((terra_querier.query_tax_rate()?).rate);
    let tax_cap = Uint256::from((terra_querier.query_tax_cap(coin.denom.to_string())?).cap);
    let amount = Uint256::from(coin.amount);
    Ok(std::cmp::min(
        amount * Decimal256::one() - amount / (Decimal256::one() + tax_rate),
        tax_cap,
    ))
}

pub fn balance_of(deps: Deps, token: String, owner: String) -> StdResult<Uint256> {
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: token,
        msg: to_binary(&Cw20QueryMsg::Balance { address: owner })?,
    }))?;

    Ok(Uint256::from(balance.balance))
}


pub fn claimable_reward(
    deps: Deps,
    _env: Env,
    passphrase: String,
    sender: String,
) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let pool = DEPOSITORS.load(deps.storage, (&sender.to_string(), &passphrase))?;

    let epoch_state = epoch_state(deps, _env, &config.moneymarket)?;
    let atoken_balance_user = balance_of(
        deps,
        deps.api
            .addr_humanize(&config.aterra_address)
            .unwrap()
            .to_string(),
        sender.clone(),
    )?;

    let exchange_rate_prev = Decimal256::from_str(&pool.exchange_rate_prev)?;
    let exchange_rate_now = epoch_state.exchange_rate;

    let rewardable_rate = exchange_rate_now - exchange_rate_prev;

    let ust_user_actual = atoken_balance_user.mul(rewardable_rate);

    let send_aust = ust_user_actual.clone().div(exchange_rate_now);

    let ust_user_redeemable = Uint256::from(
        deduct_tax(
            deps,
            Coin {
                denom: config.stable_denom,
                amount: ust_user_actual.into(),
            },
        )?
        .amount,
    );

    to_binary(&msg::ClaimableRewardResponse {
        amount: ust_user_redeemable,
        redeemable_aust: send_aust,
    })
}
