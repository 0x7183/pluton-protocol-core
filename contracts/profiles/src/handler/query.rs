use cosmwasm_std::{Addr, StdResult, Deps};

use crate::{state::PROFILES, msg::ProfileResponse};


pub fn get_profile(deps: Deps, address: String) -> StdResult<ProfileResponse> {

    let checked: Addr = deps.api.addr_validate(&address)?;
    let profile = PROFILES.may_load(deps.storage, &checked)?.unwrap();

    Ok(ProfileResponse {    
        img_url: profile.img,
        name: profile.name,
        description: profile.description,
        github: profile.github,
        linkedin: profile.linkedin,
        twitter: profile.twitter,
    })
}