use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::{Uint128};
use crate::state::{ProjectStatus};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Project id is already registerd")]
    AlreadyRegisteredProject {},

    #[error("Project id is not registerd yet")]
    NotRegisteredProject {},

    #[error("Need some coin")]
    NeedCoin{},

    #[error("Alreay enough collected")]
    AlreadyCollected{},

    #[error("Alreay done or failed")]
    AlreadyDoneFail{},

    #[error("Invalid Address")]
    InvalidAddress{},

    #[error("Already registered community member")]
    AlreadyRegisteredCommunity{},

    #[error("Not registered community member")]
    NotRegisteredCommunity{},

    #[error("Not correct status : {status}")]
    NotCorrectStatus{
        status: u32,
    },

    #[error("Alreay voted")]
    AlreadyVoted{},

    #[error("Not voted")]
    NotVoted{},

    #[error("Not backer wallet")]
    NotBackerWallet{},

    #[error("Not found Milestone index")]
    NotFoundMilestoneIndex{},

    #[error("Not correct Milestone status : step{step}:{status}")]
    NotCorrectMilestoneStatus{
        step:usize, status: Uint128,
    },

    #[error("Not correct Milestone status : {aust_balance} {estimate_exchange_rate} {epoch_exchange_rate} {withdraw_amount} {release_amount}")]
    Testing{
        aust_balance: String,
        estimate_exchange_rate: String,
        epoch_exchange_rate: String,
        withdraw_amount: String,
        release_amount: String
    }
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
