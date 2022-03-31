#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult,
    Uint128, QueryRequest, BankQuery,
    Coin, AllBalanceResponse,
};

use cw20::{ Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse };

use crate::msg::{QueryMsg, UserInfo};
use crate::state::{ OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, 
    LUNA_APR_HISTORY, LUNA_USER_INFOS};
// use crate::util::{ get_reward };

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner{ } => {
            to_binary(&OWNER.load(deps.storage)?)
        },

        QueryMsg::GetHistoryOfAPR{ } => {
            to_binary(&Uint128::zero())
        },

        QueryMsg::GetUserInfoUST{ wallet: Addr } => {
            to_binary(&Uint128::zero())
        },

        QueryMsg::GetPendingRewardsUST{ wallet: Addr } => {
            to_binary(&Uint128::zero())
        },
    }
}