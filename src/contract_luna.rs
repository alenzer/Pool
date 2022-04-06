#[cfg(not(feature = "library"))]

use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response,
    Uint128,  BankMsg
};

use crate::error::ContractError;
use crate::msg::{UserInfo, AprInfo, PayRequest};
use crate::state::{TREASURY, LUNA_APR_HISTORY, LUNA_USER_INFOS, LUNA_REWARDS_REQUEST, LUNA_WITHDRAW_REQUEST};
use crate::util::{check_onlyowner, luna_update_userinfo, compare_remove, append_amount_history};

pub fn try_set_apr_luna(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    apr: Uint128
)
    ->Result<Response, ContractError>
{
    check_onlyowner(deps.storage, info.sender)?;
    let mut apr_history = LUNA_APR_HISTORY.load(deps.storage)?;
    let apr_info = AprInfo{
        apr,
        time: env.block.time.seconds(),
    };

    apr_history.push(apr_info);
    LUNA_APR_HISTORY.save(deps.storage, &apr_history)?;
    Ok(Response::new())
}
pub fn try_deposit_luna(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    let _fund = info.funds.clone();
    let fund = &info.funds[0];

    if fund.denom != "uluna" {
        return Err(ContractError::PoolError{
            msg: "Invalid Fund Request".to_string()
        });
    }

    let res = LUNA_USER_INFOS.may_load(deps.storage, wallet.clone())?;
    let user_info = match res{
        Some(mut info) => {
            luna_update_userinfo(deps.storage, env.clone(), wallet.clone())?;

            info.amount += fund.amount;
            info
        },
        None => UserInfo{
            wallet: wallet.clone(),
            amount: fund.amount,
            reward_amount: Uint128::zero(),
            last_reward_time: Uint128::from(env.block.time.seconds() as u128),
        }
    };

    LUNA_USER_INFOS.save(deps.storage, wallet, &user_info)?;

    append_amount_history(deps.storage, env, Uint128::zero(), fund.amount, true)?;

    let send2_treasury = BankMsg::Send { 
        to_address: TREASURY.load(deps.storage)?.to_string(),
        amount: _fund
    };

    Ok(Response::new()
        .add_attribute("action", "desposit")
        .add_message(send2_treasury)
        .add_attribute("amount", fund.amount.to_string())
    )
}

pub fn try_request_withdraw_luna(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    luna_update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    let mut user_info = LUNA_USER_INFOS.load(deps.storage, wallet.clone())?;
    if user_info.amount < amount {
        return Err(ContractError::NotEnoughBalance { balance: amount });
    }
    
    user_info.amount -= amount;
    LUNA_USER_INFOS.save(deps.storage, wallet.clone(), &user_info)?;

    let mut request = LUNA_WITHDRAW_REQUEST.load(deps.storage)?;
    request.push(PayRequest{
        wallet,
        amount,
        time: Uint128::from(env.block.time.seconds() as u128)
    });
    LUNA_WITHDRAW_REQUEST.save(deps.storage, &request)?;

    append_amount_history(deps.storage, env, Uint128::zero(), amount, false)?;

    Ok(Response::new()
        .add_attribute("action", "withdraw")
    )
}

pub fn try_withdraw_luna(
    deps: DepsMut,
    info: MessageInfo,
    request: Vec<PayRequest>
)
    -> Result<Response, ContractError>
{
    if info.sender != TREASURY.load(deps.storage)? {
        return Err(ContractError::Unauthorized{ });
    }

    let mut withdraw_requests = LUNA_WITHDRAW_REQUEST.load(deps.storage)?;
    withdraw_requests = compare_remove(withdraw_requests, request)?;
    LUNA_WITHDRAW_REQUEST.save(deps.storage, &withdraw_requests)?;

    Ok(Response::new()
        .add_attribute("action", "withdraw")
    )
}

pub fn try_request_claimrewards_luna(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    luna_update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    let mut user_info = LUNA_USER_INFOS.load(deps.storage, wallet.clone())?;
 
    let mut request = LUNA_REWARDS_REQUEST.load(deps.storage)?;
    request.push(PayRequest{
        wallet: wallet.clone(),
        amount: user_info.reward_amount,
        time: Uint128::from(env.block.time.seconds() as u128)
    });
    LUNA_REWARDS_REQUEST.save(deps.storage, &request)?;

    user_info.reward_amount = Uint128::zero();
    LUNA_USER_INFOS.save(deps.storage, wallet, &user_info)?;

    Ok(Response::new())
}

pub fn try_claimrewards_luna(
    deps: DepsMut,
    info: MessageInfo,
    request: Vec<PayRequest>,
)
    -> Result<Response, ContractError>
{
    if info.sender != TREASURY.load(deps.storage)? {
        return Err(ContractError::Unauthorized{ });
    }

    let mut claim_rewards_requests = LUNA_REWARDS_REQUEST.load(deps.storage)?;
    claim_rewards_requests = compare_remove(claim_rewards_requests, request)?;
    LUNA_REWARDS_REQUEST.save(deps.storage, &claim_rewards_requests)?;

    Ok(Response::new()
        .add_attribute("action", "claim rewards")
    )
}