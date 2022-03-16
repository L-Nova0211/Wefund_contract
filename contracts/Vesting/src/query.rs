#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult,
    Uint128, QueryRequest, BankQuery,
    Coin, AllBalanceResponse,
};

use cw20::{ Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse };

use crate::msg::{QueryMsg, Config, ProjectInfo, UserInfo};
use crate::state::{PROJECT_INFOS, OWNER};
use crate::contract::{ calc_pending };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance{ project_id, wallet } => 
            to_binary(&query_balance(deps, _env, project_id, wallet)?),
            
        QueryMsg::GetConfig{ project_id } => 
            to_binary(&query_getconfig(deps, project_id)?),

        QueryMsg::GetProjectInfo{ project_id } => 
            to_binary(&query_getprojectinfo(deps, project_id)?),

        QueryMsg::GetPendingTokens{ project_id, wallet } => 
            to_binary(&query_pendingtokens(deps, _env, project_id, wallet)?),

        QueryMsg::GetAllProjectInfo{ } =>
            to_binary(&query_getallprojectinfo(deps)?),

        QueryMsg::GetOwner{ } => {
            let owner = OWNER.load(deps.storage).unwrap();
            to_binary(&owner)
        }

        QueryMsg::GetUserInfo{ project_id, wallet } =>
            to_binary(&query_getuserinfo(deps, project_id, wallet)?),
    }
}
fn query_pendingtokens(deps:Deps, _env:Env, project_id: Uint128, wallet: String) 
    -> StdResult<Uint128> 
{
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;

    let mut amount = Uint128::zero();
    for i in 0..x.users.len()-1{
        let index = x.users[i].iter().position(|x| x.wallet_address == wallet);
        if index != None {
            let pending_amount = calc_pending(
                deps.storage, _env.clone(), project_id, x.users[i][index.unwrap()].clone(), i
            );
            amount += pending_amount;
        }
    }

    Ok(amount)
}
fn query_getprojectinfo(deps:Deps, project_id: Uint128) -> StdResult<ProjectInfo>{
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    Ok(x)
}

fn query_balance(deps:Deps, _env:Env, project_id: Uint128, wallet:String) -> StdResult<AllBalanceResponse>{

    // let uusd_denom = String::from("uusd");
    let mut balance: AllBalanceResponse = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances {
            address: wallet.clone(),
        }
    ))?;

    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;

    let token_balance: Cw20BalanceResponse = deps.querier.query_wasm_smart(
        x.clone().config.token_addr,
        &Cw20QueryMsg::Balance{
            address: wallet,
        }
    )?;
    let token_info: TokenInfoResponse = deps.querier.query_wasm_smart(
        x.config.token_addr.clone(),
        &Cw20QueryMsg::TokenInfo{}
    )?;
    balance.amount.push(Coin::new(token_balance.balance.u128(), token_info.name));

    Ok(balance)
}
fn query_getconfig(deps:Deps, project_id: Uint128) -> StdResult<Config> {
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    Ok(x.config)
}
fn query_getallprojectinfo(deps: Deps) -> StdResult<Vec<ProjectInfo>>
{
    let all: StdResult<Vec<_>> = PROJECT_INFOS.range(deps.storage, None, None, 
        cosmwasm_std::Order::Ascending).collect();
    let all = all.unwrap();

    let mut all_project:Vec<ProjectInfo> = Vec::new();
    for x in all{
        all_project.push(x.1);
    }
    Ok(all_project)
}

fn query_getuserinfo(deps:Deps, project_id: Uint128, wallet: String) 
    -> StdResult<UserInfo> 
{
    let x = PROJECT_INFOS.load(deps.storage, project_id.u128().into())?;
    let mut user_info: UserInfo = UserInfo{
        total_amount: Uint128::zero(),
        released_amount: Uint128::zero(),
        wallet_address: deps.api.addr_validate(&wallet).unwrap(),
        pending_amount: Uint128::zero()
    };

    for i in 0..x.users.len()-1{
        let index = x.users[i].iter().position(|x| x.wallet_address == wallet);
        if index != None {
            user_info.total_amount += x.users[i][index.unwrap()].total_amount;
            user_info.released_amount += x.users[i][index.unwrap()].released_amount;
        }
    }

    Ok(user_info)
}