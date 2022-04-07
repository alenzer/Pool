use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub treasury: String,
    pub ust_apr: Uint128,
    pub luna_apr: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetConfig {
        owner: Option<Addr>,
        treasury: Option<Addr>,
    },

    SetAprUst{
        apr: Uint128
    },

    DepositUst {
    },

    RequestWithdrawUst{
        amount: Uint128
    },

    WithdrawUst {
        request: Vec<PayRequest>,
    },

    RequestClaimRewardsUst {
    },

    ClaimRewardsUst{
        request: Vec<PayRequest>
    },
//------------------------------------------
    SetAprLuna{
        apr: Uint128
    },

    DepositLuna {
    },

    RequestWithdrawLuna{
        amount: Uint128
    },

    WithdrawLuna {
        request: Vec<PayRequest>,
    },

    RequestClaimRewardsLuna {
    },

    ClaimRewardsLuna{
        request: Vec<PayRequest>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner{ },
    GetTreasury{ },

    GetHistoryOfAprUst{ },
    GetUserInfoUst{ wallet: Addr },
    GetPendingRewardsUst{ wallet: Addr },
    GetWithdrawRequstUst{ },
    GetClaimRewardsRequestUst{ },
//-------------------------------
    GetHistoryOfAprLuna{ },
    GetUserInfoLuna{ wallet: Addr },
    GetPendingRewardsLuna{ wallet: Addr },
    GetWithdrawRequstLuna{ },
    GetClaimRewardsRequestLuna{ },

//--------------------------------
    GetAmountHistory{ },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AprInfo{
	pub apr: Uint128,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo{
	pub wallet: Addr,
	pub amount: Uint128,
	pub reward_amount: Uint128,
    pub last_reward_time: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PayRequest{
	pub wallet: Addr,
	pub amount: Uint128,
    pub time: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AmountInfo{
	pub ust_amount: Uint128,
    pub luna_amount: Uint128,
    pub time: u64,
}