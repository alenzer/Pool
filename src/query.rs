#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult, Uint128
};

use crate::msg::{QueryMsg};
use crate::state::{ OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, UST_REWARDS_REQUEST,
    UST_WITHDRAW_REQUEST, LUNA_APR_HISTORY, LUNA_USER_INFOS, LUNA_REWARDS_REQUEST,
    LUNA_WITHDRAW_REQUEST};
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
        
        QueryMsg::GetHistoryOfAprUst{ } => {
            to_binary(&UST_APR_HISTORY.load(deps.storage)?)
        },

        QueryMsg::GetUserInfoUst{ wallet } => {
            to_binary(&UST_USER_INFOS.load(deps.storage, wallet)?)
        },

        QueryMsg::GetPendingRewardsUst{ wallet } => {
            let user_info = UST_USER_INFOS.load(deps.storage, wallet.clone())?;
            let apr_history = UST_APR_HISTORY.load(deps.storage)?;
            let current_time = Uint128::from(env.block.time.seconds() as u128);
            let rewards = get_rewards(apr_history, user_info.clone(), current_time)?;
            to_binary(&(rewards + user_info.reward_amount))
        },

        QueryMsg::GetWithdrawRequstUst{ } => {
            to_binary(&UST_WITHDRAW_REQUEST.load(deps.storage)?)
        },

        QueryMsg::GetClaimRewardsRequestUst{ } => {
            to_binary(&UST_REWARDS_REQUEST.load(deps.storage)?)
        }
//-----------------------------------------------------------------
        QueryMsg::GetHistoryOfAprLuna{ } => {
            to_binary(&LUNA_APR_HISTORY.load(deps.storage)?)
        },

        QueryMsg::GetUserInfoLuna{ wallet } => {
            to_binary(&LUNA_USER_INFOS.load(deps.storage, wallet)?)
        },

        QueryMsg::GetPendingRewardsLuna{ wallet } => {
            let user_info = LUNA_USER_INFOS.load(deps.storage, wallet.clone())?;
            let apr_history = LUNA_APR_HISTORY.load(deps.storage)?;
            let current_time = Uint128::from(env.block.time.seconds() as u128);
            let rewards = get_rewards(apr_history, user_info.clone(), current_time)?;
            to_binary(&(rewards + user_info.reward_amount))
        },

        QueryMsg::GetWithdrawRequstLuna{ } => {
            to_binary(&LUNA_WITHDRAW_REQUEST.load(deps.storage)?)
        },

        QueryMsg::GetClaimRewardsRequestLuna{ } => {
            to_binary(&LUNA_REWARDS_REQUEST.load(deps.storage)?)
        }
    }
}