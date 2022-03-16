use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, Coin, StdResult, DepsMut};
use cw_storage_plus::{Item, Map, U128Key};
use crate::msg::{ProjectInfo};

pub const OWNER: Item<Addr> = Item::new("owner");

pub const PROJECT_INFOS:Map<U128Key, ProjectInfo> = Map::new("project_infos");
