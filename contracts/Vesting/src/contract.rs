#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response,
    Uint128, CosmosMsg, WasmMsg, Storage
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ProjectInfo, UserInfo, VestingParameter, Config};
use crate::state::{PROJECT_INFOS, OWNER};

// version info for migration info
const CONTRACT_NAME: &str = "Vesting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .admin
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());
    OWNER.save(deps.storage, &owner)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetConfig{ admin }
            => try_setconfig(deps, info, admin),

        ExecuteMsg::StartRelease{ project_id, start_time }
            => try_startrelease(deps, info, project_id, start_time),

        ExecuteMsg::AddProject{ project_id, admin, token_addr, vesting_params, start_time }
            => try_addproject(deps, info, project_id, admin, token_addr, vesting_params, start_time ),

        ExecuteMsg::SetProjectInfo{ project_id, project_info }
            => try_setprojectinfo(deps, info, project_id, project_info ),

        ExecuteMsg::SetProjectConfig{ project_id, admin, token_addr , start_time} 
            => try_setprojectconfig(deps, info, project_id, admin, token_addr, start_time),

        ExecuteMsg::SetVestingParameters{ project_id, params }
            => try_setvestingparameters(deps, info, project_id, params),

        ExecuteMsg::SetUsers { project_id, stage, user_infos } 
            =>  try_setusers(deps, info, project_id, stage, user_infos),

        ExecuteMsg::AddUser { project_id, stage, wallet, amount } 
            =>  try_adduser(deps, info, project_id, stage, wallet, amount),

        ExecuteMsg::ClaimPendingTokens { project_id, }
            =>  try_claimpendingtokens(deps, _env, info, project_id )
    }
}

pub fn try_startrelease(deps: DepsMut, info:MessageInfo, project_id: Uint128, start_time: Uint128)
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.config.start_time = start_time;
    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;
    Ok(Response::new()
    .add_attribute("action", "Start Release"))  
}

pub fn try_setprojectinfo(deps: DepsMut, info: MessageInfo, project_id: Uint128, project_info: ProjectInfo)
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x = project_info;
    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;
    Ok(Response::new()
    .add_attribute("action", "set Project Info"))    
}
pub fn try_setvestingparameters(deps: DepsMut, info: MessageInfo, project_id: Uint128, params: Vec<VestingParameter>)
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.vest_param = params;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;
    Ok(Response::new()
    .add_attribute("action", "Set Vesting parameters"))
}

pub fn calc_pending(store: &dyn Storage, _env: Env, project_id: Uint128, user: UserInfo, stage: usize)
    -> Uint128
{
    let x = PROJECT_INFOS.load(store, project_id.u128().into()).unwrap();
    if x.config.start_time == Uint128::zero() {
        return Uint128::zero();
    }

    let param = x.vest_param[stage];

    let past_time = Uint128::new(_env.block.time.seconds() as u128) - x.config.start_time;

    let mut unlocked = Uint128::zero();
    if past_time > Uint128::zero() {
        unlocked = user.total_amount * param.soon / Uint128::new(100);
    }
    let locked = user.total_amount - unlocked;
    if past_time > param.after {
        unlocked += (past_time - param.after) * locked / param.period;
        if unlocked >= user.total_amount{
            unlocked = user.total_amount;
        }
    }

    return unlocked - user.released_amount;
}

pub fn try_claimpendingtokens(deps: DepsMut, _env: Env, info: MessageInfo, project_id: Uint128 )
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let mut amount = Uint128::zero();
    for i in 0..x.users.len()-1{
        let index = x.users[i].iter().position(|x| x.wallet_address == info.sender);
        if index != None {
            let pending_amount = calc_pending(
                deps.storage, _env.clone(), project_id, x.users[i][index.unwrap()].clone(), i
            );
            x.users[i][index.unwrap()].released_amount += pending_amount;
            amount += pending_amount;
        }
    }

    if amount == Uint128::zero() {
        return Err(ContractError::NoPendingTokens{});
    }

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    let token_info: TokenInfoResponse = deps.querier.query_wasm_smart(
        x.config.token_addr.clone(),
        &Cw20QueryMsg::TokenInfo{}
    )?;
    amount = amount * Uint128::new((10 as u128).pow(token_info.decimals as u32)); //for decimals

    let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        x.config.token_addr.clone(),
        &Cw20QueryMsg::Balance{
            address: _env.contract.address.to_string(),
        }
    )?;
    if token_balance.balance < amount {
        return Err(ContractError::NotEnoughBalance{})
    }

    let bank_cw20 = WasmMsg::Execute {
        contract_addr: String::from(x.config.token_addr),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount: amount,
        }).unwrap(),
        funds: Vec::new()
    };

    Ok(Response::new()
    .add_message(CosmosMsg::Wasm(bank_cw20))
    .add_attribute("action", "Claim pending tokens"))
}

