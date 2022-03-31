use crate::error::ContractError;

use cosmwasm_std::{ Storage, Uint128, Addr, StdResult, StdError, Response, Env, QuerierWrapper, Querier, BalanceResponse};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse as Cw20BalanceResponse, TokenInfoResponse};

use crate::state::{ OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, 
    LUNA_APR_HISTORY, LUNA_USER_INFOS
};
use crate::msg::{UserInfo};

pub fn check_onlyowner(storage: &dyn Storage, sender: Addr) -> Result<Response, ContractError> {
    let owner = OWNER.load(storage)?;
    if owner != sender {
        return Err(ContractError::Unauthorized{});
    }
    Ok(Response::new())
}

pub fn update_userinfo(storage: &mut dyn Storage, env: Env, wallet: Addr)
    ->StdResult<bool>
{
    // let mut user_info = USER_INFOS.load(storage, wallet.clone())?;
    // let current_time = Uint128::from(env.block.time.seconds() as u128);
    // let start_time = START_TIME.load(storage)?;

    // if current_time < start_time {
    //     return Err(StdError::GenericErr { msg: "Not started".to_string() });
    // }

    // let _user_info = user_info.clone();
    // let mut from = user_info.last_reward_time;
    // if from < start_time{
    //     from = start_time;
    // }

    // let (rewards, extra_staking) = get_reward(
    //     user_info.amount, 
    //     user_info.card_type, 
    //     from, 
    //     current_time.clone()
    // )?;
    
    // user_info.reward_amount += rewards;
    // user_info.last_reward_time = current_time.clone();

    // let month = Uint128::from((60*60*24*30) as u128);
    // if user_info.last_withdraw_time + month < current_time {
    //     user_info.amount += extra_staking;
    // }

    // USER_INFOS.save(storage, wallet, &_user_info)?;
    Ok(true)
}
