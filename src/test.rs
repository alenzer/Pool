use super::*;
use cosmwasm_std::{from_binary, Addr, Coin, Timestamp,
    BankQuery, BalanceResponse, AllBalanceResponse, Uint128, Api};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg, UserInfo, AprInfo, PayRequest};
use crate::util::{get_multiplier};
use crate::mock_querier::{mock_dependencies};
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::pair::ExecuteMsg as TerraswapExecuteMsg;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
    let mut env = mock_env();
    let mut info = mock_info("owner", &[]);
    const MONTH: u64 = 2592000; //60 * 60 * 24 * 30;
    let mut seconds = 0;
    env.block.time = Timestamp::from_seconds(seconds.clone());

    let msg = InstantiateMsg{
        owner: Some("owner".to_string()),
        ust_apr: Uint128::from(2000u128),
        luna_apr: Uint128::from(1000u128),
        treasury: "treasury".to_string()
    };
//instantiate
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

//set config
    let msg = ExecuteMsg::SetConfig{owner: None, treasury: None};
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
    println!("Setconfig{:?}\n", res);

//deposit
    info.sender = Addr::unchecked("user1".to_string());
    info.funds = vec![Coin{denom: "uusd".to_string(), amount: Uint128::from(10u128)}];
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::DepositUST {  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("deposit{:?}\n", res);

//set apr
    info.sender = Addr::unchecked("owner".to_string());
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::SetAPRUST {apr: Uint128::from(4000u128) };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("set apr{:?}\n", res);
//-------------------
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());

//get apr
    let msg = QueryMsg::GetHistoryOfAPRUST{};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let apr_infos: Vec<AprInfo> = from_binary(&res).unwrap();
    println!("ust apr info: {:?}\n", apr_infos);
    
//get multiplier
    let multiplier = get_multiplier(apr_infos, Uint128::zero(), Uint128::from(seconds as u128)).unwrap();
    println!("multiplier {:?}", multiplier);
    println!("seconds {:?}", seconds);
    println!("block seconds {:?}", env.block.time.seconds());

//getrewards
    let msg = QueryMsg::GetPendingRewardsUST{wallet: Addr::unchecked("user1".to_string())};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: Uint128 = from_binary(&res).unwrap();
    println!("pending rewards {:?}\n", res );

//request ClaimRewards
    info.sender = Addr::unchecked("user1".to_string());
    let msg = ExecuteMsg::RequestClaimRewardsUST { };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request claim rewards{:?}\n", res);

//get claimrequest
    let msg = QueryMsg::GetClaimRewardsRequestUST { };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: Vec<PayRequest> = from_binary(&res).unwrap();
    println!("claim Request {:?}\n", res );

//withdraw 5 
    info.sender = Addr::unchecked("user1".to_string());
    info.funds = vec![];
    let msg = ExecuteMsg::RequestWithdrawUST { amount: Uint128::from(5u128)  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request withdraw5 {:?}\n", res);

//withdraw 4 
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::RequestWithdrawUST { amount: Uint128::from(1u128)  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request withdraw4 {:?}\n", res);

//getrewards
    let msg = QueryMsg::GetPendingRewardsUST{wallet: Addr::unchecked("user1".to_string())};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: Uint128 = from_binary(&res).unwrap();
    println!("pending rewards {:?}\n", res );

//withdraw 3 
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::RequestWithdrawUST { amount: Uint128::from(1u128)  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request withdraw3 {:?}\n", res);

//withdraw 2
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::RequestWithdrawUST { amount: Uint128::from(1u128)  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request withdraw2 {:?}\n", res);

//withdraw 1 
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::RequestWithdrawUST { amount: Uint128::from(1u128)  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request withdraw1 {:?}\n", res);
    
//withdraw 0 
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = ExecuteMsg::RequestWithdrawUST { amount: Uint128::from(1u128)  };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request withdraw0 {:?}\n", res);

//Withdraw
    info.sender = Addr::unchecked("treasury".to_string());
    let request: Vec<PayRequest> = vec![
        PayRequest{wallet: Addr::unchecked("user1".to_string()), amount: Uint128::from(5u128), time: Uint128::from(7776000u128)},
        PayRequest{wallet: Addr::unchecked("user1".to_string()), amount: Uint128::from(1u128), time: Uint128::from(20736000u128)},
        PayRequest{wallet: Addr::unchecked("user1".to_string()), amount: Uint128::from(1u128), time: Uint128::from(18144000u128)}
    ];

    let msg = ExecuteMsg::WithdrawUST {request};
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("withdraw{:?}\n", res);

//get withdrawrequest
    let msg = QueryMsg::GetWithdrawRequstUST{ };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: Vec<PayRequest> = from_binary(&res).unwrap();
    println!("withdraw Request {:?}\n", res );

//request ClaimRewards
    info.sender = Addr::unchecked("user1".to_string());
    let msg = ExecuteMsg::RequestClaimRewardsUST { };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("request claim rewards{:?}\n", res);

//get claimrequest
    let msg = QueryMsg::GetClaimRewardsRequestUST { };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: Vec<PayRequest> = from_binary(&res).unwrap();
    println!("claim Request {:?}\n", res );    

//ClaimRewards
    let request: Vec<PayRequest> = vec![
        PayRequest{wallet: Addr::unchecked("user1".to_string()), amount: Uint128::from(40u128), time: Uint128::from(7776000u128)},
        PayRequest{wallet: Addr::unchecked("user1".to_string()), amount: Uint128::from(1u128), time: Uint128::from(20736000u128)},
        PayRequest{wallet: Addr::unchecked("user1".to_string()), amount: Uint128::from(1u128), time: Uint128::from(18144000u128)}
    ];

    info.sender = Addr::unchecked("treasury".to_string());
    let msg = ExecuteMsg::ClaimRewardsUST {request };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    println!("claim rewards{:?}\n", res);

//get claimrequest
    let msg = QueryMsg::GetClaimRewardsRequestUST { };
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: Vec<PayRequest> = from_binary(&res).unwrap();
    println!("claim Request {:?}\n", res );    

//get user info
    seconds += MONTH;
    env.block.time = Timestamp::from_seconds(seconds.clone());
    let msg = QueryMsg::GetUserInfoUST{wallet: Addr::unchecked("user1".to_string())};
    let res = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: UserInfo = from_binary(&res).unwrap();
    println!("User info {:?}\n", res );
}

