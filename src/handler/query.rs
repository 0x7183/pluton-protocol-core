use cw20::BalanceResponse;
use crate::msg::PoolInfoResponse;

use crate::state::DEPOSIT;
use crate::state::BALANCES;

use cosmwasm_std::StdResult;
use cosmwasm_std::Deps;





pub fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let balance = BALANCES
        .may_load(deps.storage, address.to_string().as_str())?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}

pub fn query_deposit(
    deps: Deps,
    address: String,
    passphrase: String,
) -> StdResult<PoolInfoResponse> {
    let info = DEPOSIT
        .may_load(deps.storage, (address.to_string().as_str(), &passphrase))?
        .unwrap();
    Ok(PoolInfoResponse {
        amount: info.amount,
        denom: info.denom,
        aust_amount: info.aust_amount,
        exchange_rate_prev: info.exchange_rate_prev,
    })
}