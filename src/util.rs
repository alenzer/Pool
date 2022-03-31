use crate::error::ContractError;

use cosmwasm_std::{ Storage, Uint128, Addr, StdResult, StdError, Response, Env, QuerierWrapper, Querier, BalanceResponse};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::state::{ OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, 
    LUNA_APR_HISTORY, LUNA_USER_INFOS
};
use crate::msg::{UserInfo, PayRequest, AprInfo};

pub fn check_onlyowner(storage: &dyn Storage, sender: Addr) -> Result<Response, ContractError> {
    let owner = OWNER.load(storage)?;
    if owner != sender {
        return Err(ContractError::Unauthorized{});
    }
    Ok(Response::new())
}

pub fn get_multiplier(history: Vec<AprInfo>, _from: Uint128, to: Uint128)
    ->StdResult<Uint128>
{
    let mut sum = Uint128::zero();
    let mut from = _from;

    let mut k = 0;
    for i in 1 .. history.len() {
        let h0 = Uint128::from(history[i].time as u128);
        k = i;
        if h0 > to {
            break;
        }
        if h0 > from {
            sum += (h0 - from) * (history[i-1].apr - Uint128::from(1000u128));
            from = h0;
        }
    }
    sum += (to - from) * (history[k].apr  - Uint128::from(1000u128)) ;

    Ok(sum)
}
pub fn get_rewards(apr_history: Vec<AprInfo>, user_info: UserInfo, current_time: Uint128)
    -> StdResult<Uint128>
{
    let from = user_info.last_reward_time;
    let multiplier = get_multiplier( apr_history, from, current_time.clone())?;
    let month = Uint128::from((60*60*24*30) as u128);

    let rewards = user_info.amount * multiplier / month / Uint128::from(1000u128);
    Ok(rewards)
}
pub fn ust_update_userinfo(storage: &mut dyn Storage, env: Env, wallet: Addr)
    ->StdResult<bool>
{
    let mut user_info = UST_USER_INFOS.load(storage, wallet.clone())?;
    let apr_hstory = UST_APR_HISTORY.load(storage)?;
    let current_time = Uint128::from(env.block.time.seconds() as u128);

    let rewards = get_rewards(apr_hstory, user_info.clone(), current_time)?;

    user_info.reward_amount += rewards;
    user_info.last_reward_time = current_time.clone();

    UST_USER_INFOS.save(storage, wallet, &user_info)?;
    Ok(true)
}

pub fn luna_update_userinfo(storage: &mut dyn Storage, env: Env, wallet: Addr)
    ->StdResult<bool>
{
    let mut user_info = LUNA_USER_INFOS.load(storage, wallet.clone())?;
    let apr_hstory = LUNA_APR_HISTORY.load(storage)?;
    let current_time = Uint128::from(env.block.time.seconds() as u128);

    let rewards = get_rewards(apr_hstory, user_info.clone(), current_time)?;

    user_info.reward_amount += rewards;
    user_info.last_reward_time = current_time.clone();

    LUNA_USER_INFOS.save(storage, wallet, &user_info)?;
    Ok(true)
}

pub fn compare_remove(_a: Vec<PayRequest>, _b: Vec<PayRequest>)
    -> StdResult<Vec<PayRequest>>
{
    let mut A = _a;
    let mut B = _b;

    B.sort_by(|a, b| a.time.cmp(&b.time));
    let mut retain = vec![true; A.len()];

    let mut j = 0;
    for i in 0 .. A.len() {
        if j >= B.len() {
            break;
        }
        while A[i].time >= B[j].time {
            if B[j] == A[i] {
                retain[i] = false;
            }
            j += 1;
            if j >= B.len() {
                break;
            }
        }
    }

    let mut iter = retain.iter();
    A.retain(|_| *iter.next().unwrap());

    Ok(A)
}