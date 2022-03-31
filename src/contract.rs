#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response,
    Uint128, CosmosMsg, BankMsg, Storage, Coin
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, UserInfo, AprInfo, PayRequest};
use crate::state::{OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, 
    LUNA_APR_HISTORY, LUNA_USER_INFOS, UST_REWARDS_REQUEST, UST_WITHDRAW_REQUEST};
use crate::util::{check_onlyowner, ust_update_userinfo, compare_remove};

// version info for migration info
const CONTRACT_NAME: &str = "Pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok()) 
        .unwrap_or(info.sender.clone());
    OWNER.save(deps.storage, &owner)?;

    let treasury = deps.api.addr_validate(msg.treasury.as_str())?;
    TREASURY.save(deps.storage, &treasury)?;

    let mut ust_apr_history: Vec<AprInfo> = Vec::new();
    ust_apr_history.push(
        AprInfo{
            apr: msg.ust_apr,
            time: env.block.time.seconds()
        }
    );
    UST_APR_HISTORY.save(deps.storage, &ust_apr_history)?;

    let mut luna_apr_history: Vec<AprInfo> = Vec::new(); 
    luna_apr_history.push(
        AprInfo{
            apr: msg.luna_apr,
            time: env.block.time.seconds()
        }
    );
    LUNA_APR_HISTORY.save(deps.storage, &luna_apr_history)?;

    UST_REWARDS_REQUEST.save(deps.storage, &Vec::new())?;
    UST_WITHDRAW_REQUEST.save(deps.storage, &Vec::new())?;
    Ok(Response::new()
        .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetConfig{ owner, treasury }
            => try_setconfig(deps, info, owner, treasury),

        ExecuteMsg::SetAPRUST{ apr }
            => try_set_apr_ust(deps, env, info, apr),

        ExecuteMsg::DepositUST {  }
            => try_deposit_ust(deps, env, info),
        
        ExecuteMsg::RequestWithdrawUST{ amount }
            => try_request_withdraw_ust(deps, env, info, amount),

        ExecuteMsg::WithdrawUST { request }
            => try_withdraw_ust(deps, info, request),

        ExecuteMsg::RequestClaimRewardsUST{ }
            => try_request_claimrewards_ust(deps, env, info),

        ExecuteMsg::ClaimRewardsUST { request }
            => try_claimrewards_ust(deps, info, request)
    }
}

pub fn try_setconfig(
    deps:DepsMut, 
    info:MessageInfo, 
    owner: Option<Addr>,
    treasury: Option<Addr>,
)
    -> Result<Response, ContractError>
{
    check_onlyowner(deps.storage, info.sender.clone())?;

    match owner{
        Some(admin) => {
            OWNER.save(deps.storage, &admin)?
        },
        None => {}
    };

    match treasury{
        Some(wallet) => {
            TREASURY.save(deps.storage, &wallet)?
        },
        None => {}
    };

    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}
pub fn try_set_apr_ust(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    apr: Uint128
)
    ->Result<Response, ContractError>
{
    check_onlyowner(deps.storage, info.sender)?;
    let mut apr_history = UST_APR_HISTORY.load(deps.storage)?;
    let apr_info = AprInfo{
        apr,
        time: env.block.time.seconds(),
    };

    apr_history.push(apr_info);
    UST_APR_HISTORY.save(deps.storage, &apr_history)?;
    Ok(Response::new())
}
pub fn try_deposit_ust(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    let _fund = info.funds.clone();
    let fund = &info.funds[0];

    if fund.denom != "uusd" {
        return Err(ContractError::PoolError{
            msg: "Invalid Fund Request".to_string()
        });
    }

    let res = UST_USER_INFOS.may_load(deps.storage, wallet.clone())?;
    let user_info = match res{
        Some(mut info) => {
            ust_update_userinfo(deps.storage, env.clone(), wallet.clone())?;

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

    UST_USER_INFOS.save(deps.storage, wallet, &user_info)?;

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

pub fn try_request_withdraw_ust(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    ust_update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    let mut user_info = UST_USER_INFOS.load(deps.storage, wallet.clone())?;
    if user_info.amount < amount {
        return Err(ContractError::NotEnoughBalance { balance: amount });
    }
    
    user_info.amount -= amount;
    UST_USER_INFOS.save(deps.storage, wallet.clone(), &user_info)?;

    let mut request = UST_WITHDRAW_REQUEST.load(deps.storage)?;
    request.push(PayRequest{
        wallet,
        amount,
        time: Uint128::from(env.block.time.seconds() as u128)
    });
    UST_WITHDRAW_REQUEST.save(deps.storage, &request)?;

    Ok(Response::new()
        .add_attribute("action", "withdraw")
    )
}

pub fn try_withdraw_ust(
    deps: DepsMut,
    info: MessageInfo,
    request: Vec<PayRequest>
)
    -> Result<Response, ContractError>
{
    if info.sender != TREASURY.load(deps.storage)? {
        return Err(ContractError::Unauthorized{ });
    }

    let mut withdraw_requests = UST_WITHDRAW_REQUEST.load(deps.storage)?;
    withdraw_requests = compare_remove(withdraw_requests, request)?;
    UST_WITHDRAW_REQUEST.save(deps.storage, &withdraw_requests)?;

    Ok(Response::new()
        .add_attribute("action", "withdraw")
    )
}

pub fn try_request_claimrewards_ust(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    ust_update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    let mut user_info = UST_USER_INFOS.load(deps.storage, wallet.clone())?;
 
    let mut request = UST_REWARDS_REQUEST.load(deps.storage)?;
    request.push(PayRequest{
        wallet: wallet.clone(),
        amount: user_info.reward_amount,
        time: Uint128::from(env.block.time.seconds() as u128)
    });
    UST_REWARDS_REQUEST.save(deps.storage, &request)?;

    user_info.reward_amount = Uint128::zero();
    UST_USER_INFOS.save(deps.storage, wallet, &user_info)?;

    Ok(Response::new())
}

pub fn try_claimrewards_ust(
    deps: DepsMut,
    info: MessageInfo,
    request: Vec<PayRequest>,
)
    -> Result<Response, ContractError>
{
    if info.sender != TREASURY.load(deps.storage)? {
        return Err(ContractError::Unauthorized{ });
    }

    let mut claim_rewards_requests = UST_REWARDS_REQUEST.load(deps.storage)?;
    claim_rewards_requests = compare_remove(claim_rewards_requests, request)?;
    UST_REWARDS_REQUEST.save(deps.storage, &claim_rewards_requests)?;

    Ok(Response::new()
        .add_attribute("action", "claim rewards")
    )
}