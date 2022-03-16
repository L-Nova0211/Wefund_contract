use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{UserInfo, CardInfo, CardType};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const START_TIME: Item<Uint128> = Item::new("start_time");
pub const REWARD_TOKEN: Item<Addr> = Item::new("reward_token");

pub const USER_INFOS: Map<Addr, UserInfo> = Map::new("user infos");
pub const CARD_INFOS: Item<Vec<CardInfo>> = Item::new("card infos");

pub const PLATIUM_CARD_NUMBER: Item<Uint128> = Item::new("platium card NUMBER");
pub const GOLD_CARD_NUMBER: Item<Uint128> = Item::new("gold card NUMBER");
pub const SILVER_CARD_NUMBER: Item<Uint128> = Item::new("silver card NUMBER");
pub const BRONZE_CARD_NUMBER: Item<Uint128> = Item::new("bronze card NUMBER");