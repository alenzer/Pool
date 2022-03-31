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

    #[error("There is no enough tokens: {:?}", balance)]
    NotEnoughBalance {balance: Uint128},

    #[error("Pool error: {:?}", msg)]
    PoolError {msg: String},
    
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
