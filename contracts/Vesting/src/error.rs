use cosmwasm_std::StdError;
use thiserror::Error;
use cosmwasm_std::{Uint128};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("There is no pending tokens")]
    NoPendingTokens {},

    #[error("There is no enough tokens")]
    NotEnoughBalance {},

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
