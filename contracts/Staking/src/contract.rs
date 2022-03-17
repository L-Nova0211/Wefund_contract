#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response,
    Uint128, CosmosMsg, WasmMsg, Storage
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, UserInfo, CardInfo, CardType};
use crate::state::{USER_INFOS, CARD_INFOS, OWNER, REWARD_TOKEN, START_TIME, 
    PLATIUM_CARD_NUMBER, GOLD_CARD_NUMBER, SILVER_CARD_NUMBER, BRONZE_CARD_NUMBER};
use crate::util::{check_onlyowner, get_cardtype, manage_card, get_reward,
        update_userinfo, get_token_balance};

const WFD_TOKEN: &str = "terra1pkytkcanua4uazlpekve7qyhg2c5xwwjr4429d";

// version info for migration info
const CONTRACT_NAME: &str = "Staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());
    OWNER.save(deps.storage, &owner)?;

    let reward_token = msg
        .reward_token
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(Addr::unchecked(WFD_TOKEN));
    REWARD_TOKEN.save(deps.storage, &reward_token)?;

    let start_time = match msg.start_time{
        Some(time) => time,
        None => Uint128::from(env.block.time.seconds())
    };
    START_TIME.save(deps.storage, &start_time)?;

    CARD_INFOS.save(deps.storage, &Vec::new())?;
    PLATIUM_CARD_NUMBER.save(deps.storage, &Uint128::zero())?;
    GOLD_CARD_NUMBER.save(deps.storage, &Uint128::zero())?;
    SILVER_CARD_NUMBER.save(deps.storage, &Uint128::zero())?;
    BRONZE_CARD_NUMBER.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetConfig{ owner, start_time, reward_token}
            => try_setconfig(deps, info, owner, start_time, reward_token),

        ExecuteMsg::Deposit { wallet, amount }
            => try_deposit(deps, env, info, wallet, amount),
        
        ExecuteMsg::Withdraw { wallet, amount }
            => try_withdraw(deps, env, info, wallet, amount),

        ExecuteMsg::ClaimRewards { wallet }
            => try_claimrewards(deps, env, info, wallet)
    }
}

pub fn try_setconfig(
    deps:DepsMut, 
    info:MessageInfo, 
    _owner: Option<Addr>,
    _start_time: Option<Uint128>,
    _reward_token: Option<Addr>
)
    -> Result<Response, ContractError>
{
    check_onlyowner(deps.storage, info.sender.clone())?;

    let mut owner = OWNER.load(deps.storage)?;
    owner = match _owner{
        Some(admin) => admin,
        None => owner
    };
    OWNER.save(deps.storage, &owner)?;

    let mut start_time = START_TIME.load(deps.storage)?;
    start_time = match _start_time{
        Some(time) => time,
        None => start_time
    };
    START_TIME.save(deps.storage, &start_time)?;

    let mut reward_token = REWARD_TOKEN.load(deps.storage)?;
    reward_token = match _reward_token{
        Some(token) => token,
        None => reward_token
    };
    REWARD_TOKEN.save(deps.storage, &reward_token)?;

    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}

pub fn try_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wallet: Addr,
    amount: Uint128
)
    -> Result<Response, ContractError>
{
    let res = USER_INFOS.may_load(deps.storage, wallet.clone())?;
    let mut user_info = match res{
        Some(info) => {
            update_userinfo(deps.storage, env.clone(), wallet.clone())?;

            let mut info = USER_INFOS.load(deps.storage, wallet.clone())?;
            info.amount += amount;
            info
        },
        None => UserInfo{
            wallet: wallet.clone(),
            amount: amount,
            last_withdraw_time: Uint128::from(env.block.time.seconds() as u128),
            reward_amount: Uint128::zero(),
            last_reward_time: Uint128::from(env.block.time.seconds() as u128),
            card_type: CardType::Other,
            card_number: Uint128::zero()
        }
    };

    let card_type = get_cardtype(deps.storage, user_info.amount)?;
    user_info.card_number = manage_card(
        deps.storage, 
        wallet.clone(),
        user_info.card_type, 
        user_info.card_number, 
        card_type.clone()
    )?;
    user_info.card_type = card_type.clone();

    USER_INFOS.save(deps.storage, wallet, &user_info)?;
    Ok(Response::new()
        .add_attribute("action", "desposit"))
}

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wallet: Addr,
    amount: Uint128
)
    -> Result<Response, ContractError>
{
    let mut user_info = USER_INFOS.load(deps.storage, wallet.clone())?;
    if user_info.amount < amount {
        return Err(ContractError::NotEnoughBalance { balance: amount });
    }
    update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    let token = REWARD_TOKEN.load(deps.storage)?;
    let balance = get_token_balance(&deps.querier, token.clone(), env.contract.address)?;
    if balance < amount {
        return Err(ContractError::NotEnoughBalance { balance });
    }

    user_info.amount -= amount;
    let card_type = get_cardtype(deps.storage, user_info.amount)?;
    user_info.card_number = manage_card(
        deps.storage, 
        wallet.clone(),
        user_info.card_type, 
        user_info.card_number, 
        card_type.clone()
    )?;
    user_info.card_type = card_type.clone();
    user_info.last_withdraw_time = Uint128::from(env.block.time.seconds() as u128);
    USER_INFOS.save(deps.storage, wallet.clone(), &user_info)?;

    let msg = WasmMsg::Execute { 
        contract_addr: token.to_string(), 
        msg: to_binary(
            &Cw20ExecuteMsg::Transfer { 
                recipient: wallet.to_string(), 
                amount: user_info.reward_amount
            }
        )?, 
        funds: vec![]
    };
    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_message(msg)
    )
}

pub fn try_claimrewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wallet: Addr,
)
    -> Result<Response, ContractError>
{
    update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    let mut user_info = USER_INFOS.load(deps.storage, wallet.clone())?;
    let token = REWARD_TOKEN.load(deps.storage)?;
    let balance = get_token_balance(&deps.querier, token.clone(), env.contract.address)?;

    if balance < user_info.reward_amount {
        return Err(ContractError::NotEnoughBalance { balance });
    }

    let msg = WasmMsg::Execute { 
        contract_addr: token.to_string(), 
        msg: to_binary(
            &Cw20ExecuteMsg::Transfer { 
                recipient: wallet.to_string(), 
                amount: user_info.reward_amount
            }
        )?, 
        funds: vec![]
    };
    user_info.reward_amount = Uint128::zero();

    USER_INFOS.save(deps.storage, wallet, &user_info)?;
    Ok(Response::new()
        .add_attribute("action", "claim rewards")
        .add_message(msg)
    )
}