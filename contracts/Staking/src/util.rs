use crate::error::ContractError;

use cosmwasm_std::{ Storage, Uint128, Addr, StdResult, StdError, Response, Env, QuerierWrapper, Querier, BalanceResponse};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::state::{ OWNER, PLATIUM_CARD_NUMBER, GOLD_CARD_NUMBER, SILVER_CARD_NUMBER,
    BRONZE_CARD_NUMBER, CARD_INFOS, USER_INFOS, REWARD_TOKEN, START_TIME,
};
use crate::msg::{CardType, CardInfo};

pub fn check_onlyowner(storage: &dyn Storage, sender: Addr) -> Result<Response, ContractError> {
    let owner = OWNER.load(storage)?;
    if owner != sender {
        return Err(ContractError::Unauthorized{});
    }
    Ok(Response::new())
}

pub fn get_cardtype(storage: &dyn Storage, amount: Uint128) -> StdResult<CardType>
{
    let platium = Uint128::from(100_000u128);
    let gold = Uint128::from(40_000u128);
    let silver = Uint128::from(10_000u128);
    let bronz = Uint128::from(1_000u128);
    if amount >= platium {
        return Ok(CardType::Platium);
    } else if amount >= gold {
        return Ok(CardType::Gold);
    } else if amount >= silver {
        return Ok(CardType::Silver);
    } else if amount >= bronz {
        return Ok(CardType::Bronze)
    } else {
        return Ok(CardType::Other )
    }
}

pub fn manage_card(storage: &mut dyn Storage, 
    wallet: Addr,
    prev_card_type: CardType, 
    prev_card_number: Uint128,
    cur_card_type: CardType
)
    -> StdResult<Uint128>
{
    if prev_card_type == cur_card_type {
        return Ok(prev_card_number);
    }
    
    let card_number = match cur_card_type {
        CardType::Platium => {
            let mut number = PLATIUM_CARD_NUMBER.load(storage)?;
            number += Uint128::from(1u128);
            PLATIUM_CARD_NUMBER.save(storage, &number)?;
            number
        },
        CardType::Gold => {
            let mut number = GOLD_CARD_NUMBER.load(storage)?;
            number += Uint128::from(1u128);
            GOLD_CARD_NUMBER.save(storage, &number)?;
            number
        },
        CardType::Silver => {
            let mut number = SILVER_CARD_NUMBER.load(storage)?;
            number += Uint128::from(1u128);
            SILVER_CARD_NUMBER.save(storage, &number)?;
            number
        },
        CardType::Bronze => {
            let mut number = BRONZE_CARD_NUMBER.load(storage)?;
            number += Uint128::from(1u128);
            BRONZE_CARD_NUMBER.save(storage, &number)?;
            number
        },
        CardType::Other => {
            Uint128::zero()
        },
    };

    remove_cardinfo(storage, wallet.clone(), prev_card_type, prev_card_number)?;
    if cur_card_type != CardType::Other {
        register_cardinfo(storage, wallet, cur_card_type, card_number)?;
    }
    Ok(card_number)
}

pub fn remove_cardinfo(storage: &mut dyn Storage, wallet: Addr, card_type: CardType, card_number: Uint128)
    -> StdResult<bool>
{
    let mut card_infos = CARD_INFOS.load(storage)?;
    card_infos.retain(|x| x.wallet != wallet 
        || x.card_type != card_type || x.card_number != card_number
    );
    CARD_INFOS.save(storage, &card_infos)?;
    Ok(true)
}
pub fn register_cardinfo(storage: &mut dyn Storage, wallet: Addr, card_type: CardType, card_number: Uint128)
    -> StdResult<bool>
{
    let mut card_infos = CARD_INFOS.load(storage)?;
    let card = CardInfo{
        wallet,
        card_type,
        card_number,
        metadata: "".to_string()
    };
    card_infos.push(card);
    CARD_INFOS.save(storage, &card_infos)?;
    Ok(true)
}

pub fn get_reward(amount: Uint128, card_type: CardType, from: Uint128, to: Uint128)
    -> StdResult<(Uint128, Uint128)>
{
    let (reward_percent, extra_staking_percent) = match card_type{
        CardType::Platium => (30, 1),
        CardType::Gold => (30, 1),
        CardType::Silver => (10, 0),
        CardType::Bronze => (1, 0),
        CardType::Other => (0, 0),
    };
    let reward_percent = Uint128::from(reward_percent as u128);
    let extra_staking_percent = Uint128::from(extra_staking_percent as u128);
    let month = Uint128::from((60 * 60 * 24 * 30) as u128);
    let reward = amount * reward_percent * (to - from)/ Uint128::from(1_000u128)/ month;
    let extra = amount * extra_staking_percent * (to - from)/ Uint128::from(1_000u128)/ month;
    Ok((reward, extra))
}

pub fn update_userinfo(storage: &mut dyn Storage, env: Env, wallet: Addr)
    ->StdResult<bool>
{
    let mut user_info = USER_INFOS.load(storage, wallet.clone())?;
    let current_time = Uint128::from(env.block.time.seconds() as u128);
    let start_time = START_TIME.load(storage)?;

    if current_time < start_time {
        return Err(StdError::GenericErr { msg: "Not started".to_string() });
    }

    let _user_info = user_info.clone();
    let mut from = user_info.last_reward_time;
    if from < start_time{
        from = start_time;
    }

    let (rewards, extra_staking) = get_reward(
        user_info.amount, 
        user_info.card_type, 
        from, 
        current_time.clone()
    )?;
    
    user_info.reward_amount += rewards;
    user_info.last_reward_time = current_time.clone();

    let month = Uint128::from((60*60*24*30) as u128);
    if user_info.last_withdraw_time + month < current_time {
        user_info.amount += extra_staking;
    }

    USER_INFOS.save(storage, wallet, &_user_info)?;
    Ok(true)
}

pub fn get_token_balance(
    querier: &QuerierWrapper, 
    token: Addr,
    wallet: Addr
)
    ->StdResult<Uint128>
{
    let res: Cw20BalanceResponse = querier.query_wasm_smart(
        token, 
    &Cw20QueryMsg::Balance { 
            address: wallet.to_string() 
        }
    )?;
    Ok(res.balance)
}