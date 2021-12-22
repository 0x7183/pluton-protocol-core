use cosmwasm_std::{Addr, Api, CanonicalAddr, Deps, Order, StdResult};
use cw20::{AllAccountsResponse};

use crate::state::{ BALANCES};
use cw_storage_plus::Bound;

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn calc_range_start_human(
    api: &dyn Api,
    start_after: Option<Addr>,
) -> StdResult<Option<Vec<u8>>> {
    match start_after {
        Some(human) => {
            let mut v: Vec<u8> = api.addr_canonicalize(human.as_ref())?.0.into();
            v.push(0);
            Ok(Some(v))
        }
        None => Ok(None),
    }
}

pub fn query_all_accounts(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AllAccountsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start =
        calc_range_start_human(deps.api, start_after.map(Addr::unchecked))?.map(Bound::exclusive);

    let accounts: Result<Vec<_>, _> = BALANCES
        .keys(deps.storage, start, None, Order::Ascending)
        .map(|k| {
            deps.api
                .addr_humanize(&CanonicalAddr::from(k))
                .map(|v| v.to_string())
        })
        .take(limit)
        .collect();

    Ok(AllAccountsResponse {
        accounts: accounts?,
    })
}
