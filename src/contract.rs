#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response,
    Uint128, CosmosMsg, BankMsg, Storage, Coin
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, UserInfo, AprInfo};
use crate::state::{OWNER, TREASURY, UST_APR_HISTORY, UST_USER_INFOS, 
    LUNA_APR_HISTORY, LUNA_USER_INFOS};
use crate::util::{check_onlyowner, update_userinfo};

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

        ExecuteMsg::SetUSTAPR{ apr }
            => try_set_ust_apr(deps, env, info, apr),

        ExecuteMsg::DepositUST {  }
            => try_deposit_ust(deps, env, info),
        
        ExecuteMsg::WithdrawUST { amount }
            => try_withdraw_ust(deps, env, info, amount),

        ExecuteMsg::ClaimRewardsUST { wallet }
            => try_claimrewards_ust(deps, env, info, wallet)
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
    };

    match treasury{
        Some(wallet) => {
            TREASURY.save(deps.storage, &wallet)?
        },
    };

    Ok(Response::new()
        .add_attribute("action", "SetConfig"))                                
}
pub fn try_set_ust_apr(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    apr: Uint128
)
    ->Result<Response, ContractError>
{
    let apr_history = UST_APR_HISTORY.load(deps.storage)?;
    let apr_info = AprInfo{
        apr,
        time: env.block.time.seconds(),
    };
    apr_history.push(apr_info);

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
    let fund = info.funds[0];
    if fund.denom != "uusd" {
        return Err(ContractError::PoolError{
            msg: "Invalid Fund Request".to_string()
        });
    }

    let mut res = UST_USER_INFOS.may_load(deps.storage, wallet.clone())?;
    let user_info = match res{
        Some(user_info) => {
            update_userinfo(deps.storage, env.clone(), wallet.clone())?;

            user_info.amount += fund.amount;
            user_info
        },
        None => UserInfo{
            wallet: wallet.clone(),
            amount: fund.amount,
            reward_amount: Uint128::zero(),
            last_reward_time: Uint128::from(env.block.time.seconds() as u128),
        }
    };

    UST_USER_INFOS.save(deps.storage, wallet, &user_info)?;

    let ust_release = Coin::new(release_amount, "uusd");
    let send2_creator = BankMsg::Send { 
        to_address: x.creator_wallet.to_string(),
        amount: vec![ust_release] 
    };

    let msg = WasmMsg::Execute { 
    contract_addr: token.to_string(), 
    msg: to_binary(
        &Cw20ExecuteMsg::Transfer { 
            recipient: wallet.to_string(), 
            amount: user_info.reward_amount
        }
    )?, 
    funds: vec![]
    };

    Ok(Response::new()
        .add_attribute("action", "desposit")
        .add_attribute("amount", fund.amount.to_string())
    )
}

pub fn try_withdraw_ust(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128
)
    -> Result<Response, ContractError>
{
    let wallet = info.sender;
    let mut user_info = UST_USER_INFOS.load(deps.storage, wallet.clone())?;
    if user_info.amount < amount {
        return Err(ContractError::NotEnoughBalance { balance: amount });
    }
    update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    user_info.amount -= amount;
    UST_USER_INFOS.save(deps.storage, wallet.clone(), &user_info)?;

    // let msg = WasmMsg::Execute { 
    //     contract_addr: token.to_string(), 
    //     msg: to_binary(
    //         &Cw20ExecuteMsg::Transfer { 
    //             recipient: wallet.to_string(), 
    //             amount: user_info.reward_amount
    //         }
    //     )?, 
    //     funds: vec![]
    // };
    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_message(msg)
    )
}

pub fn try_claimrewards_ust(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wallet: Addr,
)
    -> Result<Response, ContractError>
{
    // update_userinfo(deps.storage, env.clone(), wallet.clone())?;

    // let mut user_info = USER_INFOS.load(deps.storage, wallet.clone())?;
    // let token = REWARD_TOKEN.load(deps.storage)?;
    // let balance = get_token_balance(&deps.querier, token.clone(), env.contract.address)?;

    // if balance < user_info.reward_amount {
    //     return Err(ContractError::NotEnoughBalance { balance });
    // }

    // let msg = WasmMsg::Execute { 
    //     contract_addr: token.to_string(), 
    //     msg: to_binary(
    //         &Cw20ExecuteMsg::Transfer { 
    //             recipient: wallet.to_string(), 
    //             amount: user_info.reward_amount
    //         }
    //     )?, 
    //     funds: vec![]
    // };
    // user_info.reward_amount = Uint128::zero();

    // USER_INFOS.save(deps.storage, wallet, &user_info)?;
    Ok(Response::new()
        .add_attribute("action", "claim rewards")
    )
}