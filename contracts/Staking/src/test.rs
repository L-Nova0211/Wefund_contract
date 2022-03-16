use super::*;
use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg,
    BankQuery, BalanceResponse, AllBalanceResponse, Coin, Uint128};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR, mock_dependencies};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};

// use crate::mock_querier::mock_dependencies;
use cw20::Cw20ExecuteMsg;
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::pair::ExecuteMsg as TerraswapExecuteMsg;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
 
}

