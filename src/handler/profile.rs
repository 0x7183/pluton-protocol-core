use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};


use crate::{state::{PROFILES, ProfileInfo}, ContractError};

pub fn register(deps: DepsMut, info: MessageInfo, img_url: String, name: String, description: String, github: String, linkedin: String, twitter: String) -> Result<Response, ContractError> {
    
    let sender = info.sender;
    let profile = ProfileInfo {
        img_url: img_url,
        name: name,
        description: description,
        github:  github,
        linkedin: linkedin,
        twitter: twitter,
    };

    PROFILES.save(
        deps.storage,
        &sender,
        &profile
    )?;

    Ok(Response::new().add_attribute("method", "register"))
}

pub fn modify(deps: DepsMut, info: MessageInfo, img_url: String, name: String, description: String, github: String, linkedin: String, twitter: String) -> Result<Response, ContractError> {
    
    let sender = info.sender;

    PROFILES.update(
        deps.storage,
        &sender,
        |x| -> StdResult<_> {
            let mut profile = x.unwrap();
            profile.img_url = img_url;
            profile.name = name;
            profile.description = description;
            profile.github = github;
            profile.linkedin = linkedin;
            profile.twitter = twitter; 
            Ok(profile)
        },
        
    )?;

    Ok(Response::new().add_attribute("method", "modify"))
}

pub fn delete(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    PROFILES.remove(deps.storage, &info.sender);
    Ok(Response::new().add_attribute("method", "delete"))
}