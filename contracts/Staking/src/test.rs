use super::*;
use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg,
    BankQuery, BalanceResponse, AllBalanceResponse, Coin, Uint128, Api};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg, UserInfo, CardInfo};

use crate::mock_querier::{mock_dependencies};
use cw20::Cw20ExecuteMsg;
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::pair::ExecuteMsg as TerraswapExecuteMsg;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
    deps.querier.with_token_balances(&[
        (
            &"wfd".to_string(), 
            &[
                (&"user1".to_string(), &Uint128::from(1_000_000_000u128)),
                (&MOCK_CONTRACT_ADDR.to_string(),&Uint128::from(1_000_000_000u128)),
            ]
        )
    ]);
    
    let msg = InstantiateMsg{
        owner: Some(String::from("owner")),
        start_time: Some(Uint128::from(1u128)),
        reward_token: Some(String::from("wfd"))
    };
//instantiate
    let info = mock_info("owner", &[]);
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

//set starttime
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::SetConfig { 
        owner: None,
        start_time: Some(Uint128::from(10u128)),
        reward_token: None
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("SetConfig{:?}", res);

//define user
    let user1 = Addr::unchecked("user1".to_string());
    let user2 = Addr::unchecked("user2".to_string());
//deposit
    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::Deposit 
        { 
            wallet: user1.clone(), 
            amount: Uint128::from(1_000u128) 
        } ;
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("deposit{:?}", res);

//deposit
    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::Deposit 
        { 
            wallet: user1.clone(), 
            amount: Uint128::from(10_000u128) 
        } ;
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("deposit{:?}", res);

//deposit
    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::Deposit 
        { 
            wallet: user2.clone(), 
            amount: Uint128::from(1_000u128) 
        } ;
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("deposit{:?}", res);
// //withdraw
    let info = mock_info("user1", &[]);
    let msg = ExecuteMsg::Withdraw
        { 
            wallet: user1.clone(), 
            amount: Uint128::from(1_000u128) 
        } ;
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Withdraw{:?}", res);

//-Get owner-----------------
    let msg = QueryMsg::GetOwner{};
    let owner = query(deps.as_ref(), mock_env(), msg).unwrap();
    let res: Addr = from_binary(&owner).unwrap();
    println!("owner {:?}", res );

//get start time
    let msg = QueryMsg::GetStartTime {  };
    let owner = query(deps.as_ref(), mock_env(), msg).unwrap();
    let res: Addr = from_binary(&owner).unwrap();
    println!("start time {:?}", res );

//get user info
    let msg = QueryMsg::GetUserInfo { wallet: user1};
    let owner = query(deps.as_ref(), mock_env(), msg).unwrap();
    let res: UserInfo = from_binary(&owner).unwrap();
    println!("user info {:?}", res );    

//get card info
    let msg = QueryMsg::GetCardInfo { };
    let owner = query(deps.as_ref(), mock_env(), msg).unwrap();
    let res: Vec<CardInfo> = from_binary(&owner).unwrap();
    println!("Card info {:?}", res );    
}

