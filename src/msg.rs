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

    SetAPRUST{
        apr: Uint128
    },

    DepositUST {
    },

    RequestWithdrawUST{
        amount: Uint128
    },

    WithdrawUST {
        request: Vec<PayRequest>,
    },

    RequestClaimRewardsUST {
    },

    ClaimRewardsUST{
        request: Vec<PayRequest>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner{ },
    GetTreasury{ },
    GetHistoryOfAPRUST{ },
    GetUserInfoUST{ wallet: Addr },
    GetPendingRewardsUST{ wallet: Addr },
    GetWithdrawRequstUST{ },
    GetClaimRewardsRequestUST{ }
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