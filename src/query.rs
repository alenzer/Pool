#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult,
    Uint128, QueryRequest, BankQuery,
    Coin, AllBalanceResponse,
};

use cw20::{ Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse };

use crate::msg::{QueryMsg, UserInfo};
use crate::state::{ OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, UST_REWARDS_REQUEST,
    UST_WITHDRAW_REQUEST,
    LUNA_APR_HISTORY, LUNA_USER_INFOS};
use crate::util::{ get_rewards };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner{ } => {
            to_binary(&OWNER.load(deps.storage)?)
        },

        QueryMsg::GetTreasury{ } => {
            to_binary(&TREASURY.load(deps.storage)?)
        },
        
        QueryMsg::GetHistoryOfAPRUST{ } => {
            to_binary(&UST_APR_HISTORY.load(deps.storage)?)
        },

        QueryMsg::GetUserInfoUST{ wallet } => {
            to_binary(&UST_USER_INFOS.load(deps.storage, wallet)?)
        },

        QueryMsg::GetPendingRewardsUST{ wallet } => {
            let user_info = UST_USER_INFOS.load(deps.storage, wallet.clone())?;
            let apr_history = UST_APR_HISTORY.load(deps.storage)?;
            let current_time = Uint128::from(env.block.time.seconds() as u128);
            let rewards = get_rewards(apr_history, user_info.clone(), current_time)?;
            to_binary(&(rewards + user_info.reward_amount))
        },

        QueryMsg::GetWithdrawRequstUST{ } => {
            to_binary(&UST_WITHDRAW_REQUEST.load(deps.storage)?)
        },

        QueryMsg::GetClaimRewardsRequestUST{ } => {
            to_binary(&UST_REWARDS_REQUEST.load(deps.storage)?)
        }
    }
}