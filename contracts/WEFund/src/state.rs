use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Coin, StdResult, Storage};
use cw_storage_plus::{Item, Map, U128Key};
//------------Config---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub wefund: Addr,
    pub anchor_market: Addr,
    pub aust_token: Addr,
    pub vesting_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

//-------------backer states---------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BackerState{
    pub backer_wallet: Addr,
    pub ust_amount: Coin,
    pub aust_amount: Coin,
    pub otherchain: String,
    pub otherchain_wallet: String,
}
//--------------Vote---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote{
    pub wallet: Addr,
    pub voted: bool,
}

//--------------Milestone---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Milestone{
    pub milestone_step: Uint128,
    pub milestone_name: String,
    pub milestone_description: String,
    pub milestone_startdate: String,
    pub milestone_enddate: String,
    pub milestone_amount: Uint128,
    pub milestone_status: Uint128, //0:voting, 1:releasing 2:released
    pub milestone_votes: Vec<Vote>,
}
//------------Team Description-------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TeamMember{
    pub teammember_description: String,
    pub teammember_linkedin: String,
    pub teammember_role: String,
}
//--------------Milestone---------------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VestingParameter{
    pub stage_title: String,
    pub stage_price: Uint128,
    pub stage_amount: Uint128,
    pub stage_soon: Uint128,
    pub stage_after: Uint128,
    pub stage_period: Uint128   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProjectStatus{
    WefundVote,
    Fundraising,
    Releasing,
    Done,
    Fail
}
//------------ project state--------------------------------
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProjectState{
//---------mata data----------------------------------------------------------
    pub project_company: String,
    pub project_title: String,
    pub project_description: String,
    pub project_ecosystem: String,
    pub project_createddate: String,
    pub project_saft: String,
    pub project_logo: String,
    pub project_whitepaper: String,
    pub project_website: String,
    pub project_email: String,
//------------------------------------------------------------------------------
    pub project_id: Uint128,
    pub creator_wallet: Addr,
    pub project_collected: Uint128,

    //0:wefund voting 1:fundrasing 2:releasing 3:done 4:fail
    pub project_status: ProjectStatus, 
    pub fundraising_stage: Uint128, 

    pub backerbacked_amount: Uint128,
    pub communitybacked_amount: Uint128,
//---------backer states for 50% of collected------------------------    
    pub backer_states: Vec<BackerState>,

//---------community backer states for 50% of collected---------------
    pub communitybacker_states: Vec<BackerState>,

//----------milestone states-----------------------------------------
    pub milestone_states: Vec<Milestone>,
    pub project_milestonestep: Uint128, 
//---------team members-----------------------------------------------
    pub teammember_states: Vec<TeamMember>,
//---------vesting-----------------------------------------------
    pub vesting: Vec<VestingParameter>,

    pub token_addr: Addr,
}
pub const PROJECT_SEQ: Item<Uint128> = Item::new("prj_seq");
pub const PROJECTSTATES: Map<U128Key, ProjectState> = Map::new("prj");

pub fn save_projectstate(store: &mut dyn Storage, _prj: &mut ProjectState) 
    -> StdResult<()> 
{
    // increment id if exists, or return 1
    let id = PROJECT_SEQ.load(store)?;
    let id = id.checked_add(Uint128::new(1))?;
    PROJECT_SEQ.save(store, &id)?;

    _prj.project_id = id.clone();
    PROJECTSTATES.save(store, id.u128().into(), &_prj)
}

//------------community array------------------------------------------------
pub const COMMUNITY: Item<Vec<Addr>> = Item::new("community");