pub fn check_add_userinfo( users: &mut Vec<UserInfo>, wallet:Addr, amount: Uint128)
{
    let index =users.iter().position(|x| x.wallet_address == wallet);
    if index == None {
        users.push(UserInfo { 
            wallet_address: wallet, 
            total_amount: amount, 
            released_amount: Uint128::zero(), 
            pending_amount: Uint128::zero() 
        });
    }
    else{
        users[index.unwrap()].total_amount += amount;
    }
}
pub fn try_adduser(deps: DepsMut, info: MessageInfo, project_id: Uint128, stage:Uint128, wallet:Addr, amount: Uint128)
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    check_add_userinfo(&mut x.users[stage.u128() as usize], wallet, amount);
    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Add  User info"))
}

pub fn try_setusers(deps: DepsMut, info: MessageInfo, project_id: Uint128, stage:Uint128, user_infos: Vec<UserInfo>)
    ->Result<Response, ContractError>
{
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.users[stage.u128() as usize] = user_infos;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;

    Ok(Response::new()
    .add_attribute("action", "Set User infos for Seed stage"))
}

pub fn try_setprojectconfig(deps:DepsMut, info:MessageInfo,
    project_id: Uint128,
    admin: String, 
    token_addr: String,
    start_time: Uint128
) -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let mut x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner && info.sender != x.config.owner {
        return Err(ContractError::Unauthorized{ });
    }

    x.config.owner = deps.api.addr_validate(admin.as_str())?;
    x.config.token_addr = token_addr;
    x.config.start_time = start_time;

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &x)?;
    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}

pub fn try_addproject(deps:DepsMut, info:MessageInfo,
    project_id: Uint128,
    admin: String, 
    token_addr: String,
    vesting_params: Vec<VestingParameter>,
    start_time: Uint128
) -> Result<Response, ContractError>
{
    //-----------check owner--------------------------
    let owner = OWNER.load(deps.storage).unwrap();
    if info.sender != owner {
        return Err(ContractError::Unauthorized{});
    }

    let config: Config = Config{
        owner: deps.api.addr_validate(admin.as_str())?,
        token_addr : token_addr,
        start_time : start_time,
    };

    let mut _vesting_params = vesting_params;
    if _vesting_params.len() == 0{
        let sec_per_month = 60 * 60 * 24 * 30;
        let seed_param = VestingParameter {
            soon: Uint128::new(15), //15% unlock at tge
            after: Uint128::new(sec_per_month), //after 1 month
            period: Uint128::new(sec_per_month * 6) //release over 6 month
        };
        let presale_param = VestingParameter {
            soon: Uint128::new(20), //20% unlock at tge
            after: Uint128::new(sec_per_month), //ater 1 month
            period: Uint128::new(sec_per_month * 5) //release over 5 month
        };
        let ido_param = VestingParameter {
            soon: Uint128::new(25), //25% unlock at tge
            after: Uint128::new(sec_per_month), //after 1 month
            period: Uint128::new(sec_per_month * 4) //release over 4 month
        };
        _vesting_params = vec![seed_param, presale_param, ido_param];
    }

    let mut users = Vec::new();
    for _ in _vesting_params.clone(){
        users.push(Vec::new());
    }

    let project_info: ProjectInfo = ProjectInfo{
        project_id: project_id,
        config: config,
        vest_param: _vesting_params,
        users: users,
    };

    PROJECT_INFOS.save(deps.storage, project_id.u128().into(), &project_info)?;

    Ok(Response::new()
        .add_attribute("action", "add project"))                                
}
pub fn try_setconfig(deps:DepsMut, info:MessageInfo, admin: String) 
    -> Result<Response, ContractError>
{
    // //-----------check owner--------------------------
    // let owner = OWNER.load(deps.storage).unwrap();
    // if info.sender != owner {
    //     return Err(ContractError::Unauthorized{});
    // }

    let admin_addr = deps.api.addr_validate(&admin).unwrap();
    OWNER.save(deps.storage, &admin_addr)?;

    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}
