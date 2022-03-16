use cosmwasm_std::{Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Milestone, TeamMember, VestingParameter};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub wefund: Option<String>,
    pub anchor_market: Option<String>,
    pub aust_token: Option<String>,
    pub vesting_contract: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig { admin:Option<String>,  wefund: Option<String>, 
        anchor_market: Option<String>, aust_token:Option<String> , 
        vesting_contract:Option<String>},
    AddProject { 
        project_company: String,
        project_title: String,
        project_description: String,
        project_ecosystem: String,
        project_createddate: String,
        project_saft: String,
        project_logo: String,
        project_whitepaper: String,
        project_website: String,
        project_email: String,
        creator_wallet: String,
        project_collected: Uint128,
        project_milestones: Vec<Milestone>,
        project_teammembers: Vec<TeamMember>,
        vesting: Vec<VestingParameter>,
        token_addr: String
    },
    RemoveProject{project_id: Uint128 },

    Back2Project { project_id: Uint128, backer_wallet: String, 
        fundraising_stage: Uint128, token_amount: Uint128, 
        otherchain:String, otherchain_wallet:String},

    CompleteProject{ project_id: Uint128 },
    FailProject{project_id: Uint128 },

    TransferAllCoins{wallet: String},

    AddCommunitymember{wallet: String},
    RemoveCommunitymember{wallet: String},

    WefundApprove{project_id:Uint128},
    SetFundraisingStage{project_id: Uint128, stage: Uint128},
    
    SetMilestoneVote{project_id: Uint128, wallet:String, voted: bool},

    ReleaseMilestone{project_id: Uint128},

    SetProjectStatus{project_id: Uint128, status: Uint128}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig{},
    GetAllProject{},
    GetProject { project_id:Uint128 },
    GetBacker{ project_id:Uint128},
    GetBalance{ wallet:String },
    GetCommunitymembers{},
}

