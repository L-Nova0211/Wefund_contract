use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub start_time: Option<Uint128>,
    pub reward_token: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig {
        owner: Option<Addr>,
        start_time: Option<Uint128>,
        reward_token: Option<Addr>
    },
    Deposit {
        wallet: Addr,
        amount: Uint128,
    },
    Withdraw {
        wallet: Addr,
        amount: Uint128,
    },
    ClaimRewards{
        wallet: Addr,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner{ },
    GetTokenAddress{ },
    GetStartTime{ },
    GetUserInfo{ wallet: Addr },
    GetPendingRewards{ wallet: Addr },
    GetCardInfo{ }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet: Addr,
	pub amount: Uint128,
    pub last_withdraw_time: Uint128,
	pub reward_amount: Uint128,
    pub last_reward_time: Uint128,
    pub card_type: CardType,
    pub card_number: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum CardType{
    Platium,
    Gold,
    Silver,
    Bronze,
    Other
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CardInfo{
    pub wallet: Addr,
    pub card_type: CardType,
    pub card_number: Uint128,
    pub metadata: String
}